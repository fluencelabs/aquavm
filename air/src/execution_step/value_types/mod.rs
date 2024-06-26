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

mod canon_stream;
mod canon_stream_map;
mod iterable;
mod jvaluable;
mod scalar;
mod stream;
mod stream_map;
mod utils;

pub type Stream = stream::Stream<ValueAggregate>;

pub(crate) use canon_stream::*;
pub(crate) use canon_stream_map::*;
pub(crate) use iterable::*;
pub(crate) use jvaluable::*;
pub(crate) use scalar::CanonResultAggregate;
pub(crate) use scalar::LiteralAggregate;
pub(crate) use scalar::ScalarRef;
pub(crate) use scalar::ServiceResultAggregate;
pub(crate) use scalar::TracePosOperate;
pub(crate) use scalar::ValueAggregate;

pub(crate) use stream::Generation;
pub(crate) use stream::IterableValue;
pub(crate) use stream::RecursiveCursorState;
pub(crate) use stream::RecursiveStreamCursor;
pub(super) use stream::STREAM_MAX_SIZE;
pub(crate) use stream_map::StreamMap;

pub(crate) use utils::populate_tetraplet_with_lambda;

use super::ExecutionResult;
