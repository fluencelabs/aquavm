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
mod merger;
mod state_automata;
mod trace_handler;

pub(crate) use errors::TraceHandlerError;
pub(crate) use merger::MergerCallResult;
pub(crate) use state_automata::SubtreeType;
pub(crate) use state_automata::ValueAndPos;
pub(crate) use trace_handler::TraceHandler;

pub(crate) type TraceHandlerResult<T> = std::result::Result<T, TraceHandlerError>;

pub(self) use air_interpreter_data::*;
pub(self) use data_keeper::DataKeeper;
pub(self) use data_keeper::TraceSlider;
pub(self) use merger::MergeCtxType;
pub(self) use merger::MergerFoldResult;
pub(self) use merger::ResolvedFoldLore;
pub(self) use merger::ResolvedFoldSubTraceLore;
pub(self) use state_automata::FSMQueue;
pub(self) use state_automata::FoldFSM;
pub(self) use state_automata::ParFSM;
pub(self) use state_automata::StateFSM;
