/*
 * Copyright 2024 Fluence Labs Limited
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

const MB: u64 = 1024 * 1024;

/// These are RAM consumption related limits to be enforced by AquaVM.
/// There are two enforcing modes in AquaVM: soft and hard limit. The mode
/// is signalled by AquaVM function Invoker via its run parameters.
/// Soft limit mode sets a set of flags to return to the Invoker.
/// Hard limit mode forces AquaVM to return Uncatchable error if the limits
/// are exceeded.
/// The math behind the limits value is based on:
/// - 4GB value that provder guaratees for a Computation Unit that also has 1 CPU core.
/// - the fact that peak RAM consumption linearly depends on: particle size,
///     number of instructions and their types.
/// The limits values are to be re-considered after more RAM efficient in-memory representation.
pub static MAX_AIR_SIZE: u64 = 16 * MB;
pub static MAX_PARTICLE_SIZE: u64 = 64 * MB;
pub static MAX_CALL_RESULT_SIZE: u64 = 32 * MB;
