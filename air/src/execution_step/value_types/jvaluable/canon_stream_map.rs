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

use super::ExecutionResult;
use super::JValuable;
use crate::execution_step::lambda_applier::select_by_lambda_from_canon_map;
use crate::execution_step::lambda_applier::MapLensResult;
use crate::execution_step::value_types::CanonStreamMap;
use crate::execution_step::ExecutionCtx;
use crate::execution_step::RcSecurityTetraplets;
use crate::JValue;
use crate::LambdaAST;
use crate::SecurityTetraplet;

use air_interpreter_data::Provenance;

impl JValuable for &CanonStreamMap {
    #[inline]
    fn apply_lambda(&self, lambda: &LambdaAST<'_>, exec_ctx: &ExecutionCtx<'_>) -> ExecutionResult<JValue> {
        let select_result = select_by_lambda_from_canon_map(self, lambda, exec_ctx)?;
        Ok(select_result.result)
    }

    fn apply_lambda_with_tetraplets(
        &self,
        lambda: &LambdaAST<'_>,
        exec_ctx: &ExecutionCtx<'_>,
        root_provenance: &Provenance,
    ) -> ExecutionResult<(JValue, SecurityTetraplet, Provenance)> {
        let MapLensResult { result, tetraplet } = select_by_lambda_from_canon_map(self, lambda, exec_ctx)?;

        // Provenance is borrowed from the map.
        Ok((result, tetraplet.as_ref().clone(), root_provenance.clone()))
    }

    #[inline]
    fn as_jvalue(&self) -> JValue {
        CanonStreamMap::as_jvalue(self)
    }

    fn as_tetraplets(&self) -> RcSecurityTetraplets {
        self.iter().map(|r| r.get_tetraplet()).collect()
    }
}
