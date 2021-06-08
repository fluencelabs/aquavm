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

use crate::execution_step::FoldState;
use crate::JValue;
use crate::ResolvedTriplet;

use serde::Deserialize;
use serde::Serialize;

use std::cell::RefCell;
use std::fmt::Display;
use std::fmt::Formatter;
use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResolvedCallResult {
    pub result: Rc<JValue>,
    pub triplet: Rc<ResolvedTriplet>,
}

pub(crate) enum AValue<'i> {
    JValueRef(ResolvedCallResult),
    JValueStreamRef(RefCell<Vec<ResolvedCallResult>>),
    JValueFoldCursor(FoldState<'i>),
}

impl<'i> Display for AValue<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AValue::JValueRef(value) => write!(f, "{:?}", value)?,
            AValue::JValueStreamRef(stream) => {
                write!(f, "[ ")?;
                for value in stream.borrow().iter() {
                    write!(f, "{:?} ", value)?;
                }
                write!(f, "]")?;
            }
            AValue::JValueFoldCursor(_) => {
                write!(f, "cursor")?;
            }
        }

        Ok(())
    }
}
