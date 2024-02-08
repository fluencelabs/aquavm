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

use super::preparation::PreparationResult;
use crate::PreparationError;

use air_interpreter_interface::{RunParameters, SoftLimitsTriggering};

pub(crate) fn limit_behavior(
    run_parameters: &RunParameters,
    error: PreparationError,
    soft_limit_flag: &mut bool,
) -> PreparationResult<()> {
    *soft_limit_flag = true;

    if run_parameters.hard_limit_enabled {
        Err(error)
    } else {
        Ok(())
    }
}

pub(crate) fn check_against_size_limits(
    run_parameters: &RunParameters,
    air: &str,
    raw_current_data: &[u8],
) -> PreparationResult<SoftLimitsTriggering> {
    let mut soft_limits_triggering = SoftLimitsTriggering::default();

    if air.len() as u64 > run_parameters.air_size_limit {
        let error = PreparationError::air_size_limit(air.len(), run_parameters.air_size_limit);
        limit_behavior(
            run_parameters,
            error,
            &mut soft_limits_triggering.air_size_limit_exceeded,
        )?;
    }

    if raw_current_data.len() as u64 > run_parameters.particle_size_limit {
        let error = PreparationError::particle_size_limit(raw_current_data.len(), run_parameters.particle_size_limit);
        limit_behavior(
            run_parameters,
            error,
            &mut soft_limits_triggering.particle_size_limit_exceeded,
        )?;
    }

    Ok(soft_limits_triggering)
}
