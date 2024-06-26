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

/// This trait is intended to differentiate between joinable and non-joinable objects.
/// Joinable objects are those that interpreter should wait on. F.e. if at least one of
/// arguments of a call instructions is joinable, the interpreter won't execute such
/// call and won't write any state for it in data. This is needed to handle collecting
/// variable from different peers in parallel.
///
/// At the moment, this trait's applied only to errors.
pub(crate) trait Joinable {
    /// Return true, if supplied object is joinable.
    fn is_joinable(&self) -> bool;
}
