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

mod errors;
pub(crate) mod execution_context;
mod instructions;
mod lambda_applier;
mod resolver;
mod value_types;

const PEEK_ALLOWED_ON_NON_EMPTY: &str = "peek always return elements inside fold,\
            this guaranteed by implementation of next and avoiding empty folds";

const TETRAPLET_IDX_CORRECT: &str = "selects always return a correct index inside stream";

pub use errors::CatchableError;
pub use errors::ExecutionError;
pub use errors::UncatchableError;
pub use execution_context::ErrorObjectError;
pub use lambda_applier::LambdaError;

pub mod errors_prelude {
    pub use super::CatchableError;
    pub use super::UncatchableError;
}

pub(super) use self::instructions::ExecutableInstruction;
pub(super) use self::instructions::FoldState;
pub(crate) use errors::ErrorAffectable;
pub(crate) use errors::Joinable;
pub(crate) use execution_context::ExecutionCtx;
pub(crate) use execution_context::InstructionError;
pub(super) use value_types::CanonResultAggregate;
pub(super) use value_types::Generation;
pub(super) use value_types::LiteralAggregate;
pub(super) use value_types::ScalarRef;
pub(super) use value_types::ServiceResultAggregate;
pub(super) use value_types::Stream;
pub(super) use value_types::ValueAggregate;
use value_types::STREAM_MAX_SIZE;

pub(crate) use air_trace_handler::TraceHandler;

use std::rc::Rc;

type ExecutionResult<T> = std::result::Result<T, ExecutionError>;
type RcSecurityTetraplet = Rc<crate::SecurityTetraplet>;
type RcSecurityTetraplets = Vec<RcSecurityTetraplet>;
