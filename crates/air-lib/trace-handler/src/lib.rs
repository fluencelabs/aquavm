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

mod data_keeper;
mod errors;
mod handler;
pub mod merger;
mod state_automata;

pub use data_keeper::KeeperError;
pub use errors::GenerationCompactificationError;
pub use errors::IntConversionError;
pub use errors::TraceHandlerError;
pub use handler::TraceHandler;
pub use merger::DataType;
pub use merger::MergeError;
pub use state_automata::StateFSMError;
pub use state_automata::SubgraphType;

pub type TraceHandlerResult<T> = std::result::Result<T, TraceHandlerError>;

use air_interpreter_data::*;
use data_keeper::DataKeeper;
use data_keeper::MergeCtx;
use merger::MergerFoldResult;
use merger::ResolvedFold;
use merger::ResolvedSubTraceDescs;
use state_automata::FSMKeeper;
use state_automata::FoldFSM;
use state_automata::ParFSM;
