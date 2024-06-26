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

pub(crate) mod anomaly;
pub(crate) mod plain;

use avm_interface::ParticleParameters;

use super::runner::TestInitParameters;

pub(crate) struct ExecutionData<'ctx> {
    pub(crate) air_script: String,
    pub(crate) current_data: Vec<u8>,
    pub(crate) prev_data: Vec<u8>,
    pub(crate) particle: ParticleParameters<'ctx>,
    pub(crate) test_init_parameters: TestInitParameters,
}
