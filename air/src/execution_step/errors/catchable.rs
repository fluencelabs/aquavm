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

/// This trait is intended to differentiate between catchable and non-catchable error types.
/// Errors of the first type could be caught by xor, the second couldn't and should stop
/// AIR execution. This is needed to prevent some malicious data merging and manage
/// prev_data always in a valid state.
pub(crate) trait Catchable {
    /// Return true, if error is catchable.
    fn is_catchable(&self) -> bool;
}
