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
