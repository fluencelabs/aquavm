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

use air_interpreter_interface::SerializedCallResults;

use super::preparation::PreparationResult;
use crate::PreparationError;

const MB: usize = 1024 * 1024;
pub const MAX_AIR_SIZE: usize = 16 * MB;
pub const MAX_PARTICLE_SIZE: usize = 64 * MB;
pub const MAX_CALL_RESULTS_SIZE: usize = 32 * MB;

pub(crate) fn check_against_size_limits(
    air: &str,
    raw_current_data: &[u8],
    call_results: &SerializedCallResults,
) -> PreparationResult<()> {
    if air.len() > MAX_AIR_SIZE {
        return Err(PreparationError::air_size_limit(air.len(), MAX_AIR_SIZE));
    }

    if raw_current_data.len() > MAX_PARTICLE_SIZE {
        return Err(PreparationError::particle_size_limit(
            raw_current_data.len(),
            MAX_PARTICLE_SIZE,
        ));
    }

    if call_results.len() > MAX_CALL_RESULTS_SIZE {
        return Err(PreparationError::call_results_size_limit(
            call_results.len(),
            MAX_CALL_RESULTS_SIZE,
        ));
    }

    Ok(())
}
