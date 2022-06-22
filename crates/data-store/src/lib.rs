/*
 * Copyright 2022 Fluence Labs Limited
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

use std::time::Duration;

/// This trait is used for
///   - persisting prev_data between successive calls of an interpreter
///   - logging previous, current, and new data in case of spikes
pub trait DataStore {
    type Error;

    fn initialize(&mut self) -> Result<(), Self::Error>;

    fn store_data(&mut self, data: &[u8], key: &str) -> Result<(), Self::Error>;

    fn read_data(&mut self, key: &str) -> Result<Vec<u8>, Self::Error>;

    /// Cleanup data that become obsolete.
    fn cleanup_data(&mut self, key: &str) -> Result<(), Self::Error>;

    /// Returns true if an anomaly happened and it's necessary to save execution data
    /// for debugging purposes.
    ///  execution_time - is a time taken by the interpreter to execute provided script
    ///  memory_delta - is a count of bytes on which an interpreter heap has been extended
    ///                 during execution of a particle
    fn detect_anomaly(&self, execution_time: Duration, memory_delta: usize) -> bool;

    fn collect_anomaly_data(
        &mut self,
        particle: &[u8], // it's byte because of the restriction on trait objects methods
        prev_data: &[u8],
        current_data: &[u8],
        avm_outcome: &[u8], // it's byte because of the restriction on trait objects methods
    ) -> Result<(), Self::Error>;
}
