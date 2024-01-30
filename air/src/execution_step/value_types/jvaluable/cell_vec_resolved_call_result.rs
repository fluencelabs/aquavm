/*
 * Copyright 2020 Fluence Labs Limited
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

    fn into_jvalue(self: Box<Self>) -> JValue {
        let jvalue_iter = self.iter().map(|r| r.get_result().clone());
        JValue::array_from_iter(jvalue_iter)
    }

    fn as_tetraplets(&self) -> RcSecurityTetraplets {
        self.iter().map(|r| r.get_tetraplet()).collect::<Vec<_>>()
    }
}
