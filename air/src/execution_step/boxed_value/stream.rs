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
use super::WithProvenance;
use crate::ExecutionError;
use crate::UncatchableError;

use air_interpreter_data::GenerationIdx;
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
    values: Vec<Vec<WithProvenance<ValueAggregate>>>,

    /// Count of values from previous data.
    previous_gens_count: usize,
}

impl Stream {
    pub(crate) fn from_generations_count(previous_count: GenerationIdx, current_count: GenerationIdx) -> Self {
        let last_generation_count = GenerationIdx::from(1);
        // TODO: bubble up an overflow error instead of expect
        let overall_count = previous_count
            .checked_add(current_count)
            .and_then(|value| value.checked_add(last_generation_count))
            .expect("it shouldn't overflow");
        Self {
            values: vec![vec![]; overall_count.into()],
            previous_gens_count: previous_count.into(),
        }
    }

    // streams created with this ctor assumed to have only one generation,
    // for streams that have values in
    pub(crate) fn from_value(value: WithProvenance<ValueAggregate>) -> Self {
        Self {
            values: vec![vec![value]],
            previous_gens_count: 0,
        }
    }

    // if generation is None, value would be added to the last generation, otherwise it would
    // be added to given generation
    pub(crate) fn add_value(
        &mut self,
        value: WithProvenance<ValueAggregate>,
        generation: Generation,
        source: ValueSource,
    ) -> ExecutionResult<GenerationIdx> {
        let generation_number = match (generation, source) {
            (Generation::Last, _) => self.values.len() - 1,
            (Generation::Nth(previous_gen), ValueSource::PreviousData) => previous_gen.into(),
            (Generation::Nth(current_gen), ValueSource::CurrentData) => {
                self.previous_gens_count + usize::from(current_gen)
            }
        };

        if generation_number >= self.values.len() {
            return Err(UncatchableError::StreamDontHaveSuchGeneration {
                stream: self.clone(),
                generation,
            }
            .into());
        }

        self.values[generation_number].push(value);
        Ok(generation_number.into())
    }

    // TODO: remove this function
    pub(crate) fn generations_count(&self) -> usize {
        // the last generation could be empty due to the logic of from_generations_count ctor
        if self.values.last().unwrap().is_empty() {
            self.values.len() - 1
        } else {
            self.values.len()
        }
    }

    pub(crate) fn last_non_empty_generation(&self) -> GenerationIdx {
        self.values
            .iter()
            .rposition(|generation| !generation.is_empty())
            // it's safe to add + 1 here, because this function is called when
            // there is a new state was added with add_new_generation_if_non_empty
            .map(|non_empty_gens| non_empty_gens + 1)
            .unwrap_or_else(|| self.generations_count())
            .into()
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

    pub(crate) fn generation_elements_count(&self, generation: Generation) -> Option<usize> {
        match generation {
            Generation::Nth(generation) if generation > self.generations_count() => None,
            Generation::Nth(generation) => {
                let elements_count = generation.into();
                Some(self.values.iter().take(elements_count).map(|v| v.len()).sum())
            }
            Generation::Last => Some(self.values.iter().map(|v| v.len()).sum()),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.values.iter().all(|v| v.is_empty())
    }

    pub(crate) fn iter(&self, generation: Generation) -> Option<StreamIter<'_>> {
        let iter: Box<dyn Iterator<Item = &WithProvenance<ValueAggregate>>> = match generation {
            Generation::Nth(generation) if generation >= self.generations_count() => return None,
            Generation::Nth(generation) => {
                Box::new(self.values.iter().take(generation.next().into()).flat_map(|v| v.iter()))
            }
            Generation::Last => Box::new(self.values.iter().flat_map(|v| v.iter())),
        };
        // unwrap is safe here, because generation's been already checked
        let len = self.generation_elements_count(generation).unwrap();

        let iter = StreamIter { iter, len };

        Some(iter)
    }

    pub(crate) fn slice_iter(&self, start: Generation, end: Generation) -> Option<StreamSliceIter<'_>> {
        if self.is_empty() {
            return None;
        }

        let generations_count = self.generations_count() - 1;
        let (start, end) = match (start, end) {
            (Generation::Nth(start), Generation::Nth(end)) => (usize::from(start), usize::from(end)),
            (Generation::Nth(start), Generation::Last) => (start.into(), generations_count),
            (Generation::Last, Generation::Nth(end)) => (generations_count, end.into()),
            (Generation::Last, Generation::Last) => (generations_count, generations_count),
        };

        if start > end || end > generations_count {
            return None;
        }

        let len = (end - start) + 1;
        let iter: Box<dyn Iterator<Item = &[WithProvenance<ValueAggregate>]>> =
            Box::new(self.values.iter().skip(start).take(len).map(|v| v.as_slice()));
        let iter = StreamSliceIter { iter, len };

        Some(iter)
    }

