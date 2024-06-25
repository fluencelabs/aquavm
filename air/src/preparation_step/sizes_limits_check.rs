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

use super::preparation::PreparationResult;
use crate::PreparationError;

use air_interpreter_interface::RunParameters;
use air_interpreter_interface::SoftLimitsTriggering;

pub(crate) fn handle_limit_exceeding(
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
        handle_limit_exceeding(
            run_parameters,
            error,
            &mut soft_limits_triggering.air_size_limit_exceeded,
        )?;
    }

    if raw_current_data.len() as u64 > run_parameters.particle_size_limit {
        let error = PreparationError::particle_size_limit(raw_current_data.len(), run_parameters.particle_size_limit);
        handle_limit_exceeding(
            run_parameters,
            error,
            &mut soft_limits_triggering.particle_size_limit_exceeded,
        )?;
    }

    Ok(soft_limits_triggering)
}
