/*
 * Copyright 2020 Fluence Labs Limited
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

use super::JValuable;
use crate::execution_step::FoldState;
use crate::execution_step::RcSecurityTetraplet;
use crate::execution_step::PEEK_ALLOWED_ON_NON_EMPTY;
use crate::JValue;

use air_interpreter_cid::CID;
use air_interpreter_data::CanonResultAggregate;
use air_interpreter_data::ServiceResultAggregate;
use air_interpreter_data::TracePos;
use polyplets::SecurityTetraplet;
use serde::Deserialize;
use serde::Serialize;

use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ValueAggregate {
    pub result: Rc<JValue>,
    pub tetraplet: RcSecurityTetraplet,
    pub trace_pos: TracePos,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Provenance {
    Literal {
        // TODO does it differ from SecurityTetraplet.lambda_path?
        // OK, let's remove it if not needed.
        lambda_path: Option<Rc<str>>,
    },
    ServiceResult {
        // the original call result CID; not changed on lambda application
        cid: Rc<CID<ServiceResultAggregate>>,
        // TODO does it differ from SecurityTetraplet.lambda_path?
        lambda_path: Option<Rc<str>>,
    },
    Canon {
        cid: Rc<CID<CanonResultAggregate>>,
        // TODO ditto
        lambda_path: Option<Rc<str>>,
    },
}

impl Provenance {
    pub(crate) fn literal(lambda_path: Option<Rc<str>>) -> Self {
        Self::Literal { lambda_path }
    }

    pub(crate) fn service_result(cid: Rc<CID<ServiceResultAggregate>>, lambda_path: Option<Rc<str>>) -> Self {
        Self::ServiceResult { cid, lambda_path }
    }

    pub(crate) fn canon(cid: Rc<CID<CanonResultAggregate>>, lambda_path: Option<Rc<str>>) -> Self {
        Self::Canon { cid, lambda_path }
    }

    // TODO remove me and refactor all the usages
    pub(crate) fn todo() -> Self {
        Self::Literal { lambda_path: None }
    }

    pub(crate) fn apply_lambda(&self, tetraplet: &SecurityTetraplet) -> Self {
        let lambda_path = Some(tetraplet.json_path.as_str().into());
        match self {
            Provenance::Literal { .. } => Self::Literal { lambda_path },
            Provenance::ServiceResult { cid, .. } => Self::ServiceResult {
                cid: cid.clone(),
                lambda_path,
            },
            Provenance::Canon { cid, .. } => Self::Canon {
                cid: cid.clone(),
                lambda_path,
            },
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ValueAggregateWithProvenance {
    pub value_aggregate: ValueAggregate,
    pub provenance: Provenance,
}

impl DerefMut for ValueAggregateWithProvenance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value_aggregate
    }
}

impl Deref for ValueAggregateWithProvenance {
    type Target = ValueAggregate;

    fn deref(&self) -> &Self::Target {
        &self.value_aggregate
    }
}

impl ValueAggregateWithProvenance {
    pub fn new(value_aggregate: ValueAggregate, provenance: Provenance) -> Self {
        Self {
            value_aggregate,
            provenance,
        }
    }
}

pub(crate) enum ScalarRef<'i> {
    Value(&'i ValueAggregateWithProvenance),
    IterableValue(&'i FoldState<'i>),
}

impl<'i> ScalarRef<'i> {
    pub(crate) fn into_jvaluable(self) -> (Box<dyn JValuable + 'i>, Provenance) {
        match self {
            ScalarRef::Value(value) => (Box::new(value.value_aggregate.clone()), value.provenance.clone()),
            ScalarRef::IterableValue(fold_state) => {
                let peeked_value = fold_state.iterable.peek().expect(PEEK_ALLOWED_ON_NON_EMPTY);
                let provenance = peeked_value.provenance();
                (Box::new(peeked_value), provenance)
            }
        }
    }
}

impl ValueAggregate {
    pub(crate) fn new(result: Rc<JValue>, tetraplet: RcSecurityTetraplet, trace_pos: TracePos) -> Self {
        Self {
            result,
            tetraplet,
            trace_pos,
        }
    }
}

use std::fmt;

impl fmt::Display for ValueAggregate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "value: {}, tetraplet: {}, position: {} ",
            self.result, self.tetraplet, self.trace_pos
        )
    }
}

impl<'i> fmt::Display for ScalarRef<'i> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScalarRef::Value(value) => write!(f, "{value:?}")?,
            ScalarRef::IterableValue(cursor) => {
                let iterable = &cursor.iterable;
                write!(f, "cursor, current value: {:?}", iterable.peek())?;
            }
        }

        Ok(())
    }
}
