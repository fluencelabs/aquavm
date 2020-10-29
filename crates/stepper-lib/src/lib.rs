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
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]

mod air;
mod build_targets;
mod call_evidence;
mod errors;
mod execution;
pub mod log_targets;
mod stepper_outcome;

pub use crate::call_evidence::CallEvidencePath;
pub use crate::call_evidence::CallResult;
pub use crate::call_evidence::EvidenceState;
pub use crate::errors::AquamarineError;
pub use crate::stepper_outcome::StepperOutcome;
pub use crate::stepper_outcome::STEPPER_SUCCESS;
pub use execution::execute_aqua;

pub(crate) type Result<T> = std::result::Result<T, AquamarineError>;
pub(crate) type JValue = serde_json::Value;

pub(crate) use build_targets::call_service;
pub(crate) use build_targets::get_current_peer_id;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum AValue {
    JValueRef(Rc<JValue>),
    JValueAccumulatorRef(RefCell<Vec<Rc<JValue>>>),
    JValueFoldCursor(crate::air::FoldState),
}
