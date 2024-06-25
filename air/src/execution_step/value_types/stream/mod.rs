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

mod recursive_stream;
mod stream_definition;
mod values_matrix;

pub(crate) use recursive_stream::IterableValue;
pub(crate) use recursive_stream::RecursiveCursorState;
pub(crate) use recursive_stream::RecursiveStreamCursor;
pub(crate) use recursive_stream::StreamCursor;
pub(crate) use stream_definition::Generation;
pub(crate) use stream_definition::Stream;
pub(crate) use stream_definition::STREAM_MAX_SIZE;
