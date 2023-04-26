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

use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ValueAggregate {
    result: Rc<JValue>,
    tetraplet: RcSecurityTetraplet,
    trace_pos: TracePos,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct WithProvenance<T> {
    pub wrapped: T,
    pub provenance: Provenance,
}

impl<T> Deref for WithProvenance<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.wrapped
    }
}

impl<T> DerefMut for WithProvenance<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wrapped
    }
}

impl<T> WithProvenance<T> {
    pub fn new(wrapped: T, provenance: Provenance) -> Self {
        Self { wrapped, provenance }
    }
}

pub(crate) enum ScalarRef<'i> {
    Value(&'i WithProvenance<ValueAggregate>),
    IterableValue(&'i FoldState<'i>),
}

impl<'i> ScalarRef<'i> {
    pub(crate) fn into_jvaluable(self) -> (Box<dyn JValuable + 'i>, Provenance) {
        match self {
            ScalarRef::Value(value) => (Box::new((**value).clone()), value.provenance.clone()),
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
        result: Rc<JValue>,
        tetraplet: RcSecurityTetraplet,
        trace_pos: TracePos,
        _provenance: Provenance,
    ) -> Self {
        Self {
            result,
            tetraplet,
            trace_pos,
        }
    }

    pub(crate) fn from_literal_result(literal: LiteralAggregate) -> Self {
        let tetraplet = literal.get_tetraplet();

        Self {
            result: literal.result,
            tetraplet,
            trace_pos: literal.trace_pos,
        }
    }

    pub(crate) fn from_service_result(
        service_result: ServiceResultAggregate,
        _service_result_agg_cid: Rc<CID<ServiceResultCidAggregate>>,
    ) -> Self {
        Self {
            result: service_result.result,
            tetraplet: service_result.tetraplet,
            trace_pos: service_result.trace_pos,
        }
    }

    pub(crate) fn from_canon_result(
        canon_result: CanonResultAggregate,
        _canon_result_agg_cid: Rc<CID<CanonResultCidAggregate>>,
    ) -> Self {
        let tetraplet = canon_result.get_tetraplet();

        Self {
            result: canon_result.result,
            tetraplet,
            trace_pos: canon_result.trace_pos,
        }
    }

    pub(crate) fn as_inner_parts(&self) -> (&Rc<JValue>, RcSecurityTetraplet, TracePos) {
        (&self.result, self.tetraplet.clone(), self.trace_pos)
    }

    #[inline]
    pub fn get_result(&self) -> &Rc<JValue> {
        &self.result
    }

    #[inline]
    pub fn get_tetraplet(&self) -> RcSecurityTetraplet {
        self.tetraplet.clone()
    }

    #[inline]
    pub fn get_trace_pos(&self) -> TracePos {
        self.trace_pos
    }

    #[inline]
    pub fn set_trace_pos(&mut self, trace_pos: TracePos) {
        self.trace_pos = trace_pos;
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
