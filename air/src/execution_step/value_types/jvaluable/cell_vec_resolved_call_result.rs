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

use super::select_by_lambda_from_stream;
use super::ExecutionResult;
use super::JValuable;
use super::ValueAggregate;
use crate::execution_step::value_types::populate_tetraplet_with_lambda;
use crate::execution_step::ExecutionCtx;
use crate::execution_step::RcSecurityTetraplets;
use crate::JValue;
use crate::LambdaAST;
use crate::SecurityTetraplet;

use air_interpreter_data::Provenance;

impl JValuable for std::cell::Ref<'_, Vec<ValueAggregate>> {
    fn apply_lambda(&self, lambda: &LambdaAST<'_>, exec_ctx: &ExecutionCtx<'_>) -> ExecutionResult<JValue> {
        let stream_iter = self.iter().map(|r| r.get_result());
        let select_result = select_by_lambda_from_stream(stream_iter, lambda, exec_ctx)?;
        Ok(select_result.result)
    }

    fn apply_lambda_with_tetraplets(
        &self,
        lambda: &LambdaAST<'_>,
        exec_ctx: &ExecutionCtx<'_>,
        root_provenance: &Provenance,
    ) -> ExecutionResult<(JValue, SecurityTetraplet, Provenance)> {
        let stream_iter = self.iter().map(|r| r.get_result());
        let select_result = select_by_lambda_from_stream(stream_iter, lambda, exec_ctx)?;

        let tetraplet = match select_result.tetraplet_idx {
            Some(idx) => {
                let tetraplet = self[idx].get_tetraplet();
                populate_tetraplet_with_lambda(tetraplet.as_ref().clone(), lambda)
            }
            None => SecurityTetraplet::new(exec_ctx.run_parameters.current_peer_id.to_string(), "", "", ""),
        };

        Ok((select_result.result, tetraplet, root_provenance.clone()))
    }

    fn as_jvalue(&self) -> JValue {
        let jvalue_iter = self.iter().map(|r| r.get_result().clone());
        JValue::array_from_iter(jvalue_iter)
    }

    fn as_tetraplets(&self) -> RcSecurityTetraplets {
        self.iter().map(|r| r.get_tetraplet()).collect::<Vec<_>>()
    }
}
