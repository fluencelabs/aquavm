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

pub mod values;

pub(crate) use self::values::CanonResultAggregate;
pub(crate) use self::values::LiteralAggregate;
pub(crate) use self::values::ServiceResultAggregate;

use super::JValuable;
use crate::execution_step::FoldState;
use crate::execution_step::RcSecurityTetraplet;
use crate::execution_step::PEEK_ALLOWED_ON_NON_EMPTY;
use crate::JValue;

use air_interpreter_cid::CID;
use air_interpreter_data::CanonResultCidAggregate;
use air_interpreter_data::Provenance;
use air_interpreter_data::ServiceResultCidAggregate;
use air_interpreter_data::TracePos;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ValueAggregate {
    Literal(LiteralAggregate),
    ServiceResult {
        #[serde(flatten)]
        result: ServiceResultAggregate,
        // the original call result CID; not changed on lambda application
        #[serde(rename = "cid")]
        provenance_cid: CID<ServiceResultCidAggregate>,
    },
    Canon {
        #[serde(flatten)]
        result: CanonResultAggregate,
        // the original canon CID; not changed on lambda application
        #[serde(rename = "cid")]
        provenance_cid: CID<CanonResultCidAggregate>,
    },
}

pub(crate) enum ScalarRef<'i> {
    Value(&'i ValueAggregate),
    IterableValue(&'i FoldState<'i>),
}

impl<'i> ScalarRef<'i> {
    pub(crate) fn into_jvaluable(self) -> (Box<dyn JValuable + 'i>, Provenance) {
        match self {
            ScalarRef::Value(value) => (Box::new(value.clone()), value.get_provenance()),
            ScalarRef::IterableValue(fold_state) => {
                let peeked_value = fold_state.iterable.peek().expect(PEEK_ALLOWED_ON_NON_EMPTY);
                let provenance = peeked_value.provenance();
                (Box::new(peeked_value), provenance)
            }
        }
    }
}

impl ValueAggregate {
    pub(crate) fn new(
        result: JValue,
        tetraplet: RcSecurityTetraplet,
        trace_pos: TracePos,
        provenance: Provenance,
    ) -> Self {
        match provenance {
            Provenance::Literal => ValueAggregate::Literal(LiteralAggregate::new(
                result,
                tetraplet.peer_pk.as_str().into(),
                trace_pos,
            )),
            Provenance::ServiceResult { cid } => ValueAggregate::ServiceResult {
                result: ServiceResultAggregate::new(result, tetraplet, trace_pos),
                provenance_cid: cid,
            },
            Provenance::Canon { cid } => ValueAggregate::Canon {
                result: CanonResultAggregate::new(
                    result,
                    tetraplet.peer_pk.as_str().into(),
                    &tetraplet.lambda,
                    trace_pos,
                ),
                provenance_cid: cid,
            },
        }
    }

    pub(crate) fn from_literal_result(literal: LiteralAggregate) -> Self {
        Self::Literal(literal)
    }

    pub(crate) fn from_service_result(
        service_result: ServiceResultAggregate,
        service_result_agg_cid: CID<ServiceResultCidAggregate>,
    ) -> Self {
        Self::ServiceResult {
            result: service_result,
            provenance_cid: service_result_agg_cid,
        }
    }

    pub(crate) fn from_canon_result(
        canon_result: CanonResultAggregate,
        canon_result_agg_cid: CID<CanonResultCidAggregate>,
    ) -> Self {
        Self::Canon {
            result: canon_result,
            provenance_cid: canon_result_agg_cid,
        }
    }

    pub(crate) fn as_inner_parts(&self) -> (&JValue, RcSecurityTetraplet, TracePos) {
        match self {
            ValueAggregate::Literal(ref literal) => (&literal.result, literal.get_tetraplet(), literal.trace_pos),
            ValueAggregate::ServiceResult {
                result: ref service_result,
                provenance_cid: _,
            } => (
                &service_result.result,
                service_result.tetraplet.clone(),
                service_result.trace_pos,
            ),
            ValueAggregate::Canon {
                result: ref canon_result,
                provenance_cid: _,
            } => (
                &canon_result.result,
                canon_result.get_tetraplet(),
                canon_result.trace_pos,
            ),
        }
    }

    pub fn get_result(&self) -> &JValue {
        match self {
            ValueAggregate::Literal(literal) => &literal.result,
            ValueAggregate::ServiceResult {
                result: service_result,
                provenance_cid: _,
            } => &service_result.result,
            ValueAggregate::Canon {
                result: canon_result,
                provenance_cid: _,
            } => &canon_result.result,
        }
    }

    pub fn get_tetraplet(&self) -> RcSecurityTetraplet {
        match self {
            ValueAggregate::Literal(literal) => literal.get_tetraplet(),
            ValueAggregate::ServiceResult {
                result: service_result,
                provenance_cid: _,
            } => service_result.tetraplet.clone(),
            ValueAggregate::Canon {
                result: canon_result,
                provenance_cid: _,
            } => canon_result.get_tetraplet(),
        }
    }

    pub fn get_provenance(&self) -> Provenance {
        match self {
            ValueAggregate::Literal(_) => Provenance::Literal,
            ValueAggregate::ServiceResult {
                result: _,
                provenance_cid: cid,
            } => Provenance::ServiceResult { cid: cid.clone() },
            ValueAggregate::Canon {
                result: _,
                provenance_cid: cid,
            } => Provenance::Canon { cid: cid.clone() },
        }
    }
}

pub trait TracePosOperate {
    fn get_trace_pos(&self) -> TracePos;

    fn set_trace_pos(&mut self, pos: TracePos);
}

impl TracePosOperate for ValueAggregate {
    fn get_trace_pos(&self) -> TracePos {
        match self {
            ValueAggregate::Literal(literal) => literal.trace_pos,
            ValueAggregate::ServiceResult {
                result: service_result,
                provenance_cid: _,
            } => service_result.trace_pos,
            ValueAggregate::Canon {
                result: canon_result,
                provenance_cid: _,
            } => canon_result.trace_pos,
        }
    }

    fn set_trace_pos(&mut self, trace_pos: TracePos) {
        let trace_pos_ref = match self {
            ValueAggregate::Literal(literal) => &mut literal.trace_pos,
            ValueAggregate::ServiceResult {
                result: service_result,
                provenance_cid: _,
            } => &mut service_result.trace_pos,
            ValueAggregate::Canon {
                result: canon_result,
                provenance_cid: _,
            } => &mut canon_result.trace_pos,
        };
        *trace_pos_ref = trace_pos;
    }
}

use std::fmt;

impl fmt::Display for ValueAggregate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (result, tetraplet, trace_pos) = self.as_inner_parts();
        write!(
            f,
            "value: {}, tetraplet: {}, position: {}, provenance: {:?} ",
            result,
            tetraplet,
            trace_pos,
            self.get_provenance(),
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
