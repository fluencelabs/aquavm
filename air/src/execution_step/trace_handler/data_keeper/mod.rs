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

mod errors;
mod keeper;
mod merge_ctx;
mod trace_slider;

pub(crate) use errors::KeeperError;
pub(crate) use keeper::DataKeeper;
pub(crate) use keeper::DataPositions;
pub(super) use merge_ctx::MergeCtx;
pub(super) use trace_slider::TraceSlider;

pub(self) type KeeperResult<T> = std::result::Result<T, KeeperError>;

use super::ExecutedState;
use super::ExecutionTrace;
