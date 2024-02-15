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

pub struct AquaVMRuntimeLimits {
    /// AIR script size limit.
    pub air_size_limit: u64,
    /// Particle data size limit.
    pub particle_size_limit: u64,
    /// Service call result size limit.
    pub call_result_size_limit: u64,
    /// Knob to enable/disable RAM consumption hard limits in AquaVM.
    pub hard_limit_enabled: bool,
}

#[derive(Default)]
pub struct AVMRuntimeLimits {
    pub air_size_limit: Option<u64>,
    pub particle_size_limit: Option<u64>,
    pub call_result_size_limit: Option<u64>,
    pub hard_limit_enabled: bool,
}

impl AquaVMRuntimeLimits {
    pub fn new(
        air_size_limit: u64,
        particle_size_limit: u64,
        call_result_size_limit: u64,
        hard_limit_enabled: bool,
    ) -> Self {
        Self {
            air_size_limit,
            particle_size_limit,
            call_result_size_limit,
            hard_limit_enabled,
        }
    }
}

impl From<AVMRuntimeLimits> for AquaVMRuntimeLimits {
    fn from(value: AVMRuntimeLimits) -> Self {
        use air_interpreter_interface::MAX_AIR_SIZE;
        use air_interpreter_interface::MAX_CALL_RESULT_SIZE;
        use air_interpreter_interface::MAX_PARTICLE_SIZE;

        AquaVMRuntimeLimits::new(
            value.air_size_limit.unwrap_or(MAX_AIR_SIZE),
            value.particle_size_limit.unwrap_or(MAX_PARTICLE_SIZE),
            value.call_result_size_limit.unwrap_or(MAX_CALL_RESULT_SIZE),
            value.hard_limit_enabled,
        )
    }
}
