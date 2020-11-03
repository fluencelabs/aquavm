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

use std::hash::Hash;

/// Deduplicate values in a supplied vector.
pub(super) fn dedup<T: Eq + Hash>(mut vec: Vec<T>) -> Vec<T> {
    use std::collections::HashSet;

    let set: HashSet<_> = vec.drain(..).collect();
    set.into_iter().collect()
}
