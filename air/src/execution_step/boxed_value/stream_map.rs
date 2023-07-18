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

use super::Generation;
use super::Stream;
use super::ValueAggregate;
use crate::execution_step::boxed_value::TracePosOperate;
use crate::execution_step::execution_context::stream_map_key::StreamMapKey;
use crate::execution_step::ExecutionResult;
use crate::JValue;

use air_trace_handler::TraceHandler;

use serde_json::json;
use std::borrow::Cow;
use std::rc::Rc;

pub(super) fn from_key_value(key: StreamMapKey<'_>, value: &JValue) -> Rc<JValue> {
    Rc::new(json!({ "key": key, "value": value }))
}

#[derive(Debug, Clone)]
pub struct StreamMap {
    stream: Stream,
}

impl StreamMap {
    pub(crate) fn new() -> Self {
        Self { stream: Stream::new() }
    }

    pub(crate) fn insert(&mut self, key: StreamMapKey<'_>, value: &ValueAggregate, generation: Generation) {
        let obj = from_key_value(key, value.get_result());
        let value = ValueAggregate::new(
            obj,
            value.get_tetraplet(),
            value.get_trace_pos(),
            value.get_provenance(),
        );
        self.stream.add_value(value, generation)
    }

    pub(crate) fn compactify(&mut self, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        self.stream.compactify(trace_ctx)
    }

    pub(crate) fn get_mut_stream_ref(&mut self) -> &mut Stream {
        &mut self.stream
    }

    pub(crate) fn get_unique_map_keys_stream(&mut self) -> Cow<'_, Stream> {
        self.stream.get_unique_map_keys_stream()
    }
}

use std::fmt;

impl fmt::Display for StreamMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.stream.fmt(f)
    }
}

#[cfg(test)]
mod test {
    use super::Generation;
    use super::StreamMap;
    use crate::execution_step::boxed_value::stream_map::from_key_value;
    use crate::execution_step::execution_context::stream_map_key::StreamMapKey;
    use crate::execution_step::ValueAggregate;
    use crate::ExecutionError;
    use crate::JValue;
    use crate::UncatchableError;

    use air_interpreter_cid::CID;
    use air_interpreter_data::ExecutionTrace;
    use air_trace_handler::GenerationCompactificationError;
    use air_trace_handler::TraceHandler;
    use serde_json::json;

    use std::borrow::Cow;
    use std::rc::Rc;

    fn create_value_aggregate(value: Rc<JValue>) -> ValueAggregate {
        ValueAggregate::new(
            value,
            <_>::default(),
            0.into(),
            air_interpreter_data::Provenance::literal(),
        )
    }

    fn compare_stream_iter<'value>(
        mut iter: impl Iterator<Item = &'value ValueAggregate>,
        key: StreamMapKey<'_>,
        value: &Rc<JValue>,
    ) -> bool {
        let actual_value = iter.next().map(|e| e.get_result()).unwrap();
        let expected_value = from_key_value(key, value);

        actual_value == &expected_value
    }

    #[test]
    fn test_from_value_key_str() {
        let key = StreamMapKey::Str(Cow::Borrowed("some_key"));
        let value = Rc::new(json!("1"));
        let value_aggregate = create_value_aggregate(value.clone());

        let mut stream_map = StreamMap::new();
        stream_map.insert(key.clone(), &value_aggregate, Generation::New);
        let mut iter = stream_map.stream.iter();

        assert!(compare_stream_iter(&mut iter, key, &value));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_from_value_key_int() {
        let key = StreamMapKey::I64(42.into());
        let value = Rc::new(json!("1"));
        let value_aggregate = create_value_aggregate(value.clone());

        let mut stream_map = StreamMap::new();
        stream_map.insert(key.clone(), &value_aggregate, Generation::New);
        let mut iter = stream_map.stream.iter();

        assert!(compare_stream_iter(&mut iter, key, &value));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_insert() {
        let key_1_2 = StreamMapKey::Str(Cow::Borrowed("some_key"));
        let value_1 = Rc::new(json!("1"));
        let value_aggregate_1 = create_value_aggregate(value_1.clone());

        let value_2 = Rc::new(json!("2"));
        let value_aggregate_2 = create_value_aggregate(value_2.clone());

        let mut stream_map = StreamMap::new();
        stream_map.insert(key_1_2.clone(), &value_aggregate_1, Generation::New);
        stream_map.insert(key_1_2.clone(), &value_aggregate_2, Generation::Current(0.into()));

        let key_3 = StreamMapKey::Str(Cow::Borrowed("other_key"));
        let value_3 = Rc::new(json!("3"));
        let value_aggregate_3 = create_value_aggregate(value_3.clone());
        stream_map.insert(key_3.clone(), &value_aggregate_3, Generation::Current(0.into()));

        let key_4 = StreamMapKey::I64(42.into());
        let value_4 = Rc::new(json!("4"));
        let value_aggregate_4 = create_value_aggregate(value_4.clone());
        stream_map.insert(key_4.clone(), &value_aggregate_4, Generation::Current(0.into()));

        let mut iter = stream_map.stream.iter();

        assert!(compare_stream_iter(&mut iter, key_1_2.clone(), &value_2));
        assert!(compare_stream_iter(&mut iter, key_3, &value_3));
        assert!(compare_stream_iter(&mut iter, key_4, &value_4));
        assert!(compare_stream_iter(&mut iter, key_1_2, &value_1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn compactification_invalid_state_error() {
        use air_interpreter_data::CanonResult;

        let key = StreamMapKey::Str(Cow::Borrowed("some_key"));
        let value = Rc::new(json!("1"));
        let value_aggregate = create_value_aggregate(value.clone());
        let mut stream_map = StreamMap::new();

        stream_map.insert(key, &value_aggregate, Generation::current(0));

        let trace = ExecutionTrace::from(vec![]);
        let mut trace_ctx = TraceHandler::from_trace(trace.clone(), trace);
        let canon_result = CanonResult(Rc::new(CID::new("fake canon CID")));
        trace_ctx.meet_canon_end(canon_result.clone());
        trace_ctx.meet_canon_end(canon_result.clone());
        trace_ctx.meet_canon_end(canon_result);

        let compactification_result = stream_map.compactify(&mut trace_ctx);
        assert!(matches!(
            compactification_result,
            Err(ExecutionError::Uncatchable(
                UncatchableError::GenerationCompactificationError(
                    GenerationCompactificationError::TracePosPointsToInvalidState { .. }
                )
            ))
        ));
    }

    #[test]
    fn compactification_points_to_nowhere_error() {
        let key = StreamMapKey::Str(Cow::Borrowed("some_key"));
        let value = Rc::new(json!("1"));
        let value_aggregate = create_value_aggregate(value.clone());
        let mut stream_map = StreamMap::new();

        stream_map.insert(key, &value_aggregate, Generation::current(0));

        let trace = ExecutionTrace::from(vec![]);
        let mut trace_ctx = TraceHandler::from_trace(trace.clone(), trace);

        let compactification_result = stream_map.compactify(&mut trace_ctx);
        assert!(matches!(
            compactification_result,
            Err(ExecutionError::Uncatchable(
                UncatchableError::GenerationCompactificationError(
                    GenerationCompactificationError::TracePosPointsToNowhere { .. }
                )
            ))
        ));
    }
}
