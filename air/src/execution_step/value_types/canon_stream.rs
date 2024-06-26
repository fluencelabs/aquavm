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
        let jvalue_iter = self.values.iter().map(|r| r.get_result().clone());
        JValue::array_from_iter(jvalue_iter)
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
        self.values.push(value_aggregate);
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
    pub(crate) cid: CID<CanonResultCidAggregate>,
}

impl CanonStreamWithProvenance {
    pub(crate) fn new(canon_stream: CanonStream, cid: CID<CanonResultCidAggregate>) -> Self {
        Self { canon_stream, cid }
    }
}

impl Deref for CanonStreamWithProvenance {
    type Target = CanonStream;

    fn deref(&self) -> &Self::Target {
        &self.canon_stream
    }
}
