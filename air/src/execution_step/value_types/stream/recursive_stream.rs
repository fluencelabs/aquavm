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

use super::Stream;
use crate::execution_step::value_types::Iterable;
use crate::execution_step::value_types::IterableItem;
use crate::execution_step::value_types::IterableVecResolvedCall;
use crate::execution_step::ValueAggregate;

use air_interpreter_data::GenerationIdx;

pub(crate) type IterableValue = Box<dyn for<'ctx> Iterable<'ctx, Item = IterableItem<'ctx>>>;

/// Tracks a state of a stream by storing last generation of every value type.
#[derive(Debug, Clone, Copy)]
pub struct StreamCursor {
    pub previous_start_idx: GenerationIdx,
    pub current_start_idx: GenerationIdx,
    pub new_start_idx: GenerationIdx,
}

/// Intended to generate values for recursive stream handling.
///
/// It could be considered as a simple state machine which should be started with
/// fold_started and then continued with next_iteration:
///    met_fold_start  - met_iteration_end - ... met_iteration_end - Exhausted
///          |                  |
///      Exhausted          Exhausted
#[derive(Debug, Clone, Copy)]
pub(crate) struct RecursiveStreamCursor {
    cursor: StreamCursor,
}

pub(crate) enum RecursiveCursorState {
    Continue(Vec<IterableValue>),
    Exhausted,
}

impl RecursiveStreamCursor {
    pub fn new() -> Self {
        Self {
            cursor: StreamCursor::empty(),
        }
    }

    pub fn met_fold_start(&mut self, stream: &mut Stream<ValueAggregate>) -> RecursiveCursorState {
        let state = self.cursor_state(stream);
        self.cursor = stream.cursor();

        if state.should_continue() {
            // add a new generation to made all consequence "new" (meaning that they are just executed on this peer)
            // write operation to this stream to write to this new generation
            stream.new_values().add_new_empty_generation();
        }

        state
    }

    pub fn met_iteration_end(&mut self, stream: &mut Stream<ValueAggregate>) -> RecursiveCursorState {
        let state = self.cursor_state(stream);

        // remove last generation if it empty to track cursor state
        remove_last_generation_if_empty(stream);
        self.cursor = stream.cursor();
        // add new last generation to store new values into this generation
        stream.new_values().add_new_empty_generation();

        state
    }

    fn cursor_state(&self, stream: &Stream<ValueAggregate>) -> RecursiveCursorState {
        let slice_iter = stream.slice_iter(self.cursor);
        let iterable = Self::slice_iter_to_iterable(slice_iter);

        RecursiveCursorState::from_iterable_values(iterable)
    }

    fn slice_iter_to_iterable<'value>(iter: impl Iterator<Item = &'value [ValueAggregate]>) -> Vec<IterableValue> {
        iter.map(|iterable| {
            let foldable = IterableVecResolvedCall::init(iterable.to_vec());
            let foldable: IterableValue = Box::new(foldable);
            foldable
        })
        .collect::<Vec<_>>()
    }
}

fn remove_last_generation_if_empty(stream: &mut Stream<ValueAggregate>) {
    if stream.new_values().last_generation_is_empty() {
        stream.new_values().remove_last_generation();
    }
}

impl StreamCursor {
    pub(crate) fn empty() -> Self {
        Self {
            previous_start_idx: GenerationIdx::from(0),
            current_start_idx: GenerationIdx::from(0),
            new_start_idx: GenerationIdx::from(0),
        }
    }

    pub(crate) fn new(
        previous_start_idx: GenerationIdx,
        current_start_idx: GenerationIdx,
        new_start_idx: GenerationIdx,
    ) -> Self {
        Self {
            previous_start_idx,
            current_start_idx,
            new_start_idx,
        }
    }
}

impl RecursiveCursorState {
    pub(crate) fn from_iterable_values(values: Vec<IterableValue>) -> Self {
        if values.is_empty() {
            Self::Exhausted
        } else {
            Self::Continue(values)
        }
    }

