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

/// This trait controls whether to set %last_error% and :error: or not.
pub(crate) trait ErrorAffectable {
    /// Return true, if this error type affects last error
    /// (meaning that it should be set after occurring such an error).
    fn affects_last_error(&self) -> bool;
    fn affects_error(&self) -> bool;
}
