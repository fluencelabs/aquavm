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