    pub(crate) fn should_continue(&self) -> bool {
        matches!(self, Self::Continue(_))
    }
}

#[cfg(test)]
mod test {
    use super::IterableValue;
    use super::RecursiveCursorState;
    use super::RecursiveStreamCursor;
    use super::Stream;
    use super::ValueAggregate;
    use crate::execution_step::Generation;
    use crate::execution_step::ServiceResultAggregate;
    use crate::JValue;

    use air_interpreter_cid::CID;

    fn create_value(value: impl  Into<JValue>) -> ValueAggregate {
        ValueAggregate::from_service_result(
            ServiceResultAggregate::new(value.into(), <_>::default(), 0.into()),
            CID::new("some fake cid").into(),
        )
    }

    fn iterables_unwrap(cursor_state: RecursiveCursorState) -> Vec<IterableValue> {
        match cursor_state {
            RecursiveCursorState::Continue(iterables) => iterables,
            RecursiveCursorState::Exhausted => panic!("cursor is exhausted"),
        }
    }

    #[test]
    fn fold_started_empty_if_no_values() {
        let mut stream = Stream::new();
        let mut recursive_stream = RecursiveStreamCursor::new();
        let cursor_state = recursive_stream.met_fold_start(&mut stream);

        assert!(!cursor_state.should_continue())
    }

    #[test]
    fn next_iteration_empty_if_no_values() {
        let mut stream = Stream::new();
        let mut recursive_stream = RecursiveStreamCursor::new();
        let cursor_state = recursive_stream.met_iteration_end(&mut stream);

        assert!(!cursor_state.should_continue())
    }

    #[test]
    fn next_iteration_empty_if_no_values_added() {
        let mut stream = Stream::new();
        let mut recursive_stream = RecursiveStreamCursor::new();

        let value = create_value("1");
        stream.add_value(value, Generation::current(0)).unwrap();

        let cursor_state = recursive_stream.met_fold_start(&mut stream);
        let iterables = iterables_unwrap(cursor_state);
        assert_eq!(iterables.len(), 1);

        let cursor_state = recursive_stream.met_iteration_end(&mut stream);
        assert!(!cursor_state.should_continue());
    }

    #[test]
    fn one_recursive_iteration() {
        let mut stream = Stream::new();
        let mut recursive_stream = RecursiveStreamCursor::new();

        let value = create_value("1");
        stream.add_value(value.clone(), Generation::current(0)).unwrap();

        let cursor_state = recursive_stream.met_fold_start(&mut stream);
        let iterables = iterables_unwrap(cursor_state);
        assert_eq!(iterables.len(), 1);

        stream.add_value(value.clone(), Generation::new()).unwrap();
        stream.add_value(value, Generation::new()).unwrap();

        let cursor_state = recursive_stream.met_iteration_end(&mut stream);
        let iterables = iterables_unwrap(cursor_state);
        assert_eq!(iterables.len(), 1);

        let cursor_state = recursive_stream.met_iteration_end(&mut stream);
        assert!(!cursor_state.should_continue());
    }

    #[test]
    fn add_value_into_prev_and_current() {
        let mut stream = Stream::new();
        let mut recursive_stream = RecursiveStreamCursor::new();

        let value = create_value("1");
        stream.add_value(value.clone(), Generation::current(0)).unwrap();

        let cursor_state = recursive_stream.met_fold_start(&mut stream);
        assert!(cursor_state.should_continue());

        stream.add_value(value.clone(), Generation::previous(0)).unwrap();

        let cursor_state = recursive_stream.met_iteration_end(&mut stream);
        assert!(cursor_state.should_continue());

        stream.add_value(value, Generation::current(1)).unwrap();

        let cursor_state = recursive_stream.met_iteration_end(&mut stream);
        assert!(cursor_state.should_continue());

        let cursor_state = recursive_stream.met_iteration_end(&mut stream);
        assert!(!cursor_state.should_continue());
    }
}
