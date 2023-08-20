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
use crate::execution_step::execution_context::stream_map_key::StreamMapKey;
use crate::execution_step::execution_context::stream_map_key::KEY_FIELD;
use crate::execution_step::value_types::TracePosOperate;
use crate::execution_step::ExecutionResult;
use crate::JValue;

use air_trace_handler::TraceHandler;

use serde_json::json;
use std::rc::Rc;

pub(super) static VALUE_FIELD: &str = "value";

pub(super) fn from_key_value(key: StreamMapKey<'_>, value: &JValue) -> Rc<JValue> {
    Rc::new(json!({ KEY_FIELD: key, VALUE_FIELD: value }))
}

#[derive(Debug, Default, Clone)]
pub struct StreamMap {
    stream: Stream,
}

impl StreamMap {
    pub(crate) fn new() -> Self {
        Self { stream: Stream::new() }
    }

    pub(crate) fn insert(
        &mut self,
        key: StreamMapKey<'_>,
        value: &ValueAggregate,
        generation: Generation,
    ) -> ExecutionResult<()> {
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

    /// Returns an iterator to values with unique keys.
    pub(crate) fn iter_unique_key(&self) -> impl Iterator<Item = &ValueAggregate> {
        use std::collections::HashSet;

        let mut met_keys = HashSet::new();

        self.stream.iter().filter(move |value| {
            StreamMapKey::from_kvpair_ref(value)
                .map(|key| met_keys.insert(key))
                .unwrap_or(false)
        })
    }

    /// Returns an iterator to JSON objects {"key": value} where all keys are unique.
    pub(crate) fn iter_kvpair_as_in_json(&self) -> impl Iterator<Item = ValueAggregate> + '_ {
        use std::collections::HashSet;

        let mut met_keys = HashSet::new();

        // There are two issues with this implementation:
        // 1. There might be key values overlap, given the key value is casted to String, e.g. 42 vs "42".
        // 2. If they original kvpair key field might has an unsupported type, e.g. float.
        self.stream.iter().filter_map(move |value_aggregate| {
            let provenance = value_aggregate.get_provenance();
            let (value, tetraplet, trace_pos) = value_aggregate.as_inner_parts();

            let obj = value.as_object();
            let key = obj
                .and_then(|obj| obj.get(KEY_FIELD))
                .and_then(|key| key.as_str())
                .and_then(|key| if met_keys.insert(key) { Some(key) } else { None })?;

            let value = obj.and_then(|obj| obj.get(VALUE_FIELD))?;

            let result = Rc::new(json!({ key: value }));
            Some(ValueAggregate::new(result, tetraplet, trace_pos, provenance))
        })
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
    use crate::execution_step::execution_context::stream_map_key::StreamMapKey;
    use crate::execution_step::value_types::stream_map::from_key_value;
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
        stream_map
            .insert(key.clone(), &value_aggregate, Generation::New)
            .unwrap();
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
        stream_map
            .insert(key.clone(), &value_aggregate, Generation::New)
            .unwrap();
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
        stream_map
            .insert(key_1_2.clone(), &value_aggregate_1, Generation::new())
            .unwrap();
        stream_map
            .insert(key_1_2.clone(), &value_aggregate_2, Generation::current(0))
            .unwrap();

        let key_3 = StreamMapKey::Str(Cow::Borrowed("other_key"));
        let value_3 = Rc::new(json!("3"));
        let value_aggregate_3 = create_value_aggregate(value_3.clone());
        stream_map
            .insert(key_3.clone(), &value_aggregate_3, Generation::current(0))
            .unwrap();

        let key_4 = StreamMapKey::I64(42.into());
        let value_4 = Rc::new(json!("4"));
        let value_aggregate_4 = create_value_aggregate(value_4.clone());
        stream_map
            .insert(key_4.clone(), &value_aggregate_4, Generation::current(0))
            .unwrap();

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

        stream_map
            .insert(key, &value_aggregate, Generation::current(0))
            .unwrap();

        let trace = ExecutionTrace::from(vec![]);
        let mut trace_ctx = TraceHandler::from_trace(trace.clone(), trace);
        let canon_result = CanonResult::executed(Rc::new(CID::new("fake canon CID")));
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

        stream_map
            .insert(key, &value_aggregate, Generation::current(0))
            .unwrap();

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

    fn generate_key_values(count: usize) -> Vec<(String, ValueAggregate)> {
        (0..count)
            .map(|id| {
                let key = id.to_string();
                let value = json!(id);
                let value = ValueAggregate::new(
                    Rc::new(value),
                    <_>::default(),
                    0.into(),
                    air_interpreter_data::Provenance::literal(),
                );

                (key, value)
            })
            .collect()
    }

    fn insert_into_map(stream_map: &mut StreamMap, key_value: &(String, ValueAggregate), generation: Generation) {
        stream_map
            .insert(key_value.0.as_str().into(), &key_value.1, generation)
            .unwrap();
    }

    #[test]
    fn get_unique_map_keys_stream_behaves_correct_with_no_duplicates() {
        const TEST_DATA_SIZE: usize = 3;
        let key_values = generate_key_values(TEST_DATA_SIZE);
        let mut stream_map = StreamMap::new();

        for id in 0..3 {
            insert_into_map(&mut stream_map, &key_values[id], Generation::current(id as u32));
        }

        let mut iter = stream_map.iter_unique_key();

        assert_eq!(&json!(0), iter.next().unwrap().get_result().get("value").unwrap());
        assert_eq!(&json!(1), iter.next().unwrap().get_result().get("value").unwrap());
        assert_eq!(&json!(2), iter.next().unwrap().get_result().get("value").unwrap());
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn get_unique_map_keys_stream_removes_duplicates() {
        const TEST_DATA_SIZE: usize = 5;
        let key_values = generate_key_values(TEST_DATA_SIZE);

        let mut stream_map = StreamMap::new();
        insert_into_map(&mut stream_map, &key_values[0], Generation::current(0));
        insert_into_map(&mut stream_map, &key_values[0], Generation::current(1));
        insert_into_map(&mut stream_map, &key_values[2], Generation::current(1));
        insert_into_map(&mut stream_map, &key_values[2], Generation::current(3));
        insert_into_map(&mut stream_map, &key_values[2], Generation::current(4));
        insert_into_map(&mut stream_map, &key_values[1], Generation::current(4));
        insert_into_map(&mut stream_map, &key_values[3], Generation::current(2));

        let mut iter = stream_map.iter_unique_key();

        assert_eq!(&json!(0), iter.next().unwrap().get_result().get("value").unwrap());
        assert_eq!(&json!(2), iter.next().unwrap().get_result().get("value").unwrap());
        assert_eq!(&json!(3), iter.next().unwrap().get_result().get("value").unwrap());
        assert_eq!(&json!(1), iter.next().unwrap().get_result().get("value").unwrap());
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn get_unique_map_keys_stream_removes_duplicates() {
        const TEST_DATA_SIZE: usize = 5;
        let key_values = generate_key_values(TEST_DATA_SIZE);

        let mut stream_map = StreamMap::new();
        insert_into_map(&mut stream_map, &key_values[0], Generation::current(0));
        insert_into_map(&mut stream_map, &key_values[0], Generation::current(1));
        insert_into_map(&mut stream_map, &key_values[2], Generation::current(1));
        insert_into_map(&mut stream_map, &key_values[2], Generation::current(3));
        insert_into_map(&mut stream_map, &key_values[2], Generation::current(4));
        insert_into_map(&mut stream_map, &key_values[1], Generation::current(4));
        insert_into_map(&mut stream_map, &key_values[3], Generation::current(2));

        let mut iter = stream_map.iter_unique_key();

        assert_eq!(&json!(0), iter.next().unwrap().get_result().get("value").unwrap());
        assert_eq!(&json!(2), iter.next().unwrap().get_result().get("value").unwrap());
        assert_eq!(&json!(3), iter.next().unwrap().get_result().get("value").unwrap());
        assert_eq!(&json!(1), iter.next().unwrap().get_result().get("value").unwrap());
        assert_eq!(iter.next(), None);
    }
}
