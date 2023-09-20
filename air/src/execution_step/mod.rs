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
    pub use super::ExecutionError;
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
