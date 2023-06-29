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
use crate::execution_step::boxed_value::TracePosOperate;
use crate::execution_step::ExecutionResult;

use air_interpreter_data::GenerationIdx;
use air_trace_handler::TraceHandler;

/// Streams are CRDT-like append only data structures. They are guaranteed to have locally
/// the same order of values on each peer.
#[derive(Debug, Default, Clone)]
pub struct Stream<T> {
    /// Values from previous data.
    previous_values: ValuesMatrix<T>,

    /// Values from current data.
    current_values: ValuesMatrix<T>,

    /// Values from call results executed on a current peer.
    new_values: NewValuesMatrix<T>,
}

impl<'value, T: 'value> Stream<T> {
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

    pub(super) fn new_values(&mut self) -> &mut NewValuesMatrix<T> {
        &mut self.new_values
    }
}

impl<'value, T: 'value + Clone> Stream<T> {
    pub(crate) fn add_value(&mut self, value: T, generation: Generation) {
        match generation {
            Generation::Previous(previous_gen) => self.previous_values.add_value_to_generation(value, previous_gen),
            Generation::Current(current_gen) => self.current_values.add_value_to_generation(value, current_gen),
            Generation::New => self.new_values.add_to_last_generation(value),
        }
    }
}

impl<'value, T: 'value + TracePosOperate> Stream<T> {
    /// Removes empty generations updating data.
    pub(crate) fn compactify(mut self, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        self.previous_values.remove_empty_generations();
        self.current_values.remove_empty_generations();
        self.new_values.remove_empty_generations();

        let start_idx = 0.into();
        Self::update_generations(self.previous_values.slice_iter(), start_idx, trace_ctx)?;

        let start_idx = self.previous_values.len();
        Self::update_generations(self.current_values.slice_iter(), start_idx, trace_ctx)?;

        let start_idx = start_idx.checked_add(self.current_values.len()).unwrap();
        Self::update_generations(self.new_values.slice_iter(), start_idx, trace_ctx)?;

        Ok(())
    }

