/*
 * Copyright 2023 Fluence Labs Limited
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

use super::values_matrix::NewValuesMatrix;
use super::values_matrix::ValuesMatrix;
use crate::execution_step::ExecutionResult;

use air_interpreter_data::GenerationIdx;
use air_trace_handler::TraceHandler;

/// Streams are CRDT-like append only data structures. They are guaranteed to have locally
/// the same order of values on each peer.
#[derive(Debug, Default, Clone)]
pub(crate) struct Stream<T> {
    /// Values from previous data.
    previous_values: ValuesMatrix<T>,

    /// Values from current data.
    current_values: ValuesMatrix<T>,

    /// Values from call results executed on a current peer.
    new_values: NewValuesMatrix<T>,
}

impl<T> Stream<T> {
    pub(crate) fn new() -> Self {
        Self {
            previous_values: ValuesMatrix::new(),
            current_values: ValuesMatrix::new(),
            new_values: NewValuesMatrix::new(),
        }
    }

    pub(crate) fn from_new_value(value: T) -> Self {
        Self {
            previous_values: ValuesMatrix::new(),
            current_values: ValuesMatrix::new(),
            new_values: NewValuesMatrix::from_value(value),
        }
    }

    pub(crate) fn add_value(&mut self, value: T, generation: Generation) {
        match generation {
            Generation::Previous(previous_gen) => self.previous_values.add_value_to_generation(value, previous_gen),
            Generation::Current(current_gen) => self.current_values.add_value_to_generation(value, current_gen),
            Generation::New => self.new_values.add_to_last_generation(value),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        let is_prev_empty = self.previous_values.iter().all(|v| v.is_empty());
        let is_curr_empty = self.current_values.iter().all(|v| v.is_empty());
        let is_new_empty = self.new_values.iter().all(|v| v.is_empty());

        is_prev_empty && is_curr_empty && is_new_empty
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &T> {
        self.previous_values
            .iter()
            .chain(self.current_values.iter())
            .chain(self.new_values.iter())
    }

    // Contract: all slices will be non-empty
    pub(crate) fn slice_iter(&self) -> impl Iterator<Item = &[T]> {
        self.previous_values
            .slice_iter()
            .chain(self.current_values.slice_iter())
            .chain(self.new_values.slice_iter())
    }

    /// Removes empty generations updating data.
    pub(crate) fn compactify(mut self, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        self.previous_values.remove_empty_generations();
        self.current_values.remove_empty_generations();
        self.new_values.remove_empty_generations();

        let start_idx = 0.into();
        Self::update_generations(&self.previous_values, start_idx, trace_ctx)?;

        let start_idx = self.previous_values.len();
        Self::update_generations(&self.current_values, start_idx, trace_ctx)?;

        let start_idx = start_idx.checked_add(self.current_values.len()).unwrap();
        Self::update_generations(&self.new_values, start_idx, trace_ctx)?;

        Ok(())
    }

    pub(super) fn new_values(&mut self) -> &mut NewValuesMatrix<T> {
        &mut self.new_values
    }

    fn update_generations(
        values: impl Iterator<Item = &T>,
        start_idx: GenerationIdx,
        trace_ctx: &mut TraceHandler,
    ) -> ExecutionResult<()> {
        use crate::execution_step::errors::UncatchableError;
        use crate::execution_step::ExecutionError;

        for (position, values) in values.enumerate() {
            let generation = start_idx.checked_add(position.into()).unwrap();
            for value in values.iter() {
                trace_ctx
                    .update_generation(value.get_trace_pos(), generation)
                    .map_err(|e| ExecutionError::Uncatchable(UncatchableError::GenerationCompatificationError(e)))?;
            }
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Generation {
    Previous(GenerationIdx),
    Current(GenerationIdx),
    New,
}

impl Generation {
    #[cfg(test)]
    pub fn previous(generation_id: u32) -> Self {
        use std::convert::TryFrom;

        let generation_id = usize::try_from(generation_id).unwrap();
        let generation_idx = GenerationIdx::from(generation_id);
        Self::Previous(generation_idx)
    }

    #[cfg(test)]
    pub fn current(generation_id: u32) -> Self {
        use std::convert::TryFrom;

        let generation_id = usize::try_from(generation_id).unwrap();
        let generation_idx = GenerationIdx::from(generation_id);
        Self::Current(generation_idx)
    }

    pub fn new() -> Self {
        Self::New
    }
}

use std::fmt;

impl<T: fmt::Display> fmt::Display for Stream<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "previous values:\n{}", self.previous_values)?;
        writeln!(f, "current values:\n{}", self.current_values)?;
        writeln!(f, "new values:\n{}", self.new_values)
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
    use super::ValueSource;
    use crate::execution_step::ServiceResultAggregate;

    use air_interpreter_cid::CID;
    use serde_json::json;

    use std::rc::Rc;

    #[test]
    fn test_slice_iter() {
        let value_1 = ValueAggregate::from_service_result(
            ServiceResultAggregate::new(Rc::new(json!("value")), <_>::default(), 1.into()),
            CID::new("some fake cid").into(),
        );
        let value_2 = ValueAggregate::from_service_result(
            ServiceResultAggregate::new(Rc::new(json!("value")), <_>::default(), 1.into()),
            CID::new("some fake cid").into(),
        );
        let mut stream = Stream::from_generations_count(2.into(), 0.into());

        stream
            .add_value(value_1, Generation::previous(0), ValueSource::PreviousData)
            .unwrap();
        stream
            .add_value(value_2, Generation::previous(1), ValueSource::PreviousData)
            .unwrap();

        let slice = stream
            .slice_iter(Generation::previous(0), Generation::previous(1))
            .unwrap();
        assert_eq!(slice.len, 2);

        let slice = stream.slice_iter(Generation::previous(0), Generation::Last).unwrap();
        assert_eq!(slice.len, 2);

        let slice = stream
            .slice_iter(Generation::previous(0), Generation::previous(0))
            .unwrap();
        assert_eq!(slice.len, 1);

        let slice = stream.slice_iter(Generation::Last, Generation::Last).unwrap();
        assert_eq!(slice.len, 1);
    }

    #[test]
    fn test_slice_on_empty_stream() {
        let stream = Stream::from_generations_count(2.into(), 0.into());

        let slice = stream.slice_iter(Generation::previous(0), Generation::previous(1));
        assert!(slice.is_none());

        let slice = stream.slice_iter(Generation::previous(0), Generation::Last);
        assert!(slice.is_none());

        let slice = stream.slice_iter(Generation::previous(0), Generation::previous(0));
        assert!(slice.is_none());

        let slice = stream.slice_iter(Generation::Last, Generation::Last);
        assert!(slice.is_none());
    }

    #[test]
    fn generation_from_current_data() {
        let value_1 = ValueAggregate::from_service_result(
            ServiceResultAggregate::new(Rc::new(json!("value_1")), <_>::default(), 1.into()),
            CID::new("some fake cid").into(),
        );
        let value_2 = ValueAggregate::from_service_result(
            ServiceResultAggregate::new(Rc::new(json!("value_2")), <_>::default(), 2.into()),
            CID::new("some fake cid").into(),
        );
        let mut stream = Stream::from_generations_count(5.into(), 5.into());

        stream
            .add_value(value_1.clone(), Generation::previous(2), ValueSource::CurrentData)
            .unwrap();
        stream
            .add_value(value_2.clone(), Generation::previous(4), ValueSource::PreviousData)
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
