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

use air_interpreter_interface::RunParameters;

pub(crate) fn check_against_size_limits(
    run_parameters: &RunParameters,
    air: &str,
    raw_current_data: &[u8],
) -> PreparationResult<()> {
    if air.len() as u64 > run_parameters.air_size_limit {
        return Err(PreparationError::air_size_limit(
            air.len(),
            run_parameters.air_size_limit,
        ));
    }

    if raw_current_data.len() > run_parameters.particle_size_limit as usize {
        return Err(PreparationError::particle_size_limit(
            raw_current_data.len(),
            run_parameters.particle_size_limit,
        ));
    }

    Ok(())
}
