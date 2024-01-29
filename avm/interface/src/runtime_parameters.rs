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

use serde::Deserialize;
use serde::Serialize;

/// Represents runtime parameters to hand over to AquaVM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeParameters {
    air_size_limit: u32,
    particle_size_limit: u32,
    call_results_size_limit: u32,
}

impl RuntimeParameters {
    pub fn new(
        air_size_limit: u32,
        particle_size_limit: u32,
        call_results_size_limit: u32,
    ) -> Self {
        Self {
            air_size_limit,
            particle_size_limit,
            call_results_size_limit,
        }
    }
}
