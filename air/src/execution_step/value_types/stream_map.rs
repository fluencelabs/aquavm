/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use super::Generation;
use super::Stream;
use super::ValueAggregate;
use crate::execution_step::execution_context::stream_map_key::StreamMapKey;
use crate::execution_step::execution_context::stream_map_key::KEY_FIELD_NAME;
use crate::execution_step::value_types::TracePosOperate;
use crate::execution_step::ExecutionResult;
use crate::JValue;

use air_interpreter_value::JsonString;
use air_trace_handler::TraceHandler;

pub(super) static VALUE_FIELD_NAME: &str = "value";

pub(super) fn from_key_value(key: StreamMapKey, value: &JValue) -> JValue {
    maplit::hashmap! {
        VALUE_FIELD_NAME => value.clone(),
        KEY_FIELD_NAME => key.into(),
    }
    .into()
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
        key: StreamMapKey,
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

    pub(crate) fn iter(&self) -> impl Iterator<Item = &ValueAggregate> {
        self.stream.iter()
    }

    pub(crate) fn iter_unique_key_object(&self) -> impl Iterator<Item = (JsonString, JValue)> + '_ {
        use std::collections::HashSet;
        let mut met_keys = HashSet::new();

        // There are two issues with this implementation:
        // 1. There might be key values overlap, given the key value is casted to String, e.g. 42 vs "42".
        // 2. The original kvpair key field has an unsupported type, e.g. float.
        self.stream.iter().filter_map(move |value_aggregate| {
            let (value, ..) = value_aggregate.as_inner_parts();

            let object = value.as_object()?;

            // This monadic chain casts numeric and string keys to string so that string "42" and
            // number 42 are considered equal.
            let key = object
                .get(KEY_FIELD_NAME)
                .and_then(StreamMapKey::from_value_ref)
                .and_then(|key| if met_keys.insert(key.to_key()) { Some(key) } else { None })?;

            let value = object.get(VALUE_FIELD_NAME)?.clone();

            Some((key.to_key(), value))
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

    fn create_value_aggregate(value: impl Into<JValue>) -> ValueAggregate {
        ValueAggregate::new(
            value.into(),
            <_>::default(),
            0.into(),
            air_interpreter_data::Provenance::literal(),
        )
    }

    fn compare_stream_iter<'value>(
        mut iter: impl Iterator<Item = &'value ValueAggregate>,
        key: StreamMapKey,
        value: impl Into<JValue>,
    ) -> bool {
        let value = value.into();
        let actual_value = iter.next().map(|e| e.get_result()).unwrap();
        let expected_value = from_key_value(key, &value);

        actual_value == &expected_value
    }

    #[test]
    fn test_from_value_key_str() {
        let key = StreamMapKey::Str("some_key".into());
        let value = "1";
        let value_aggregate = create_value_aggregate(value);

        let mut stream_map = StreamMap::new();
        stream_map
            .insert(key.clone(), &value_aggregate, Generation::New)
            .unwrap();
        let mut iter = stream_map.stream.iter();

        assert!(compare_stream_iter(&mut iter, key, value));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_from_value_key_int() {
        let key = StreamMapKey::I64(42.into());
        let value = "1";
        let value_aggregate = create_value_aggregate(value);

        let mut stream_map = StreamMap::new();
        stream_map
            .insert(key.clone(), &value_aggregate, Generation::New)
            .unwrap();
        let mut iter = stream_map.stream.iter();

        assert!(compare_stream_iter(&mut iter, key, value));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_insert() {
        let key_1_2 = StreamMapKey::Str("some_key".into());
        let value_1 = "1";
        let value_aggregate_1 = create_value_aggregate(value_1);

        let value_2 = "2";
        let value_aggregate_2 = create_value_aggregate(value_2);

        let mut stream_map = StreamMap::new();
        stream_map
            .insert(key_1_2.clone(), &value_aggregate_1, Generation::new())
            .unwrap();
        stream_map
            .insert(key_1_2.clone(), &value_aggregate_2, Generation::current(0))
            .unwrap();

        let key_3 = StreamMapKey::Str("other_key".into());
        let value_3 = "3";
        let value_aggregate_3 = create_value_aggregate(value_3);
        stream_map
            .insert(key_3.clone(), &value_aggregate_3, Generation::current(0))
            .unwrap();

        let key_4 = StreamMapKey::I64(42.into());
        let value_4 = "4";
        let value_aggregate_4 = create_value_aggregate(value_4);
        stream_map
            .insert(key_4.clone(), &value_aggregate_4, Generation::current(0))
            .unwrap();

        let mut iter = stream_map.stream.iter();

        assert!(compare_stream_iter(&mut iter, key_1_2.clone(), value_2));
        assert!(compare_stream_iter(&mut iter, key_3, value_3));
        assert!(compare_stream_iter(&mut iter, key_4, value_4));
        assert!(compare_stream_iter(&mut iter, key_1_2, value_1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn compactification_invalid_state_error() {
        use air_interpreter_data::CanonResult;

        let key = StreamMapKey::Str("some_key".into());
        let value = "1";
        let value_aggregate = create_value_aggregate(value);
        let mut stream_map = StreamMap::new();

        stream_map
            .insert(key, &value_aggregate, Generation::current(0))
            .unwrap();

        let trace = ExecutionTrace::from(vec![]);
        let mut trace_ctx = TraceHandler::from_trace(trace.clone(), trace);
        let canon_result = CanonResult::executed(CID::new("fake canon CID"));
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
        let key = StreamMapKey::Str("some_key".into());
        let value = "1";
        let value_aggregate = create_value_aggregate(value);
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
                let value = id.into();
                let value = ValueAggregate::new(
                    value,
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

    fn bulk_insert_into_map(
        stream_map: &mut StreamMap,
        kvpairs: &Vec<(String, ValueAggregate)>,
        kvpairs_ids: Vec<usize>,
        generations_ids: Vec<u32>,
    ) {
        kvpairs_ids
            .into_iter()
            .zip(generations_ids.into_iter())
            .for_each(|(kvpair_id, generation_id)| {
                insert_into_map(stream_map, &kvpairs[kvpair_id], Generation::current(generation_id))
            });
    }

    #[test]
    fn test_iter_unique_key_object() {
        const TEST_DATA_SIZE: usize = 5;
        let key_values = generate_key_values(TEST_DATA_SIZE);

        let key: u32 = 2;
        let value = 2;
        let value = ValueAggregate::new(
            value.into(),
            <_>::default(),
            0.into(),
            air_interpreter_data::Provenance::literal(),
        );

        let mut stream_map_json_kvpairs = StreamMap::new();
        let _ = stream_map_json_kvpairs.insert(key.into(), &value, Generation::current(0));

        bulk_insert_into_map(
            &mut stream_map_json_kvpairs,
            &key_values,
            vec![0, 0, 2, 2, 2, 1, 3],
            vec![0, 1, 1, 3, 4, 4, 2],
        );
        let mut iter = stream_map_json_kvpairs.iter_unique_key_object();

        assert_eq!(("2".into(), 2.into()), iter.next().unwrap());
        assert_eq!(("0".into(), 0.into()), iter.next().unwrap());
        assert_eq!(("3".into(), 3.into()), iter.next().unwrap());
        assert_eq!(("1".into(), 1.into()), iter.next().unwrap());
        assert_eq!(iter.next(), None);
    }
}
