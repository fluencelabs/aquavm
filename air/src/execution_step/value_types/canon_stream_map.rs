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

use super::stream_map::VALUE_FIELD_NAME;
use super::CanonStream;
use super::TracePosOperate;
use super::ValueAggregate;
use crate::execution_step::execution_context::stream_map_key::StreamMapKey;
use crate::execution_step::ExecutionResult;
use crate::ExecutionError;
use crate::JValue;
use crate::StreamMapKeyError::UnsupportedKVPairObjectOrMapKeyType;
use crate::UncatchableError;

use air_interpreter_cid::CID;
use air_interpreter_data::CanonResultCidAggregate;
use air_interpreter_value::JsonString;
use polyplets::SecurityTetraplet;

use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

/// Canon stream map is a read-only struct that mimics conventional map.
/// The contents of a map are fixed at a specific peer.
#[derive(Debug, Clone)]
pub struct CanonStreamMap {
    /// Contains all key-value pair objects in this form {"key": key, "value": value}.
    /// There might be multiple pairs with the same key.
    values: Vec<ValueAggregate>,
    /// Index access leverages the map that does key to CanonStream mapping.
    map: HashMap<StreamMapKey, CanonStream>,
    /// ap arg processing leverages this tetraplet
    tetraplet: Rc<SecurityTetraplet>,
}

impl CanonStreamMap {
    // The argument's tetraplet is used to produce canon streams for keys so
    // that the produced canon streams share tetraplets with the original canon stream
    // rendered by canon instruction.
    pub(crate) fn from_canon_stream(canon_stream: CanonStream) -> ExecutionResult<CanonStreamMap> {
        let mut map: HashMap<StreamMapKey, CanonStream> = HashMap::new();
        let tetraplet = canon_stream.tetraplet().clone();

        for kvpair_obj in canon_stream.iter() {
            let key = StreamMapKey::from_kvpair_owned(kvpair_obj)
                .ok_or(UncatchableError::StreamMapKeyError(UnsupportedKVPairObjectOrMapKeyType))?;

            let value = get_value_from_obj(kvpair_obj)?;
            let entry_canon_stream = map
                .entry(key)
                .or_insert(CanonStream::new(vec![], canon_stream.tetraplet().clone()));
            entry_canon_stream.push(value);
        }

        let values = canon_stream.into_values();
        Ok(Self { values, map, tetraplet })
    }

    // This returns a number of values in a canon map.
    pub(crate) fn len(&self) -> usize {
        self.values.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub(crate) fn as_jvalue(&self) -> JValue {
        let json_map: air_interpreter_value::Map<JsonString, JValue> =
            self.map.iter().map(|(k, v)| (k.to_key(), v.as_jvalue())).collect();
        json_map.into()
    }

    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = &ValueAggregate> {
        self.values.iter()
    }

    pub(crate) fn tetraplet(&self) -> &Rc<SecurityTetraplet> {
        &self.tetraplet
    }

    pub(crate) fn index<'self_l>(&'self_l self, stream_map_key: &StreamMapKey) -> Option<&'self_l CanonStream> {
        self.map.get(stream_map_key)
    }
}

use std::fmt;

impl fmt::Display for CanonStreamMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (key, canon_stream) in self.map.iter() {
            write!(f, "{key} : {canon_stream}, ")?;
        }
        write!(f, "]")
    }
}

#[derive(Debug, Clone)]
pub struct CanonStreamMapWithProvenance {
    pub(crate) canon_stream_map: CanonStreamMap,
    pub(crate) cid: CID<CanonResultCidAggregate>,
}

impl CanonStreamMapWithProvenance {
    pub(crate) fn new(canon_stream_map: CanonStreamMap, cid: CID<CanonResultCidAggregate>) -> Self {
        Self { canon_stream_map, cid }
    }
}

impl Deref for CanonStreamMapWithProvenance {
    type Target = CanonStreamMap;

    fn deref(&self) -> &Self::Target {
        &self.canon_stream_map
    }
}

fn get_value_from_obj(value_aggregate: &ValueAggregate) -> ExecutionResult<ValueAggregate> {
    use crate::StreamMapKeyError::NotAnObject;
    use crate::StreamMapKeyError::ValueFieldIsAbsent;

    let tetraplet = value_aggregate.get_tetraplet();
    let provenance = value_aggregate.get_provenance();
    let trace_pos = value_aggregate.get_trace_pos();
    let object = value_aggregate
        .get_result()
        .as_object()
        .ok_or(UncatchableError::StreamMapKeyError(NotAnObject))?;
    let value =
        object
            .get(VALUE_FIELD_NAME)
            .ok_or(ExecutionError::Uncatchable(UncatchableError::StreamMapKeyError(
                ValueFieldIsAbsent,
            )))?;
    let result = value.clone();
    Ok(ValueAggregate::new(result, tetraplet, trace_pos, provenance))
}

