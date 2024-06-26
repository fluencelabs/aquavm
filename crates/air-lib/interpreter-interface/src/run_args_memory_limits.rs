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
