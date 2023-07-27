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

// use super::Stream;
use super::CanonStream;
use super::ValueAggregate;
// use crate::execution_step::Generation;
// use crate::JValue;

use crate::execution_step::ExecutionResult;
use crate::JValue;
use crate::StreamMapError::UnsupportedKVPairObjectOrMapKeyType;
use crate::UncatchableError;
// use air_interpreter_cid::CID;
// use air_interpreter_data::CanonResultCidAggregate;
use crate::execution_step::execution_context::stream_map_key::StreamMapKey;
use crate::CanonStreamMapError::IndexIsAbsentInTheMap;
use crate::CanonStreamMapError::NonexistentMappingIdx;
use air_interpreter_cid::CID;
use air_interpreter_data::CanonResultCidAggregate;
use polyplets::SecurityTetraplet;

use std::collections::HashMap;
use std::ops::Deref;
// use std::ops::Deref;
use std::rc::Rc;

/// Canon stream map is a map type that is fixed at a certain node.
#[derive(Debug, Clone)]
pub struct CanonStreamMap<'a> {
    values: Vec<ValueAggregate>,
    map: HashMap<StreamMapKey<'a>, usize>, // key to position mapping
    // tetraplet is needed to handle adding canon streams as a whole to a stream
    tetraplet: Rc<SecurityTetraplet>,
}

fn from_obj_idx_pair(pair: (usize, &ValueAggregate)) -> ExecutionResult<(StreamMapKey<'_>, usize)> {
    let (idx, obj) = pair;
    StreamMapKey::from_kvpair(obj)
        .map(|key| (key, idx))
        .ok_or(UncatchableError::StreamMapError(UnsupportedKVPairObjectOrMapKeyType).into())
}

#[allow(dead_code)]
impl<'l> CanonStreamMap<'l> {
    pub(crate) fn from_values(values: Vec<ValueAggregate>, tetraplet: Rc<SecurityTetraplet>) -> Self {
        let map = <_>::default();
        Self { values, map, tetraplet }
    }

    pub(crate) fn from_canon_stream(canon_stream: &'l CanonStream) -> ExecutionResult<CanonStreamMap<'_>> {
        let values = canon_stream.values.clone();
        let tetraplet = canon_stream.tetraplet.clone();
        let map: ExecutionResult<HashMap<StreamMapKey<'_>, usize>> =
            canon_stream.iter().enumerate().map(from_obj_idx_pair).collect();
        let map = map?;
        Ok(Self { values, map, tetraplet })
    }

    pub(crate) fn len(&self) -> usize {
        self.values.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    // WIP move into a generic function?
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

    pub(crate) fn nth(&self, idx: usize) -> Option<&ValueAggregate> {
        self.values.get(idx)
    }

    pub(crate) fn tetraplet(&self) -> &Rc<SecurityTetraplet> {
        &self.tetraplet
    }

    pub(crate) fn index(&self, stream_map_key: &StreamMapKey<'_>) -> ExecutionResult<&ValueAggregate> {
        let &value_array_idx = self
            .map
            .get(stream_map_key)
            .ok_or(UncatchableError::CanonStreamMapError(IndexIsAbsentInTheMap))?; // WIP change the error
        Ok(self
            .values
            .get(value_array_idx)
            .ok_or(UncatchableError::CanonStreamMapError(NonexistentMappingIdx))?)
        // WIP negative
    }
}

use std::fmt;

impl fmt::Display for CanonStreamMap<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for value in self.values.iter() {
            write!(f, "{value:?}, ")?;
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

    pub(crate) fn index(&self, stream_map_key: &StreamMapKey<'_>) -> ExecutionResult<&ValueAggregate> {
        self.canon_stream_map.index(stream_map_key)
    }
}

impl<'a> Deref for CanonStreamMapWithProvenance<'a> {
    type Target = CanonStreamMap<'a>;

    fn deref(&self) -> &Self::Target {
        &self.canon_stream_map
    }
}
