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

mod canon_stream;
mod iterable;
mod jvaluable;
mod scalar;
mod stream;
mod stream_map;
mod utils;

pub type Stream = stream::Stream<ValueAggregate>;

pub(crate) use canon_stream::*;
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
