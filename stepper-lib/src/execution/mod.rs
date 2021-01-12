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

mod air;
mod boxed_value;
mod errors;
mod utils;

pub(self) use errors::ExecutionError;
pub(self) type ExecutionResult<T> = std::result::Result<T, ExecutionError>;

pub(self) use crate::build_targets::call_service;
pub(self) use crate::build_targets::get_current_peer_id;
