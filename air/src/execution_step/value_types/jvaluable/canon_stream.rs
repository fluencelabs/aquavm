/*
 * Copyright 2022 Fluence Labs Limited
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

    fn into_jvalue(self: Box<Self>) -> JValue {
        CanonStream::as_jvalue(&self)
    }

    fn as_tetraplets(&self) -> RcSecurityTetraplets {
        self.iter().map(|r| r.get_tetraplet()).collect()
    }
}
