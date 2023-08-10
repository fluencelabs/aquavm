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

use super::stream_map::VALUE_FIELD;
use super::CanonStream;
use super::ValueAggregate;
use crate::execution_step::execution_context::stream_map_key::StreamMapKey;
use crate::execution_step::ExecutionResult;
use crate::CanonStreamMapError::IndexIsAbsentInTheMap;
// use crate::CanonStreamMapError::NonexistentMappingIdx;
use crate::JValue;
use crate::StreamMapKeyError::NotAnObject;
use crate::StreamMapKeyError::UnsupportedKVPairObjectOrMapKeyType;
use crate::StreamMapKeyError::ValueFieldIsAbsent;
use crate::UncatchableError;

use air_interpreter_cid::CID;
use air_interpreter_data::CanonResultCidAggregate;
use polyplets::SecurityTetraplet;

use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

// #[derive(Clone)]
// pub struct ValueAndIndex<'a> {
//     pub(crate) value: &'a JValue,
//     pub(crate) value_aggregate_array_index: usize,
// }

// #[derive(Clone)]
// pub struct CanonStreamAndIndicesRefs<'refs> {
//     pub(crate) canon_stream: &'refs CanonStream,
//     pub(crate) indices: &'refs Vec<usize>,
// }

// #[derive(Debug, Clone)]
// pub struct CanonStreamAndIndices {
//     canon_stream: CanonStream,
//     indices: Vec<usize>,
// }

// impl CanonStreamAndIndices {
//     fn from_values_indices_and_tetraplet(
//         values: Vec<ValueAggregate>,
//         indices: Vec<usize>,
//         tetraplet: Rc<SecurityTetraplet>,
//     ) -> Self {
//         let canon_stream = CanonStream::from_values_and_tetraplet(values, tetraplet);
//         Self { canon_stream, indices }
//     }
// }

/// Canon stream map is a read-only map that mimics conventional map.
/// The contents of a map is fixed at a certain node.
/// The values vec contains a value per unique key.
#[derive(Debug, Clone)]
pub struct CanonStreamMap<'key> {
    values: Vec<ValueAggregate>,
    map: HashMap<StreamMapKey<'key>, CanonStream>, // key to position mapping
}

impl<'l> CanonStreamMap<'l> {
    // The argument's tetraplet is used to produce canon streams for keys so
    // that the produced canon streams share tetraplets with the original canon stream
    // rendered by canon instruction.
    pub(crate) fn from_canon_stream(canon_stream: CanonStream) -> ExecutionResult<CanonStreamMap<'l>> {
        let mut map: HashMap<StreamMapKey<'_>, CanonStream> = HashMap::new();
        for value_aggregate in canon_stream.iter() {
            let key: StreamMapKey<'static> = StreamMapKey::from_kvpair(value_aggregate.clone())
                .ok_or(UncatchableError::StreamMapKeyError(UnsupportedKVPairObjectOrMapKeyType))?;
            map.entry(key)
                .and_modify(|canon_stream| {
                    canon_stream.push(value_aggregate);
                })
                .or_insert(CanonStream::from_values_and_tetraplet(
                    vec![value_aggregate.clone()],
                    canon_stream.tetraplet().clone(),
                ));
        }
        let values = canon_stream.values;
        Ok(Self { values, map })
    }

    // This returns a number of distinct keys in the canon map.
    pub(crate) fn len(&self) -> usize {
        self.map.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub(crate) fn as_jvalue(&self) -> JValue {
        // TODO: this clone will be removed after boxed values
        let jvalue_array = self
            .values
            .iter()
            .map(|r| r.get_result().deref().clone())
            .collect::<Vec<_>>();
        JValue::Array(jvalue_array)
    }

    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = &ValueAggregate> {
        self.values.iter()
    }

    // WIP don't need if there is CanonStream per Key
    // pub(crate) fn nth(&self, idx: usize) -> Option<&ValueAggregate> {
    //     self.values.get(idx)
    // }

    pub(crate) fn tetraplet(&self) -> &Rc<SecurityTetraplet> {
        unimplemented!()
        // &self.tetraplet
    }

    pub(crate) fn index(&'l self, stream_map_key: &StreamMapKey<'l>) -> ExecutionResult<&'l CanonStream> {
        let canon_stream = self
            .map
            .get(stream_map_key)
            .ok_or(UncatchableError::CanonStreamMapError(IndexIsAbsentInTheMap))?;
        Ok(canon_stream)
        // let value = self
        //     .values
        //     .get(value_aggregate_array_index)
        //     .ok_or(UncatchableError::CanonStreamMapError(NonexistentMappingIdx))
        //     .map(from_kvpair)??;
        // Ok(ValueAndIndex {
        //     value,
        //     value_aggregate_array_index,
        // })
    }
}

use std::fmt;

impl fmt::Display for CanonStreamMap<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (key, canon_stream) in self.map.iter() {
            write!(f, "{key:?} : {canon_stream:?}, ")?;
        }
        write!(f, "]")
    }
}

#[derive(Debug, Clone)]
pub struct CanonStreamMapWithProvenance<'a> {
    pub(crate) canon_stream_map: CanonStreamMap<'a>,
    pub(crate) cid: Rc<CID<CanonResultCidAggregate>>,
}

#[allow(dead_code)]
impl<'a> CanonStreamMapWithProvenance<'a> {
    pub(crate) fn new(canon_stream_map: CanonStreamMap<'a>, cid: Rc<CID<CanonResultCidAggregate>>) -> Self {
        Self { canon_stream_map, cid }
    }
}

impl<'a> Deref for CanonStreamMapWithProvenance<'a> {
    type Target = CanonStreamMap<'a>;

