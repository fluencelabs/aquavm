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

use super::select_by_lambda_from_scalar;
use super::ExecutionResult;
use super::JValuable;
use super::LambdaAST;
use super::ValueAggregate;
use crate::execution_step::value_types::populate_tetraplet_with_lambda;
use crate::execution_step::ExecutionCtx;
use crate::execution_step::RcSecurityTetraplets;
use crate::JValue;
use crate::SecurityTetraplet;

use air_interpreter_data::Provenance;

impl JValuable for ValueAggregate {
    fn apply_lambda(&self, lambda: &LambdaAST<'_>, exec_ctx: &ExecutionCtx<'_>) -> ExecutionResult<JValue> {
        let selected_value = select_by_lambda_from_scalar(self.get_result(), lambda, exec_ctx)?;
        Ok(selected_value)
    }

    fn apply_lambda_with_tetraplets(
        &self,
        lambda: &LambdaAST<'_>,
        exec_ctx: &ExecutionCtx<'_>,
        _root_provenane: &Provenance,
    ) -> ExecutionResult<(JValue, SecurityTetraplet, Provenance)> {
        let selected_value = select_by_lambda_from_scalar(self.get_result(), lambda, exec_ctx)?;
        let tetraplet = populate_tetraplet_with_lambda(self.get_tetraplet().as_ref().clone(), lambda);

        Ok((selected_value, tetraplet, self.get_provenance()))
    }

    #[inline]
    fn as_jvalue(&self) -> JValue {
        self.get_result().clone()
    }

    fn as_tetraplets(&self) -> RcSecurityTetraplets {
        vec![self.get_tetraplet()]
    }
}
