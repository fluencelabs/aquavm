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

#![allow(improper_ctypes)]
#![warn(rust_2018_idioms)]
#![deny(
    // dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    // unused_variables,
    unused_unsafe,
    unreachable_patterns
)]

mod air;
mod build_targets;
mod call_evidence;
mod errors;
mod execution;
pub mod log_targets;

pub use crate::call_evidence::CallEvidencePath;
pub use crate::call_evidence::CallResult;
pub use crate::call_evidence::EvidenceState;
pub use crate::errors::AquamarineError;
pub use air_parser::ast::Instruction;
pub use execution::execute_aqua;
pub use execution::parse;

pub use plets::ResolvedTriplet;
pub use plets::SecurityTetraplet;

pub(crate) type Result<T> = std::result::Result<T, AquamarineError>;
pub(crate) type JValue = serde_json::Value;

pub(crate) use build_targets::call_service;
pub(crate) use build_targets::get_current_peer_id;

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
    JValueAccumulatorRef(RefCell<Vec<ResolvedCallResult>>),
    JValueFoldCursor(crate::air::FoldState<'i>),
}

pub(crate) trait JValuable {}

impl<'i> Display for AValue<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AValue::JValueRef(value) => write!(f, "{:?}", value)?,
            AValue::JValueAccumulatorRef(acc) => {
                write!(f, "[ ")?;
                for value in acc.borrow().iter() {
                    write!(f, "{:?} ", value)?;
                }
                write!(f, "]")?;
            }
            AValue::JValueFoldCursor(_fold_state) => {
                write!(f, "cursor")?;
            }
        }

        Ok(())
    }
}
