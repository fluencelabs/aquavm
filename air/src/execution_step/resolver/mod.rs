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

mod resolvable_impl;

use crate::execution_step::ExecutionCtx;
use crate::execution_step::ExecutionResult;
use crate::execution_step::RcSecurityTetraplets;
use crate::JValue;

use air_interpreter_data::Provenance;

pub(crate) trait Resolvable {
    fn resolve(&self, ctx: &ExecutionCtx<'_>) -> ExecutionResult<(JValue, RcSecurityTetraplets, Provenance)>;
}
