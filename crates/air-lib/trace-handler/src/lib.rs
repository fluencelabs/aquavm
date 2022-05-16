/*
 * Copyright 2021 Fluence Labs Limited
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

mod data_keeper;
mod errors;
mod handler;
mod merger;
mod state_automata;

pub use errors::TraceHandlerError;
pub use handler::TraceHandler;
pub use merger::ApResultError;
pub use merger::CallResultError;
pub use merger::FoldResultError;
pub use merger::MergeCtxType;
pub use merger::MergeError;
pub use merger::MergerApResult;
pub use merger::MergerCallResult;
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
