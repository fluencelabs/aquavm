/*
 * Copyright 2021 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use super::ExecutionResult;
use super::ValueAggregate;
use crate::execution_step::CatchableError;
use crate::JValue;

use air_interpreter_data::TracePos;

use std::collections::HashMap;
use std::fmt::Formatter;

/// Streams are CRDT-like append only data structures. They are guaranteed to have the same order
/// of values on each peer.
#[derive(Debug, Default, Clone)]
pub struct Stream {
    /// The first Vec represents generations, the second values in a generation. Generation is a set
    /// of values that interpreter obtained from one particle. It means that number of generation on
    /// a peer is equal to number of the interpreter runs in context of one particle. And each set of
    /// obtained values from a current_data that were not present in prev_data becomes a new generation.
    values: Vec<Vec<ValueAggregate>>,

    /// This map is intended to support canonicalized stream creation, such streams has
    /// corresponding value positions in a data and this field are used to create such streams.
    values_by_pos: HashMap<TracePos, StreamValueLocation>,
}

impl Stream {
    pub(crate) fn from_generations_count(count: usize) -> Self {
        Self {
            values: vec![vec![]; count + 1],
            values_by_pos: HashMap::new(),
        }
    }

    pub(crate) fn from_value(value: ValueAggregate) -> Self {
        let values_by_pos = maplit::hashmap! {
            value.trace_pos => StreamValueLocation::new(0, 0),
        };
        Self {
            values: vec![vec![value]],
            values_by_pos,
        }
    }

    // if generation is None, value would be added to the last generation, otherwise it would
    // be added to given generation
    pub(crate) fn add_value(&mut self, value: ValueAggregate, generation: Generation) -> ExecutionResult<u32> {
        let generation = match generation {
            Generation::Last => self.values.len() - 1,
            Generation::Nth(id) => id as usize,
        };

        if generation >= self.values.len() {
            return Err(CatchableError::StreamDontHaveSuchGeneration(self.clone(), generation).into());
        }

        let values = &mut self.values[generation];
        self.values_by_pos
            .insert(value.trace_pos, StreamValueLocation::new(generation, values.len()));
        values.push(value);
        Ok(generation as u32)
    }

    pub(crate) fn generations_count(&self) -> usize {
        // the last generation could be empty due to the logic of from_generations_count ctor
        self.values.iter().filter(|gen| !gen.is_empty()).count()
    }

    /// Add a new empty generation if the latest isn't empty.
    pub(crate) fn add_new_generation_if_non_empty(&mut self) -> bool {
        let should_add_generation = match self.values.last() {
            Some(last) => !last.is_empty(),
            None => true,
        };

        if should_add_generation {
            self.values.push(vec![]);
        }
        should_add_generation
    }

    /// Remove a last generation if it's empty.
    pub(crate) fn remove_last_generation_if_empty(&mut self) -> bool {
        let should_remove_generation = match self.values.last() {
            Some(last) => last.is_empty(),
            None => false,
        };

        if should_remove_generation {
            self.values.pop();
        }

        should_remove_generation
    }

    pub(crate) fn elements_count(&self, generation: Generation) -> Option<usize> {
        match generation {
            Generation::Nth(generation) if generation as usize > self.generations_count() => None,
            Generation::Nth(generation) => Some(self.values.iter().take(generation as usize).map(|v| v.len()).sum()),
            Generation::Last => Some(self.values.iter().map(|v| v.len()).sum()),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        if self.values.is_empty() {
            return false;
        }

        self.values.iter().all(|v| v.is_empty())
    }

    pub(crate) fn as_jvalue(&self, generation: Generation) -> Option<JValue> {
        use std::ops::Deref;

        let iter = self.iter(generation)?;
        let jvalue_array = iter.map(|r| r.result.deref().clone()).collect::<Vec<_>>();

        Some(JValue::Array(jvalue_array))
    }

    pub(crate) fn get_value_by_pos(&self, position: TracePos) -> Option<&ValueAggregate> {
        let StreamValueLocation {
            generation,
            position_in_generation,
        } = self.values_by_pos.get(&position)?;
        let value = &self.values[*generation][*position_in_generation];
        Some(value)
    }

    pub(crate) fn iter(&self, generation: Generation) -> Option<StreamIter<'_>> {
        let iter: Box<dyn Iterator<Item = &ValueAggregate>> = match generation {
            Generation::Nth(generation) if generation as usize >= self.generations_count() => return None,
            Generation::Nth(generation) => {
                Box::new(self.values.iter().take(generation as usize + 1).flat_map(|v| v.iter()))
            }
            Generation::Last => Box::new(self.values.iter().flat_map(|v| v.iter())),
        };
        // unwrap is safe here, because generation's been already checked
        let len = self.elements_count(generation).unwrap();

        let iter = StreamIter { iter, len };

        Some(iter)
    }

    pub(crate) fn slice_iter(&self, start: Generation, end: Generation) -> Option<StreamSliceIter<'_>> {
        if self.is_empty() {
            return None;
        }

        let generations_count = self.generations_count() as u32 - 1;
        let (start, end) = match (start, end) {
            (Generation::Nth(start), Generation::Nth(end)) => (start, end),
            (Generation::Nth(start), Generation::Last) => (start, generations_count),
            (Generation::Last, Generation::Nth(end)) => (generations_count, end),
            (Generation::Last, Generation::Last) => (generations_count, generations_count),
        };

        if start > end || end > generations_count {
            return None;
        }

        let len = (end - start) as usize + 1;
        let iter: Box<dyn Iterator<Item = &[ValueAggregate]>> =
            Box::new(self.values.iter().skip(start as usize).take(len).map(|v| v.as_slice()));
        let iter = StreamSliceIter { iter, len };

        Some(iter)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Generation {
    Last,
    Nth(u32),
}

pub(crate) struct StreamIter<'result> {
    iter: Box<dyn Iterator<Item = &'result ValueAggregate> + 'result>,
    len: usize,
}

impl<'result> Iterator for StreamIter<'result> {
    type Item = &'result ValueAggregate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.len -= 1;
        }
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'result> ExactSizeIterator for StreamIter<'result> {}

pub(crate) struct StreamSliceIter<'slice> {
    iter: Box<dyn Iterator<Item = &'slice [ValueAggregate]> + 'slice>,
    pub len: usize,
}

impl<'slice> Iterator for StreamSliceIter<'slice> {
    type Item = &'slice [ValueAggregate];

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.len -= 1;
        }
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct StreamValueLocation {
    pub generation: usize,
    pub position_in_generation: usize,
}

impl StreamValueLocation {
    pub(super) fn new(generation: usize, position_in_generation: usize) -> Self {
        Self {
            generation,
            position_in_generation,
        }
    }
}

use std::fmt;

impl fmt::Display for Stream {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.values.is_empty() {
            return write!(f, "[]");
        }

        writeln!(f, "[")?;
        for (id, generation) in self.values.iter().enumerate() {
            write!(f, " -- {}: ", id)?;
            for value in generation.iter() {
                write!(f, "{:?}, ", value)?;
            }
            writeln!(f)?;
        }

        write!(f, "]")
    }
}

#[cfg(test)]
mod test {
    use super::Generation;
    use super::Stream;
    use super::ValueAggregate;

    use serde_json::json;

    use std::rc::Rc;

    #[test]
    fn test_slice_iter() {
        let value_1 = ValueAggregate::new(Rc::new(json!("value")), <_>::default(), 1.into());
        let value_2 = ValueAggregate::new(Rc::new(json!("value")), <_>::default(), 1.into());
        let mut stream = Stream::from_generations_count(2);

        stream.add_value(value_1, Generation::Nth(0)).unwrap();
        stream.add_value(value_2, Generation::Nth(1)).unwrap();

        let slice = stream.slice_iter(Generation::Nth(0), Generation::Nth(1)).unwrap();
        assert_eq!(slice.len, 2);

        let slice = stream.slice_iter(Generation::Nth(0), Generation::Last).unwrap();
        assert_eq!(slice.len, 2);

        let slice = stream.slice_iter(Generation::Nth(0), Generation::Nth(0)).unwrap();
        assert_eq!(slice.len, 1);

        let slice = stream.slice_iter(Generation::Last, Generation::Last).unwrap();
        assert_eq!(slice.len, 1);
    }

    #[test]
    fn test_slice_on_empty_stream() {
        let stream = Stream::from_generations_count(2);

        let slice = stream.slice_iter(Generation::Nth(0), Generation::Nth(1));
        assert!(slice.is_none());

        let slice = stream.slice_iter(Generation::Nth(0), Generation::Last);
        assert!(slice.is_none());

        let slice = stream.slice_iter(Generation::Nth(0), Generation::Nth(0));
        assert!(slice.is_none());

        let slice = stream.slice_iter(Generation::Last, Generation::Last);
        assert!(slice.is_none());
    }
}
