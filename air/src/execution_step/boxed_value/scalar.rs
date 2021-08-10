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
use crate::JValue;
use crate::ResolvedTriplet;

use serde::Deserialize;
use serde::Serialize;

use std::fmt::Display;
use std::fmt::Formatter;
use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResolvedCallResult {
    pub result: Rc<JValue>,
    pub triplet: Rc<ResolvedTriplet>,
    pub trace_pos: usize,
}

pub(crate) enum Scalar<'i> {
    JValueRef(ResolvedCallResult),
    JValueFoldCursor(FoldState<'i>),
}

impl<'i> Scalar<'i> {
    pub(crate) fn to_jvaluable<'ctx>(&'ctx self) -> Box<dyn JValuable + 'ctx> {
        match self {
            Scalar::JValueRef(value) => Box::new(value.clone()),
            Scalar::JValueFoldCursor(fold_state) => {
                let peeked_value = fold_state.iterable.peek().unwrap();
                Box::new(peeked_value)
            }
        }
    }
}

impl ResolvedCallResult {
    pub(crate) fn new(result: Rc<JValue>, triplet: Rc<ResolvedTriplet>, trace_pos: usize) -> Self {
        Self {
            result,
            triplet,
            trace_pos,
        }
    }
}

impl<'i> Display for Scalar<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Scalar::JValueRef(value) => write!(f, "{:?}", value)?,
            Scalar::JValueFoldCursor(cursor) => {
                let iterable = &cursor.iterable;
                write!(f, "cursor, current value: {:?}", iterable.peek())?;
            }
        }

        Ok(())
    }
}
