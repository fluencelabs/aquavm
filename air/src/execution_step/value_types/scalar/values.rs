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

use crate::execution_step::RcSecurityTetraplet;
use crate::JValue;

use air_interpreter_data::TracePos;
use polyplets::SecurityTetraplet;
use serde::Deserialize;
use serde::Serialize;

use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
// no lambda here are literal + lambda is literal
pub struct LiteralAggregate {
    pub result: JValue,
    // this Rc is not really shared ATM, as execution passes through the Resolvable needle
    pub init_peer_id: Rc<str>,
    // TODO #[serde(skip)]
    pub trace_pos: TracePos,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ServiceResultAggregate {
    pub result: JValue,
    pub tetraplet: RcSecurityTetraplet,
    // TODO #[serde(skip)]
    pub trace_pos: TracePos,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CanonResultAggregate {
    pub result: JValue,
    pub peer_id: Rc<str>,
    pub lambda: Rc<str>,
    // TODO #[serde(skip)]
    pub trace_pos: TracePos,
}

impl LiteralAggregate {
    pub(crate) fn new(result: JValue, init_peer_id: Rc<str>, trace_pos: TracePos) -> Self {
        Self {
            result,
            init_peer_id,
            trace_pos,
        }
    }

    pub(crate) fn get_tetraplet(&self) -> RcSecurityTetraplet {
        SecurityTetraplet::literal_tetraplet(self.init_peer_id.as_ref()).into()
    }
}

impl ServiceResultAggregate {
    pub(crate) fn new(result: JValue, tetraplet: RcSecurityTetraplet, trace_pos: TracePos) -> Self {
        Self {
            result,
            tetraplet,
            trace_pos,
        }
    }
}

impl CanonResultAggregate {
    pub(crate) fn new(result: JValue, peer_id: Rc<str>, lambda: &str, trace_pos: TracePos) -> Self {
        Self {
            result,
            peer_id,
            lambda: lambda.into(),
            trace_pos,
        }
    }

    pub(crate) fn get_tetraplet(&self) -> RcSecurityTetraplet {
        SecurityTetraplet::new(self.peer_id.as_ref(), "", "", self.lambda.as_ref()).into()
    }
}
