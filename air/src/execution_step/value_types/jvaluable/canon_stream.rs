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
use crate::execution_step::value_types::CanonStream;
use crate::execution_step::ExecutionCtx;
use crate::execution_step::RcSecurityTetraplets;
use crate::JValue;
use crate::LambdaAST;
use crate::SecurityTetraplet;

use air_interpreter_data::Provenance;

use std::ops::Deref;

impl JValuable for &CanonStream {
    fn apply_lambda(&self, lambda: &LambdaAST<'_>, exec_ctx: &ExecutionCtx<'_>) -> ExecutionResult<JValue> {
        let iter = self.iter().map(|v| v.get_result());
        let select_result = select_by_lambda_from_stream(iter, lambda, exec_ctx)?;

        Ok(select_result.result)
    }

    fn apply_lambda_with_tetraplets(
        &self,
        lambda: &LambdaAST<'_>,
        exec_ctx: &ExecutionCtx<'_>,
        root_provenance: &Provenance,
    ) -> ExecutionResult<(JValue, SecurityTetraplet, Provenance)> {
        let iter = self.iter().map(|v| v.get_result());
        let select_result = select_by_lambda_from_stream(iter, lambda, exec_ctx)?;

        let (tetraplet, provenance) = match select_result.tetraplet_idx {
            Some(idx) => {
                let resolved_call = self.nth(idx).expect(crate::execution_step::TETRAPLET_IDX_CORRECT);
                (
                    resolved_call.get_tetraplet().deref().clone(),
                    resolved_call.get_provenance(),
                )
            }
            // TODO it seems it is not covered by tests
            None => (
                SecurityTetraplet::new(
                    exec_ctx.run_parameters.current_peer_id.to_string(),
                    lambda.to_string(),
                    "",
                    "",
                ),
                root_provenance.clone(),
            ),
        };

        Ok((select_result.result, tetraplet, provenance))
    }

    #[inline]
    fn as_jvalue(&self) -> JValue {
        CanonStream::as_jvalue(self)
    }

    fn as_tetraplets(&self) -> RcSecurityTetraplets {
        self.iter().map(|r| r.get_tetraplet()).collect()
    }
}