    /// Removes empty generations updating data and returns final generation count.
    pub(crate) fn compactify(mut self, trace_ctx: &mut TraceHandler) -> ExecutionResult<GenerationIdx> {
        self.remove_empty_generations();

        for (generation, values) in self.values.iter().enumerate() {
            for value in values.iter() {
                trace_ctx
                    .update_generation(value.get_trace_pos(), generation.into())
                    .map_err(|e| ExecutionError::Uncatchable(UncatchableError::GenerationCompatificationError(e)))?;
            }
        }
        let last_generation_idx = self.values.len();
        Ok(last_generation_idx.into())
    }

    /// Removes empty generations from current values.
    fn remove_empty_generations(&mut self) {
        self.values.retain(|values| !values.is_empty());
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Generation {
    Last,
    Nth(GenerationIdx),
}

impl Generation {
    pub fn last() -> Self {
        Self::Last
    }

    #[cfg(test)]
    pub fn nth(generation_id: u32) -> Self {
        use std::convert::TryFrom;

        let generation_id = usize::try_from(generation_id).unwrap();
        let generation_idx = GenerationIdx::from(generation_id);
        Self::Nth(generation_idx)
    }
}

pub(crate) struct StreamIter<'result> {
    iter: Box<dyn Iterator<Item = &'result WithProvenance<ValueAggregate>> + 'result>,
    len: usize,
}

impl<'result> Iterator for StreamIter<'result> {
    type Item = &'result WithProvenance<ValueAggregate>;

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
    iter: Box<dyn Iterator<Item = &'slice [WithProvenance<ValueAggregate>]> + 'slice>,
    pub len: usize,
}

impl<'slice> Iterator for StreamSliceIter<'slice> {
    type Item = &'slice [WithProvenance<ValueAggregate>];

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
    use crate::execution_step::ServiceResultAggregate;

    use super::Generation;
    use super::Stream;
    use super::ValueAggregate;
    use super::ValueSource;
    use super::WithProvenance;

    use air_interpreter_cid::CID;
    use air_interpreter_data::Provenance;
    use serde_json::json;

    use std::rc::Rc;

    #[test]
    fn test_slice_iter() {
        let value_1 = WithProvenance::new(
            ValueAggregate::from_service_result(
                ServiceResultAggregate::new(Rc::new(json!("value")), <_>::default(), 1.into()),
                CID::new("some fake cid").into(),
            ),
            Provenance::service_result(CID::new("some fake cid").into()),
        );
        let value_2 = WithProvenance::new(
            ValueAggregate::from_service_result(
                ServiceResultAggregate::new(Rc::new(json!("value")), <_>::default(), 1.into()),
                CID::new("some fake cid").into(),
            ),
            Provenance::service_result(CID::new("some fake cid").into()),
        );
        let mut stream = Stream::from_generations_count(2.into(), 0.into());

        stream
            .add_value(value_1, Generation::nth(0), ValueSource::PreviousData)
            .unwrap();
        stream
            .add_value(value_2, Generation::nth(1), ValueSource::PreviousData)
            .unwrap();

        let slice = stream.slice_iter(Generation::nth(0), Generation::nth(1)).unwrap();
        assert_eq!(slice.len, 2);

        let slice = stream.slice_iter(Generation::nth(0), Generation::Last).unwrap();
        assert_eq!(slice.len, 2);

        let slice = stream.slice_iter(Generation::nth(0), Generation::nth(0)).unwrap();
        assert_eq!(slice.len, 1);

        let slice = stream.slice_iter(Generation::Last, Generation::Last).unwrap();
        assert_eq!(slice.len, 1);
    }

    #[test]
    fn test_slice_on_empty_stream() {
        let stream = Stream::from_generations_count(2.into(), 0.into());

        let slice = stream.slice_iter(Generation::nth(0), Generation::nth(1));
        assert!(slice.is_none());

        let slice = stream.slice_iter(Generation::nth(0), Generation::Last);
        assert!(slice.is_none());

        let slice = stream.slice_iter(Generation::nth(0), Generation::nth(0));
        assert!(slice.is_none());

        let slice = stream.slice_iter(Generation::Last, Generation::Last);
        assert!(slice.is_none());
    }

    #[test]
    fn generation_from_current_data() {
        let value_1 = WithProvenance::new(
            ValueAggregate::from_service_result(
                ServiceResultAggregate::new(Rc::new(json!("value_1")), <_>::default(), 1.into()),
                CID::new("some fake cid").into(),
            ),
            Provenance::service_result(CID::new("some fake cid").into()),
        );
        let value_2 = WithProvenance::new(
            ValueAggregate::from_service_result(
                ServiceResultAggregate::new(Rc::new(json!("value_2")), <_>::default(), 2.into()),
                CID::new("some fake cid").into(),
            ),
            Provenance::service_result(CID::new("some fake cid").into()),
        );
        let mut stream = Stream::from_generations_count(5.into(), 5.into());

        stream
            .add_value(value_1.clone(), Generation::nth(2), ValueSource::CurrentData)
            .unwrap();
        stream
            .add_value(value_2.clone(), Generation::nth(4), ValueSource::PreviousData)
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
