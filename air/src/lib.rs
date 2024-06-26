/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

#![forbid(unsafe_code)]
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
mod human_readable_data;
mod preparation_step;
mod runner;
mod signing_step;
mod utils;
mod verification_step;

pub use air_interpreter_interface::InterpreterOutcome;
pub use air_interpreter_interface::RunParameters;
pub use air_interpreter_interface::INTERPRETER_SUCCESS;
pub use execution_step::execution_context::errors::unsupported_map_key_type;
pub use execution_step::execution_context::errors::CanonStreamMapError;
pub use execution_step::execution_context::errors::StreamMapError;
pub use execution_step::execution_context::errors::StreamMapKeyError;
pub use execution_step::execution_context::no_error;
pub use execution_step::execution_context::no_error_object;
pub use execution_step::execution_context::ExecutionCidState;
pub use execution_step::execution_context::InstructionError;
pub use execution_step::execution_context::ERROR_CODE_FIELD_NAME;
pub use execution_step::execution_context::INSTRUCTION_FIELD_NAME;
pub use execution_step::execution_context::MESSAGE_FIELD_NAME;
pub use execution_step::execution_context::NO_ERROR_ERROR_CODE;
pub use execution_step::execution_context::NO_ERROR_MESSAGE;
pub use execution_step::CatchableError;
pub use execution_step::ErrorObjectError;
pub use execution_step::ExecutionError;
pub use execution_step::LambdaError;
pub use execution_step::UncatchableError;
pub use farewell_step::FarewellError;
pub use polyplets::ResolvedTriplet;
pub use polyplets::SecurityTetraplet;
pub use preparation_step::interpreter_version;
pub use preparation_step::min_supported_version;
pub use preparation_step::PreparationError;
pub use utils::ToErrorCode;

pub use crate::human_readable_data::to_human_readable_data;
pub use crate::runner::execute_air;

pub mod interpreter_data {
    pub use air_interpreter_data::*;
}

pub mod parser {
    pub use air_parser::ast::Instruction;

    /// Parse an AIR script to AST.
    pub fn parse(script: &str) -> Result<Instruction<'_>, String> {
        air_parser::parse(script)
    }
}

pub(crate) type JValue = air_interpreter_value::JValue;

use air_lambda_parser::LambdaAST;