#[cfg(test)]
mod test {
    use super::get_value_from_obj;
    use super::CanonStream;
    use super::CanonStreamMap;
    use crate::execution_step::execution_context::stream_map_key::StreamMapKey;
    use crate::execution_step::value_types::stream_map::from_key_value;
    use crate::execution_step::ValueAggregate;
    use crate::JValue;

    fn create_value_aggregate(value: JValue) -> ValueAggregate {
        ValueAggregate::new(
            value,
            <_>::default(),
            0.into(),
            air_interpreter_data::Provenance::literal(),
        )
    }

    fn create_value_aggregate_and_keys_vec() -> (Vec<ValueAggregate>, Vec<&'static str>) {
        let keys = vec!["key_one", "key_two", "key_one"];
        let values = vec!["first_value", "second_value", "third_value"];
        let va_vec = keys
            .iter()
            .zip(values)
            .clone()
            .map(|(&key, value)| {
                let key = StreamMapKey::Str(key.into());
                let value = JValue::string(value);
                let kvpair = from_key_value(key.clone(), &value);
                create_value_aggregate(kvpair)
            })
            .collect();

        (va_vec, keys)
    }

    fn create_va_canon_and_keys_vecs(peer_pk: &str) -> (Vec<ValueAggregate>, Vec<CanonStream>, Vec<&'static str>) {
        let (va_vec, keys) = create_value_aggregate_and_keys_vec();

        let va_one = get_value_from_obj(&va_vec[0]).unwrap();
        let va_two = get_value_from_obj(&va_vec[1]).unwrap();
        let va_three = get_value_from_obj(&va_vec[2]).unwrap();

        let va_vec_one = vec![va_one, va_three];
        let va_vec_two = vec![va_two];
        let canon_stream_one = CanonStream::from_values(va_vec_one, peer_pk.into());
        let canon_stream_two = CanonStream::from_values(va_vec_two, peer_pk.into());

        (va_vec, vec![canon_stream_one, canon_stream_two], keys)
    }

    #[test]
    fn from_canon_stream() {
        let peer_pk = "some_tetraplet";
        let (va_vec, canon_streams, keys) = create_va_canon_and_keys_vecs(peer_pk);
        let canon_stream = CanonStream::from_values(va_vec, peer_pk.into());
        let canon_stream_map = CanonStreamMap::from_canon_stream(canon_stream).expect("This ctor call must not fail");

        let key_one = (*keys.first().expect("There must be a key")).into();
        let key_two = (*keys[1]).into();

        let canon_stream_map_key_one = canon_stream_map.map.get(&key_one).expect("There must be a key");
        let canon_stream_map_key_two = canon_stream_map.map.get(&key_two).expect("There must be a key");
        let canon_stream_one = canon_streams.first().expect("There must be a canon stream");
        let canon_stream_two = canon_streams.last().expect("There must be a canon stream");

        assert!(canon_stream_map_key_one.clone().into_values() == canon_stream_one.clone().into_values());
        assert!(canon_stream_map_key_two.clone().into_values() == canon_stream_two.clone().into_values());
    }

    #[test]
    fn test_index_ok() {
        let peer_pk = "some_tetraplet";
        let (va_vec, canon_streams, _) = create_va_canon_and_keys_vecs(peer_pk);
        let canon_stream = CanonStream::from_values(va_vec, peer_pk.into());
        let canon_stream_map =
            CanonStreamMap::from_canon_stream(canon_stream.clone()).expect("This ctor call must not fail");
        let key_one = StreamMapKey::Str("key_one".into());

        let result_canon_stream = canon_stream_map
            .index(&key_one)
            .expect("There must be a value for this index.");
        let canon_stream_one = canon_streams.first().unwrap();

        assert!(result_canon_stream.clone().into_values() == canon_stream_one.clone().into_values());

        let key_two = StreamMapKey::Str("key_two".into());
        let result_canon_stream = canon_stream_map
            .index(&key_two)
            .expect("There must be a value for this index.");
        let canon_stream_two = canon_streams.last().unwrap();

        assert!(result_canon_stream.clone().into_values() == canon_stream_two.clone().into_values());
    }
}
