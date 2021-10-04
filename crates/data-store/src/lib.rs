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

/// This trait should be used to persist prev_data between successive calls of an interpreter o.
pub trait DataStore<E> {
    fn initialize(&mut self) -> Result<(), E>;

    fn store_data(&mut self, data: &[u8], key: &str) -> Result<(), E>;

    fn read_data(&mut self, key: &str) -> Result<Vec<u8>, E>;

    fn cleanup_data(&mut self, key: &str) -> Result<(), E>;
}