    fn update_generations(
        values: impl Iterator<Item = &'value [T]>,
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

use air_trace_handler::merger::MetApResult;
use air_trace_handler::merger::ValueSource;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Generation {
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

    pub fn from_met_result(result: &MetApResult) -> Self {
        Self::from_data(result.value_source, result.generation)
    }

    pub fn from_data(data_type: ValueSource, generation: GenerationIdx) -> Self {
        match data_type {
            ValueSource::PreviousData => Generation::Previous(generation),
            ValueSource::CurrentData => Generation::Current(generation),
        }
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
            Generation::Previous(generation) => write!(f, "previous({})", generation),
            Generation::Current(generation) => write!(f, "current({})", generation),
            Generation::New => write!(f, "new"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Generation;
    use super::TraceHandler;
    use crate::ExecutionError;
    use crate::UncatchableError;
    use crate::execution_step::ServiceResultAggregate;
    use crate::execution_step::ValueAggregate;
    use crate::JValue;

    use air_interpreter_cid::CID;
    use air_interpreter_data::ApResult;
    use air_interpreter_data::CanonResult;
    use air_interpreter_data::TracePos;
    use air_interpreter_data::ExecutedState;
    use air_interpreter_data::ExecutionTrace;
    use air_trace_handler::GenerationCompatificationError;
    use serde_json::json;

    use std::rc::Rc;

    type Stream = super::Stream<ValueAggregate>;

    fn create_value(value: JValue) -> ValueAggregate {
        ValueAggregate::from_service_result(
            ServiceResultAggregate::new(Rc::new(value), <_>::default(), 1.into()),
            CID::new("some fake cid").into(),
        )
    }

    fn create_value_with_pos(value: JValue, trace_pos: TracePos) -> ValueAggregate {
        ValueAggregate::from_service_result(
            ServiceResultAggregate::new(Rc::new(value), <_>::default(), trace_pos),
            CID::new("some fake cid").into(),
        )
    }

    #[test]
    fn test_iter() {
        let value_1 = create_value(json!("value_1"));
        let value_2 = create_value(json!("value_2"));
        let mut stream = Stream::new();

        stream.add_value(value_1.clone(), Generation::previous(0));
        stream.add_value(value_2.clone(), Generation::previous(1));

        let mut iter = stream.iter();
        println!("after getting iter");
        assert_eq!(iter.next(), Some(&value_1));
        assert_eq!(iter.next(), Some(&value_2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_slice_iter_prev() {
        let value_1 = create_value(json!("value_1"));
        let value_2 = create_value(json!("value_2"));
        let value_3 = create_value(json!("value_3"));
        let value_4 = create_value(json!("value_4"));
        let mut stream = Stream::new();

        stream.add_value(value_1.clone(), Generation::previous(0));
        stream.add_value(value_2.clone(), Generation::previous(0));
        stream.add_value(value_3.clone(), Generation::previous(0));
        stream.add_value(value_4.clone(), Generation::previous(0));

        let mut slice_iter = stream.slice_iter();
        assert_eq!(slice_iter.next(), Some(vec![value_1, value_2, value_3, value_4].as_slice()));
        assert_eq!(slice_iter.next(), None);
    }

    #[test]
    fn test_slice_iter_current() {
        let value_1 = create_value(json!("value_1"));
        let value_2 = create_value(json!("value_2"));
        let value_3 = create_value(json!("value_3"));
        let value_4 = create_value(json!("value_4"));
        let mut stream = Stream::new();

        stream.add_value(value_1.clone(), Generation::current(0));
        stream.add_value(value_2.clone(), Generation::current(0));
        stream.add_value(value_3.clone(), Generation::current(0));
        stream.add_value(value_4.clone(), Generation::current(0));

        let mut slice_iter = stream.slice_iter();
        assert_eq!(slice_iter.next(), Some(vec![value_1, value_2, value_3, value_4].as_slice()));
        assert_eq!(slice_iter.next(), None);
    }

    #[test]
    fn test_slice_iter_new() {
        let value_1 = create_value(json!("value_1"));
        let value_2 = create_value(json!("value_2"));
        let value_3 = create_value(json!("value_3"));
        let value_4 = create_value(json!("value_4"));
        let mut stream = Stream::new();

        stream.add_value(value_1.clone(), Generation::new());
        stream.add_value(value_2.clone(), Generation::new());
        stream.add_value(value_3.clone(), Generation::new());
        stream.add_value(value_4.clone(), Generation::new());

        let mut slice_iter = stream.slice_iter();
        assert_eq!(slice_iter.next(), Some(vec![value_1, value_2, value_3, value_4].as_slice()));
        assert_eq!(slice_iter.next(), None);
    }

    #[test]
    fn test_iter_on_empty_stream() {
        let stream = Stream::new();

        let mut slice = stream.iter();
        assert_eq!(slice.next(), None);
    }

    #[test]
    fn test_slice_on_empty_stream() {
        let stream = Stream::new();

        let mut slice = stream.slice_iter();
        assert_eq!(slice.next(), None);
    }

    #[test]
    fn generation_from_current_data_after_previous() {
        let value_1 = create_value(json!("value_1"));
        let value_2 = create_value(json!("value_2"));
        let mut stream = Stream::new();

        stream.add_value(value_1.clone(), Generation::current(0));
        stream.add_value(value_2.clone(), Generation::previous(0));

        let mut iter = stream.iter();
        assert_eq!(iter.next(), Some(&value_2));
        assert_eq!(iter.next(), Some(&value_1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn generation_from_new_data_after_current_and_previous() {
        let value_1 = create_value(json!("value_1"));
        let value_2 = create_value(json!("value_2"));
        let value_3 = create_value(json!("value_3"));
        let mut stream = Stream::new();

        stream.add_value(value_1.clone(), Generation::new());
        stream.add_value(value_2.clone(), Generation::current(0));
        stream.add_value(value_3.clone(), Generation::previous(0));

        let mut iter = stream.iter();
        assert_eq!(iter.next(), Some(&value_3));
        assert_eq!(iter.next(), Some(&value_2));
        assert_eq!(iter.next(), Some(&value_1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn empty_generations_skipped_in_slice_iter_prev() {
        let value_1 = create_value(json!("value_1"));
        let value_2 = create_value(json!("value_2"));
        let value_3 = create_value(json!("value_3"));
        let mut stream = Stream::new();

        stream.add_value(value_1.clone(), Generation::previous(0));
        stream.add_value(value_2.clone(), Generation::previous(1));
        stream.add_value(value_3.clone(), Generation::previous(3));

        let mut slice_iter = stream.slice_iter();
        assert_eq!(slice_iter.next(), Some(vec![value_1].as_slice()));
        assert_eq!(slice_iter.next(), Some(vec![value_2].as_slice()));
        assert_eq!(slice_iter.next(), Some(vec![value_3].as_slice()));
        assert_eq!(slice_iter.next(), None);
    }

    #[test]
    fn empty_generations_skipped_in_slice_iter_current() {
        let value_1 = create_value(json!("value_1"));
        let value_2 = create_value(json!("value_2"));
        let value_3 = create_value(json!("value_3"));
        let mut stream = Stream::new();

        stream.add_value(value_1.clone(), Generation::current(0));
        stream.add_value(value_2.clone(), Generation::current(1));
        stream.add_value(value_3.clone(), Generation::current(3));

        let mut slice_iter = stream.slice_iter();
        assert_eq!(slice_iter.next(), Some(vec![value_1].as_slice()));
        assert_eq!(slice_iter.next(), Some(vec![value_2].as_slice()));
        assert_eq!(slice_iter.next(), Some(vec![value_3].as_slice()));
        assert_eq!(slice_iter.next(), None);
    }

    #[test]
    fn compatification_works() {
        let value_1 = create_value_with_pos(json!("value_1"), 0.into());
        let value_2 = create_value_with_pos(json!("value_2"), 1.into());
        let value_3 = create_value_with_pos(json!("value_3"), 2.into());
        let mut stream = Stream::new();

        stream.add_value(value_1.clone(), Generation::current(0));
        stream.add_value(value_2.clone(), Generation::current(2));
        stream.add_value(value_3.clone(), Generation::current(4));

        let trace = ExecutionTrace::from(vec![]);
        let mut trace_ctx = TraceHandler::from_trace(trace.clone(), trace);
        let ap_result = ApResult::stub();
        trace_ctx.meet_ap_end(ap_result.clone());
        trace_ctx.meet_ap_end(ap_result.clone());
        trace_ctx.meet_ap_end(ap_result);

        let compatification_result = stream.compactify(&mut trace_ctx);
        assert!(compatification_result.is_ok());

        let actual_trace = trace_ctx.into_result_trace();
        let expected_trace = vec![
            ExecutedState::Ap(ApResult::new(0.into())),
            ExecutedState::Ap(ApResult::new(1.into())),
            ExecutedState::Ap(ApResult::new(2.into())),
        ];
        let expected_trace = ExecutionTrace::from(expected_trace);

        assert_eq!(actual_trace, expected_trace);
    }

    #[test]
    fn compatification_works_with_mixed_generations() {
        let value_1 = create_value_with_pos(json!("value_1"), 0.into());
        let value_2 = create_value_with_pos(json!("value_2"), 1.into());
        let value_3 = create_value_with_pos(json!("value_3"), 2.into());
        let value_4 = create_value_with_pos(json!("value_1"), 3.into());
        let value_5 = create_value_with_pos(json!("value_2"), 4.into());
        let value_6 = create_value_with_pos(json!("value_3"), 5.into());
        let mut stream = Stream::new();

        stream.add_value(value_1.clone(), Generation::new());
        stream.add_value(value_2.clone(), Generation::current(4));
        stream.add_value(value_3.clone(), Generation::current(0));
        stream.add_value(value_4.clone(), Generation::previous(100));
        stream.add_value(value_5.clone(), Generation::new());
        stream.add_value(value_6.clone(), Generation::current(2));

        let trace = ExecutionTrace::from(vec![]);
        let mut trace_ctx = TraceHandler::from_trace(trace.clone(), trace);
        let ap_result = ApResult::stub();
        trace_ctx.meet_ap_end(ap_result.clone());
        trace_ctx.meet_ap_end(ap_result.clone());
        trace_ctx.meet_ap_end(ap_result.clone());
        trace_ctx.meet_ap_end(ap_result.clone());
        trace_ctx.meet_ap_end(ap_result.clone());
        trace_ctx.meet_ap_end(ap_result);

        let compatification_result = stream.compactify(&mut trace_ctx);
        assert!(compatification_result.is_ok());

        let actual_trace = trace_ctx.into_result_trace();
        let expected_trace = vec![
            ExecutedState::Ap(ApResult::new(4.into())),
            ExecutedState::Ap(ApResult::new(3.into())),
            ExecutedState::Ap(ApResult::new(1.into())),
            ExecutedState::Ap(ApResult::new(0.into())),
            ExecutedState::Ap(ApResult::new(4.into())),
            ExecutedState::Ap(ApResult::new(2.into())),
        ];
        let expected_trace = ExecutionTrace::from(expected_trace);

        assert_eq!(actual_trace, expected_trace);
    }

    #[test]
    fn compatification_invalid_state_error() {
        let value_1 = create_value(json!("value_1"));
        let mut stream = Stream::new();

        stream.add_value(value_1.clone(), Generation::current(0));

        let trace = ExecutionTrace::from(vec![]);
        let mut trace_ctx = TraceHandler::from_trace(trace.clone(), trace);
        let canon_result = CanonResult(Rc::new(CID::new("fake canon CID")));
        trace_ctx.meet_canon_end(canon_result.clone());
        trace_ctx.meet_canon_end(canon_result.clone());
        trace_ctx.meet_canon_end(canon_result);

        let compatification_result = stream.compactify(&mut trace_ctx);
        assert!(matches!(
            compatification_result,
            Err(ExecutionError::Uncatchable(
                UncatchableError::GenerationCompatificationError(GenerationCompatificationError::TracePosPointsToInvalidState {..})
            ))
        ));
    }

    #[test]
    fn compatification_points_to_nowhere_error() {
        let value_1 = create_value(json!("value_1"));
        let mut stream = Stream::new();

        stream.add_value(value_1.clone(), Generation::current(0));

        let trace = ExecutionTrace::from(vec![]);
        let mut trace_ctx = TraceHandler::from_trace(trace.clone(), trace);

        let compatification_result = stream.compactify(&mut trace_ctx);
        assert!(matches!(
            compatification_result,
            Err(ExecutionError::Uncatchable(
                UncatchableError::GenerationCompatificationError(GenerationCompatificationError::TracePosPointsToNowhere {..})
            ))
        ));
    }
}
