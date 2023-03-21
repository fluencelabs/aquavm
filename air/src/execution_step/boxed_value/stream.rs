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
use crate::ExecutionError;
use crate::JValue;
use crate::UncatchableError;

use air_trace_handler::merger::ValueSource;
use air_trace_handler::TraceHandler;

/// Streams are CRDT-like append only data structures. They are guaranteed to have the same order
/// of values on each peer.
#[derive(Debug, Default, Clone)]
pub struct Stream {
    /// The first Vec represents generations, the second values in a generation. Generation is a set
    /// of values that interpreter obtained from one particle. It means that number of generation on
    /// a peer is equal to number of the interpreter runs in context of one particle. And each set of
    /// obtained values from a current_data that were not present in prev_data becomes a new generation.
    values: Vec<Vec<ValueAggregate>>,

    /// Count of values from previous data.
    previous_gens_count: usize,
}

impl Stream {
    pub(crate) fn from_generations_count(previous_count: usize, current_count: usize) -> Self {
        let last_generation_count = 1;
        // TODO: bubble up an overflow error instead of expect
        let overall_count = previous_count
            .checked_add(current_count)
            .and_then(|value| value.checked_add(last_generation_count))
            .expect("it shouldn't overflow");

        Self {
            values: vec![vec![]; overall_count],
            previous_gens_count: previous_count,
        }
    }

    // streams created with this ctor assumed to have only one generation,
    // for streams that have values in
    pub(crate) fn from_value(value: ValueAggregate) -> Self {
        Self {
            values: vec![vec![value]],
            previous_gens_count: 0,
        }
    }

    // if generation is None, value would be added to the last generation, otherwise it would
    // be added to given generation
    pub(crate) fn add_value(
        &mut self,
        value: ValueAggregate,
        generation: Generation,
        source: ValueSource,
    ) -> ExecutionResult<u32> {
        let generation_number = match (generation, source) {
            (Generation::Last, _) => self.values.len() - 1,
            (Generation::Nth(previous_gen), ValueSource::PreviousData) => previous_gen as usize,
            (Generation::Nth(current_gen), ValueSource::CurrentData) => self.previous_gens_count + current_gen as usize,
        };

        if generation_number >= self.values.len() {
            return Err(UncatchableError::StreamDontHaveSuchGeneration {
                stream: self.clone(),
                generation,
            }
            .into());
        }

        self.values[generation_number].push(value);
        Ok(generation_number as u32)
    }

    pub(crate) fn generations_count(&self) -> usize {
        // the last generation could be empty due to the logic of from_generations_count ctor
        if self.values.last().unwrap().is_empty() {
            self.values.len() - 1
        } else {
            self.values.len()
        }
    }

    pub(crate) fn last_non_empty_generation(&self) -> usize {
        self.values
            .iter()
            .rposition(|generation| !generation.is_empty())
            // it's safe to add + 1 here, because this function is called when
            // there is a new state was added with add_new_generation_if_non_empty
            .map(|non_empty_gens| non_empty_gens + 1)
            .unwrap_or_else(|| self.generations_count())
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

    /// Removes empty generations updating data and returns final generation count.
    pub(crate) fn compactify(mut self, trace_ctx: &mut TraceHandler) -> ExecutionResult<usize> {
        self.remove_empty_generations();

        for (generation, values) in self.values.iter().enumerate() {
            for value in values.iter() {
                trace_ctx
                    .update_generation(value.trace_pos, generation as u32)
                    .map_err(|e| ExecutionError::Uncatchable(UncatchableError::GenerationCompatificationError(e)))?;
            }
        }

        Ok(self.values.len())
    }

    /// Removes empty generations from current values.
    fn remove_empty_generations(&mut self) {
        self.values.retain(|values| !values.is_empty());
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Generation {
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

use std::fmt;

impl fmt::Display for Stream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.values.is_empty() {
            return write!(f, "[]");
        }

        writeln!(f, "[")?;
        for (id, generation) in self.values.iter().enumerate() {
            write!(f, " -- {id}: ")?;
            for value in generation.iter() {
                write!(f, "{value:?}, ")?;
            }
            writeln!(f)?;
        }

        write!(f, "]")
    }
}

impl fmt::Display for Generation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Generation::Nth(generation) => write!(f, "{}", generation),
            Generation::Last => write!(f, "Last"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Generation;
    use super::Stream;
    use super::ValueAggregate;

    use air_interpreter_data::PosType;
    use serde_json::json;

    use air_trace_handler::merger::ValueSource;
    use std::rc::Rc;

    #[test]
    fn test_slice_iter() {
        let pos_one: PosType = 1;
        let value_1 = ValueAggregate::new(Rc::new(json!("value")), <_>::default(), pos_one.into());
        let value_2 = ValueAggregate::new(Rc::new(json!("value")), <_>::default(), pos_one.into());
        let mut stream = Stream::from_generations_count(2, 0);

        stream
            .add_value(value_1, Generation::Nth(0), ValueSource::PreviousData)
            .unwrap();
        stream
            .add_value(value_2, Generation::Nth(1), ValueSource::PreviousData)
            .unwrap();

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
        let stream = Stream::from_generations_count(2, 0);

        let slice = stream.slice_iter(Generation::Nth(0), Generation::Nth(1));
        assert!(slice.is_none());

        let slice = stream.slice_iter(Generation::Nth(0), Generation::Last);
        assert!(slice.is_none());

        let slice = stream.slice_iter(Generation::Nth(0), Generation::Nth(0));
        assert!(slice.is_none());

        let slice = stream.slice_iter(Generation::Last, Generation::Last);
        assert!(slice.is_none());
    }

    #[test]
    fn generation_from_current_data() {
        let pos_one: PosType = 1;
        let pos_two: PosType = 2;
        let value_1 = ValueAggregate::new(Rc::new(json!("value_1")), <_>::default(), pos_one.into());
        let value_2 = ValueAggregate::new(Rc::new(json!("value_2")), <_>::default(), pos_two.into());
        let mut stream = Stream::from_generations_count(5, 5);

        stream
            .add_value(value_1.clone(), Generation::Nth(2), ValueSource::CurrentData)
            .unwrap();
        stream
            .add_value(value_2.clone(), Generation::Nth(4), ValueSource::PreviousData)
            .unwrap();

        let generations_count = stream.generations_count();
        assert_eq!(generations_count, 10);

        let mut iter = stream.iter(Generation::Last).unwrap();
        let stream_value_1 = iter.next().unwrap();
        let stream_value_2 = iter.next().unwrap();

        assert_eq!(stream_value_1, &value_2);
        assert_eq!(stream_value_2, &value_1);
    }
}
