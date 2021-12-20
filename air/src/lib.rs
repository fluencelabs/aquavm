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

mod execution_step;
mod farewell_step;
mod preparation_step;
mod runner;
mod utils;

pub use air_interpreter_interface::InterpreterOutcome;
pub use air_interpreter_interface::RunParameters;
pub use air_interpreter_interface::INTERPRETER_SUCCESS;
pub use execution_step::execution_context::LastError;
pub use execution_step::CatchableError;
pub use execution_step::ExecutionError;
pub use execution_step::LambdaError;
pub use execution_step::UncatchableError;
pub use polyplets::ResolvedTriplet;
pub use polyplets::SecurityTetraplet;
pub use preparation_step::PreparationError;
pub use utils::ToErrorCode;

pub use crate::runner::execute_air;

pub mod interpreter_data {
    pub use air_interpreter_data::*;
}

pub mod parser {
    pub use air_parser::ast::Instruction;

    /// Parse an AIR script to AST.
    pub fn parse(script: &str) -> Result<Box<Instruction<'_>>, String> {
        air_parser::parse(script)
    }
}

pub(crate) type JValue = serde_json::Value;

use air_lambda_parser::LambdaAST;
