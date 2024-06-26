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
mod keeper;
mod merge_ctx;
mod trace_slider;

pub use errors::KeeperError;
pub use merge_ctx::MergeCtx;
pub use trace_slider::TraceSlider;

pub(crate) use keeper::DataKeeper;

type KeeperResult<T> = std::result::Result<T, KeeperError>;

use super::ExecutedState;
use super::ExecutionTrace;
