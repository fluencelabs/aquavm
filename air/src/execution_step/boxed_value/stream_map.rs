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

use std::rc::Rc;

use super::ValueAggregate;
use crate::execution_step::ExecutionResult;
use crate::JValue;

use super::stream::*;

use air_interpreter_data::GenerationIdx;
use air_trace_handler::merger::ValueSource;
use air_trace_handler::TraceHandler;
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Default, Clone)]
pub struct StreamMap {
    stream: Stream,
}

impl StreamMap {
    fn envelope(key: &(impl Into<JValue> + Serialize + Clone), value: Rc<JValue>) -> Rc<JValue> {
        Rc::new(json!({ "key": key.clone(), "value": value }))
    }

    pub(crate) fn from_generations_count(previous_count: GenerationIdx, current_count: GenerationIdx) -> Self {
        Self {
            stream: Stream::from_generations_count(previous_count, current_count),
        }
    }

    pub(crate) fn from_value(key: &(impl Into<JValue> + Serialize + Clone), value: ValueAggregate) -> Self {
        let obj = StreamMap::envelope(key, value.result);
        Self {
            stream: Stream::from_value(ValueAggregate::new(obj, value.tetraplet, value.trace_pos)),
        }
    }

    pub(crate) fn insert(
        &mut self,
        key: &(impl Into<JValue> + Serialize + Clone),
        value: ValueAggregate,
        generation: Generation,
        source: ValueSource,
    ) -> ExecutionResult<GenerationIdx> {
        let obj = StreamMap::envelope(key, value.result);
        self.stream.add_value(
            ValueAggregate::new(obj, value.tetraplet, value.trace_pos),
            generation,
            source,
        )
    }

    pub(crate) fn slice_iter(&self, start: Generation, end: Generation) -> Option<StreamSliceIter<'_>> {
        self.stream.slice_iter(start, end)
    }

    pub(crate) fn iter(&self, generation: Generation) -> Option<StreamIter<'_>> {
        self.stream.iter(generation)
    }

    pub(crate) fn compactify(self, trace_ctx: &mut TraceHandler) -> ExecutionResult<GenerationIdx> {
        self.stream.compactify(trace_ctx)
    }

    pub(crate) fn last_non_empty_generation(&self) -> GenerationIdx {
        self.stream.last_non_empty_generation()
    }

    pub(crate) fn remove_last_generation_if_empty(&mut self) -> bool {
        self.stream.remove_last_generation_if_empty()
    }

    pub(crate) fn add_new_generation_if_non_empty(&mut self) -> bool {
        self.stream.add_new_generation_if_non_empty()
    }
}

use std::fmt;

impl fmt::Display for StreamMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.stream.is_empty() {
            return write!(f, "[]");
        }

        writeln!(f, "[")?;
        for (id, generation) in self.stream.get_values_iter().enumerate() {
            write!(f, " -- {id}: ")?;
            for value in generation.iter() {
                write!(f, "{value:?}, ")?;
            }
            writeln!(f)?;
        }

        write!(f, "]")
    }
}

#[cfg(test)]
mod test {
    use super::Generation;
    use super::StreamMap;
    use crate::execution_step::ValueAggregate;
    use air_trace_handler::merger::ValueSource;
    use serde_json::json;
    use std::rc::Rc;

    #[test]
    fn test_from_value() {
        let obj = json!([{"top_level": [{"first": 42},{"second": 43}]}]);
        let key = String::from("some_key");
        let value = Rc::new(obj.clone());

        let generation_idx = 0;
        let generation = Generation::Nth(generation_idx.into());
        let stream_map = StreamMap::from_value(
            &key.clone(),
            ValueAggregate::new(value.clone(), <_>::default(), 0.into()),
        );

        let mut internal_stream_iter = stream_map.stream.iter(generation).unwrap();
        let v = internal_stream_iter.next().map(|e| e.result.as_ref()).unwrap();
        let examplar = StreamMap::envelope(&key, value.clone());
        assert_eq!(*v, *examplar.as_ref());
        assert_eq!(internal_stream_iter.next(), None);

        let key = 42;
        let stream_map = StreamMap::from_value(
            &key.clone(),
            ValueAggregate::new(value.clone(), <_>::default(), 0.into()),
        );

        let mut internal_stream_iter = stream_map.stream.iter(generation).unwrap();
        let v = internal_stream_iter.next().map(|e| e.result.as_ref()).unwrap();
        let examplar = StreamMap::envelope(&key, value);
        assert_eq!(*v, *examplar.as_ref());
        assert_eq!(internal_stream_iter.next(), None);
    }

    #[test]
    fn test_insert() {
        let obj = json!([{"top_level": [{"first": 42},{"second": 43}]}]);
        let key12 = String::from("some_key");
        let value = Rc::new(obj.clone());
        let generation_idx = 0;
        let va = ValueAggregate::new(value.clone(), <_>::default(), 0.into());
        let mut stream_map = StreamMap::from_value(&key12, va.clone());
        let generation = Generation::Nth(generation_idx.into());
        let generation_idx_res = stream_map
            .insert(&key12.clone(), va.clone(), generation, ValueSource::CurrentData)
            .unwrap();
        assert_eq!(generation_idx_res, generation_idx);
        let examplar = StreamMap::envelope(&key12.clone(), value.clone());
        let s = stream_map
            .stream
            .iter(generation)
            .unwrap()
            .all(|e| *e.result.as_ref() == *examplar.as_ref());
        assert!(s);
        let key3 = "other_key";
        let generation_idx = stream_map
            .insert(&key3.clone(), va.clone(), generation, ValueSource::CurrentData)
            .unwrap();
        assert_eq!(generation_idx_res, generation_idx);
        let key4 = 42;
        let generation_idx = stream_map
            .insert(&key4, va, generation, ValueSource::CurrentData)
            .unwrap();
        assert_eq!(generation_idx_res, generation_idx);

        let mut internal_stream_iter = stream_map.stream.iter(generation).unwrap();
        let v = internal_stream_iter.next().map(|e| e.result.as_ref()).unwrap();
        assert_eq!(*v, *examplar.as_ref());
        let v = internal_stream_iter.next().map(|e| e.result.as_ref()).unwrap();
        assert_eq!(*v, *examplar.as_ref());
        let v = internal_stream_iter.next().map(|e| e.result.as_ref()).unwrap();
        let examplar = StreamMap::envelope(&key3.clone(), value.clone());
        assert_eq!(*v, *examplar.as_ref());
        let v = internal_stream_iter.next().map(|e| e.result.as_ref()).unwrap();
        let examplar = StreamMap::envelope(&key4, value.clone());
        assert_eq!(*v, *examplar.as_ref());
        assert_eq!(internal_stream_iter.next(), None);
    }
}
