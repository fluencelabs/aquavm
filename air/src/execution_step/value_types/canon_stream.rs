/*
 * Copyright 2022 Fluence Labs Limited
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

use super::ValueAggregate;
use crate::JValue;

use air_interpreter_cid::CID;
use air_interpreter_data::CanonResultCidAggregate;
use polyplets::SecurityTetraplet;
use serde::Deserialize;
use serde::Serialize;

use std::ops::Deref;
use std::rc::Rc;

/// Canon stream is a value type lies between a scalar and a stream, it has the same algebra as
/// scalars, and represent a stream fixed at some execution point.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CanonStream {
    values: Vec<ValueAggregate>,
    // tetraplet is needed to handle adding canon streams as a whole to a stream
    tetraplet: Rc<SecurityTetraplet>,
}

impl CanonStream {
    pub(crate) fn new(values: Vec<ValueAggregate>, tetraplet: Rc<SecurityTetraplet>) -> Self {
        Self { values, tetraplet }
    }

    pub(crate) fn from_values(values: Vec<ValueAggregate>, peer_pk: String) -> Self {
        let tetraplet = SecurityTetraplet::new(peer_pk, "", "", "");
        Self {
            values,
            tetraplet: Rc::new(tetraplet),
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.values.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.values.is_empty()
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

    pub(crate) fn nth(&self, idx: usize) -> Option<&ValueAggregate> {
        self.values.get(idx)
    }

    pub(crate) fn tetraplet(&self) -> &Rc<SecurityTetraplet> {
        &self.tetraplet
    }

    pub(crate) fn push(&mut self, value_aggregate: ValueAggregate) {
        self.values.push(value_aggregate.clone());
    }

    pub fn into_values(self) -> Vec<ValueAggregate> {
        self.values
    }
}

use std::fmt;

impl fmt::Display for CanonStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for value in self.values.iter() {
            // TODO debug? only aggregate? should we drop Display entirely?
            write!(f, "{value:?}, ")?;
        }
        write!(f, "]")
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CanonStreamWithProvenance {
    pub(crate) canon_stream: CanonStream,
    pub(crate) cid: Rc<CID<CanonResultCidAggregate>>,
}

impl CanonStreamWithProvenance {
    pub(crate) fn new(canon_stream: CanonStream, cid: Rc<CID<CanonResultCidAggregate>>) -> Self {
        Self { canon_stream, cid }
    }
}

impl Deref for CanonStreamWithProvenance {
    type Target = CanonStream;

    fn deref(&self) -> &Self::Target {
        &self.canon_stream
    }
}
