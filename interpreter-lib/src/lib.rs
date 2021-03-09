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
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]

mod build_targets;
mod contexts;
mod execution;
mod preparation;

mod aqua;
pub mod log_targets;

pub use aqua_interpreter_interface::InterpreterOutcome;
pub use aqua_interpreter_interface::INTERPRETER_SUCCESS;
pub use polyplets::ResolvedTriplet;
pub use polyplets::SecurityTetraplet;

pub use aqua::execute_aqua;

pub mod execution_trace {
    pub use crate::contexts::execution_trace::CallResult;
    pub use crate::contexts::execution_trace::ExecutedState;
    pub use crate::contexts::execution_trace::ExecutionTrace;
    pub use crate::contexts::execution_trace::ParResult;
    pub use crate::contexts::execution_trace::ValueType;
}

pub mod parser {
    pub use air_parser::ast::Instruction;

    /// Parse an AIR script to AST.
    pub fn parse(script: &str) -> Result<Box<Instruction<'_>>, String> {
        air_parser::parse(script)
    }
}

pub(crate) type JValue = serde_json::Value;
