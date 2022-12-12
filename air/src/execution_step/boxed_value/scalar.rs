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

use air_interpreter_data::TracePos;
use serde::Deserialize;
use serde::Serialize;

use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ValueAggregate {
    pub result: Rc<JValue>,
    pub tetraplet: RcSecurityTetraplet,
    pub trace_pos: TracePos,
}

pub(crate) enum ScalarRef<'i> {
    Value(&'i ValueAggregate),
    IterableValue(&'i FoldState<'i>),
}

impl<'i> ScalarRef<'i> {
    pub(crate) fn into_jvaluable(self) -> Box<dyn JValuable + 'i> {
        match self {
            ScalarRef::Value(value) => Box::new(value.clone()),
            ScalarRef::IterableValue(fold_state) => {
                let peeked_value = fold_state.iterable.peek().expect(PEEK_ALLOWED_ON_NON_EMPTY);
                Box::new(peeked_value)
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
            "value: {}, tetraplet: {}, position: {}",
            self.result, self.tetraplet, self.trace_pos,
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