    fn deref(&self) -> &Self::Target {
        &self.canon_stream_map
    }
}

fn _from_obj_idx_pair(pair: (usize, &ValueAggregate)) -> ExecutionResult<(StreamMapKey<'static>, usize)> {
    let (idx, obj) = pair;
    StreamMapKey::from_kvpair(obj.clone())
        .map(|key| (key, idx))
        .ok_or(UncatchableError::StreamMapKeyError(UnsupportedKVPairObjectOrMapKeyType).into())
}

fn _from_kvpair(value_aggregate: &ValueAggregate) -> ExecutionResult<&JValue> {
    let object = value_aggregate
        .get_result()
        .as_object()
        .ok_or(UncatchableError::StreamMapKeyError(NotAnObject))?;
    object
        .get(VALUE_FIELD)
        .ok_or(UncatchableError::StreamMapKeyError(ValueFieldIsAbsent).into())
}

#[cfg(test)]
mod test {
    use super::from_kvpair;
    use super::from_obj_idx_pair;
    use super::CanonStream;
    use super::CanonStreamMap;
    use super::ValueAndIndex;
    use crate::execution_step::execution_context::stream_map_key::StreamMapKey;
    use crate::execution_step::value_types::stream_map::from_key_value;
    use crate::execution_step::ValueAggregate;
    use crate::CanonStreamMapError::IndexIsAbsentInTheMap;
    use crate::CanonStreamMapError::NonexistentMappingIdx;
    use crate::ExecutionError;
    use crate::JValue;
    use crate::StreamMapKeyError::NotAnObject;
    use crate::StreamMapKeyError::UnsupportedKVPairObjectOrMapKeyType;
    use crate::StreamMapKeyError::ValueFieldIsAbsent;

    use crate::UncatchableError;
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

    fn create_value_aggregate_vec() -> Vec<ValueAggregate> {
        let key_1 = StreamMapKey::Str(Cow::Borrowed("key_one"));
        let value_1 = Rc::new(json!("first_value"));
        let kvpair_1 = from_key_value(key_1.clone(), value_1.as_ref());

        let key_2 = StreamMapKey::Str(Cow::Borrowed("key_two"));
        let value_2 = Rc::new(json!("second_value"));
        let kvpair_2 = from_key_value(key_2, value_2.as_ref());

        let va_1 = create_value_aggregate(kvpair_1);
        let va_2 = create_value_aggregate(kvpair_2);

        vec![va_1, va_2]
    }

    #[test]
    fn test_index_ok() {
        let va_vec = create_value_aggregate_vec();

        let canon_stream = CanonStream::from_values(va_vec, "some_tetraplet".into());
        let canon_stream_map = CanonStreamMap::from_canon_stream(canon_stream).unwrap();
        let key_1 = StreamMapKey::Str(Cow::Borrowed("key_one"));
        let value_1 = Rc::new(json!("first_value"));
        let ValueAndIndex {
            value,
            value_aggregate_array_index: _index,
        } = canon_stream_map
            .index(&key_1)
            .expect("There must be a value for this index.");

        assert_eq!(value, value_1.as_ref());
    }

    #[test]
    fn test_index_absent_key() {
        let va_vec = create_value_aggregate_vec();

        let canon_stream = CanonStream::from_values(va_vec, "some_tetraplet".into());
        let canon_stream_map = CanonStreamMap::from_canon_stream(canon_stream).unwrap();

        let absent_key = StreamMapKey::Str(Cow::Borrowed("absent_key"));
        let index_result = canon_stream_map.index(&absent_key);

        assert!(matches!(
            index_result,
            Err(ExecutionError::Uncatchable(UncatchableError::CanonStreamMapError(
                IndexIsAbsentInTheMap
            ),))
        ));
    }

    #[test]
    fn test_index_absent_idx() {
        let va_vec = create_value_aggregate_vec();

        let canon_stream = CanonStream::from_values(va_vec, "some_tetraplet".into());
        let mut canon_stream_map = CanonStreamMap::from_canon_stream(canon_stream).unwrap();
        canon_stream_map.values.clear();

        let key_with_absent_idx = StreamMapKey::Str(Cow::Borrowed("key_two"));
        let index_result = canon_stream_map.index(&key_with_absent_idx);

        assert!(matches!(
            index_result,
            Err(ExecutionError::Uncatchable(UncatchableError::CanonStreamMapError(
                NonexistentMappingIdx
            ),))
        ));
    }

    #[test]
    fn test_from_obj_idx_pair_unsupported_object_format() {
        let value_1 = Rc::new(json!("first_value"));
        let va_1 = create_value_aggregate(value_1);
        let result = from_obj_idx_pair((0, &va_1));

        assert!(matches!(
            result,
            Err(ExecutionError::Uncatchable(UncatchableError::StreamMapKeyError(
                UnsupportedKVPairObjectOrMapKeyType
            ),))
        ));
    }

    #[test]
    fn test_from_kvpair_not_an_object() {
        let value = Rc::new(json!("first_value"));
        let va = create_value_aggregate(value);
        let result = from_kvpair(&va);

        assert!(matches!(
            result,
            Err(ExecutionError::Uncatchable(UncatchableError::StreamMapKeyError(
                NotAnObject
            ),))
        ));
    }

    #[test]
    fn test_from_kvpair_value_field_is_absent() {
        let value = Rc::new(json!({"key": "first_value"}));
        let va = create_value_aggregate(value);
        let result = from_kvpair(&va);

        assert!(matches!(
            result,
            Err(ExecutionError::Uncatchable(UncatchableError::StreamMapKeyError(
                ValueFieldIsAbsent
            ),))
        ));
    }
}
