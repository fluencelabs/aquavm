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

use super::stream::*;
use super::ValueAggregate;
use crate::execution_step::execution_context::stream_map_key::StreamMapKey;
use crate::execution_step::execution_context::stream_map_key::KEY_FIELD;
use crate::execution_step::ExecutionResult;
use crate::JValue;

use air_interpreter_data::GenerationIdx;
use air_trace_handler::merger::ValueSource;
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
    pub(crate) fn from_generations_count(previous_count: GenerationIdx, current_count: GenerationIdx) -> Self {
        Self {
            stream: Stream::from_generations_count(previous_count, current_count),
        }
    }

    pub(crate) fn from_value(key: StreamMapKey<'_>, value: &ValueAggregate) -> Self {
        let obj = from_key_value(key, value.get_result());
        let value = ValueAggregate::new(
            obj,
            value.get_tetraplet(),
            value.get_trace_pos(),
            value.get_provenance(),
        );
        Self {
            stream: Stream::from_value(value),
        }
    }

    pub(crate) fn insert(
        &mut self,
        key: StreamMapKey<'_>,
        value: &ValueAggregate,
        generation: Generation,
        source: ValueSource,
    ) -> ExecutionResult<GenerationIdx> {
        let obj = from_key_value(key, value.get_result());
        let value = ValueAggregate::new(
            obj,
            value.get_tetraplet(),
            value.get_trace_pos(),
            value.get_provenance(),
        );
        self.stream.add_value(value, generation, source)
    }

    pub(crate) fn compactify(self, trace_ctx: &mut TraceHandler) -> ExecutionResult<GenerationIdx> {
        self.stream.compactify(trace_ctx)
    }

    pub(crate) fn get_mut_stream_ref(&mut self) -> &mut Stream {
        &mut self.stream
    }

    /// Returns an iterator to values with unique keys.
    pub(crate) fn iter_unique_key(&self) -> impl Iterator<Item = &ValueAggregate> {
        use std::collections::HashSet;

        let mut met_keys = HashSet::new();

        // it's always possible to go through all values
        self.stream.iter(Generation::Last).unwrap().filter(move |value| {
            StreamMapKey::from_kvpair_ref(value)
                .map(|key| met_keys.insert(key))
                .unwrap_or(false)
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
    use crate::execution_step::boxed_value::stream_map::from_key_value;
    use crate::execution_step::execution_context::stream_map_key::StreamMapKey;
    use crate::execution_step::ValueAggregate;
    use air_trace_handler::merger::ValueSource;
    use serde_json::json;
    use std::borrow::Cow;
    use std::rc::Rc;

    #[test]
    fn test_from_value() {
        let obj = json!([{"top_level": [{"first": 42},{"second": 43}]}]);
        let key_str = "some_key";
        let key = StreamMapKey::Str(Cow::Borrowed(key_str));
        let value = Rc::new(obj.clone());

        let generation_idx = 0;
        let generation = Generation::Nth(generation_idx.into());
        let value_aggregate: ValueAggregate = ValueAggregate::new(
            value.clone(),
            <_>::default(),
            0.into(),
            air_interpreter_data::Provenance::literal(),
        );
        let stream_map = StreamMap::from_value(key.clone(), &value_aggregate);

        let mut internal_stream_iter = stream_map.stream.iter(generation).unwrap();
        let v = internal_stream_iter.next().map(|e| e.get_result()).unwrap();
        let examplar = from_key_value(key, value.as_ref());
        assert_eq!(*v, examplar);
        assert_eq!(internal_stream_iter.next(), None);

        let key = StreamMapKey::I64(42.into());
        let value_aggregate = ValueAggregate::new(
            value.clone(),
            <_>::default(),
            0.into(),
            air_interpreter_data::Provenance::literal(),
        );
        let stream_map = StreamMap::from_value(key.clone(), &value_aggregate);

        let mut internal_stream_iter = stream_map.stream.iter(generation).unwrap();
        let v = internal_stream_iter.next().map(|e| e.get_result().as_ref()).unwrap();
        let examplar = from_key_value(key, value.as_ref());
        assert_eq!(*v, *examplar.as_ref());
        assert_eq!(internal_stream_iter.next(), None);
    }

    #[test]
    fn test_insert() {
        let obj = json!([{"top_level": [{"first": 42},{"second": 43}]}]);
        let key_str = "some_key";
        let key12 = StreamMapKey::Str(Cow::Borrowed(key_str));
        let value = Rc::new(obj.clone());
        let generation_idx = 0;
        let value_aggregate: ValueAggregate = ValueAggregate::new(
            value.clone(),
            <_>::default(),
            0.into(),
            air_interpreter_data::Provenance::literal(),
        );
        let mut stream_map = StreamMap::from_value(key12.clone(), &value_aggregate);
        let generation = Generation::Nth(generation_idx.into());
        let generation_idx_res = stream_map
            .insert(key12.clone(), &value_aggregate, generation, ValueSource::CurrentData)
            .unwrap();
        assert_eq!(generation_idx_res, generation_idx);

        let examplar = from_key_value(key12, value.as_ref());
        let s = stream_map
            .stream
            .iter(generation)
            .unwrap()
            .all(|e| *e.get_result().as_ref() == *examplar.as_ref());
        assert!(s);

        let key_str = "other_key";
        let key3 = StreamMapKey::Str(Cow::Borrowed(key_str));
        let generation_idx = stream_map
            .insert(key3.clone(), &value_aggregate, generation, ValueSource::CurrentData)
            .unwrap();
        assert_eq!(generation_idx_res, generation_idx);

        let key4 = StreamMapKey::I64(42.into());
        let generation_idx = stream_map
            .insert(key4.clone(), &value_aggregate, generation, ValueSource::CurrentData)
            .unwrap();
        assert_eq!(generation_idx_res, generation_idx);

        let mut internal_stream_iter = stream_map.stream.iter(generation).unwrap();
        let v = internal_stream_iter.next().map(|e| e.get_result().as_ref()).unwrap();
        assert_eq!(*v, *examplar.as_ref());

        let v = internal_stream_iter.next().map(|e| e.get_result().as_ref()).unwrap();
        assert_eq!(*v, *examplar.as_ref());

        let v = internal_stream_iter.next().map(|e| e.get_result().as_ref()).unwrap();
        let examplar = from_key_value(key3, value.as_ref());
        assert_eq!(*v, *examplar.as_ref());

        let v = internal_stream_iter.next().map(|e| e.get_result().as_ref()).unwrap();
        let examplar = from_key_value(key4, value.as_ref());
        assert_eq!(*v, *examplar.as_ref());
        assert_eq!(internal_stream_iter.next(), None);
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

    fn insert_into_map(
        stream_map: &mut StreamMap,
        key_value: &(String, ValueAggregate),
        generation: Generation,
        source: ValueSource,
    ) {
        stream_map
            .insert(key_value.0.as_str().into(), &key_value.1, generation, source)
            .unwrap();
    }

    #[test]
    fn get_unique_map_keys_stream_behaves_correct_with_no_duplicates() {
        const TEST_DATA_SIZE: usize = 3;
        let key_values = generate_key_values(TEST_DATA_SIZE);
        let mut stream_map = StreamMap::from_generations_count(0.into(), TEST_DATA_SIZE.into());

        for id in 0..3 {
            insert_into_map(
                &mut stream_map,
                &key_values[id],
                Generation::nth(id as u32),
                ValueSource::CurrentData,
            );
        }

        let mut iter = stream_map.iter_unique_key();

        assert_eq!(&json!(0), iter.next().unwrap().get_result().get("value").unwrap());
        assert_eq!(&json!(1), iter.next().unwrap().get_result().get("value").unwrap());
        assert_eq!(&json!(2), iter.next().unwrap().get_result().get("value").unwrap());
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn get_unique_map_keys_stream_removes_duplicates() {
        use ValueSource::CurrentData;

        const TEST_DATA_SIZE: usize = 5;
        let key_values = generate_key_values(TEST_DATA_SIZE);

        let mut stream_map = StreamMap::from_generations_count(0.into(), TEST_DATA_SIZE.into());
        insert_into_map(&mut stream_map, &key_values[0], Generation::nth(0), CurrentData);
        insert_into_map(&mut stream_map, &key_values[0], Generation::nth(1), CurrentData);
        insert_into_map(&mut stream_map, &key_values[2], Generation::nth(1), CurrentData);
        insert_into_map(&mut stream_map, &key_values[2], Generation::nth(3), CurrentData);
        insert_into_map(&mut stream_map, &key_values[2], Generation::nth(4), CurrentData);
        insert_into_map(&mut stream_map, &key_values[1], Generation::nth(4), CurrentData);
        insert_into_map(&mut stream_map, &key_values[3], Generation::nth(2), CurrentData);

        let mut iter = stream_map.iter_unique_key();

        assert_eq!(&json!(0), iter.next().unwrap().get_result().get("value").unwrap());
        assert_eq!(&json!(2), iter.next().unwrap().get_result().get("value").unwrap());
        assert_eq!(&json!(3), iter.next().unwrap().get_result().get("value").unwrap());
        assert_eq!(&json!(1), iter.next().unwrap().get_result().get("value").unwrap());
        assert_eq!(iter.next(), None);
    }
}
