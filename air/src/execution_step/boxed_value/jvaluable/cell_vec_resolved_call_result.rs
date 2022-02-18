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

use super::select_from_stream;
use super::ExecutionResult;
use super::JValuable;
use super::ValueAggregate;
use crate::execution_step::ExecutionCtx;
use crate::execution_step::RSecurityTetraplets;
use crate::JValue;
use crate::LambdaAST;
use crate::SecurityTetraplet;

use air_lambda_ast::format_ast;

use std::borrow::Cow;
use std::ops::Deref;

impl JValuable for std::cell::Ref<'_, Vec<ValueAggregate>> {
    fn apply_lambda<'i>(&self, lambda: &LambdaAST<'_>, exec_ctx: &ExecutionCtx<'i>) -> ExecutionResult<&JValue> {
        let stream_iter = self.iter().map(|r| r.result.deref());
        let select_result = select_from_stream(stream_iter, lambda, exec_ctx)?;
        Ok(select_result.result)
    }

    fn apply_lambda_with_tetraplets<'i>(
        &self,
        lambda: &LambdaAST<'_>,
        exec_ctx: &ExecutionCtx<'i>,
    ) -> ExecutionResult<(&JValue, SecurityTetraplet)> {
        let stream_iter = self.iter().map(|r| r.result.deref());
        let select_result = select_from_stream(stream_iter, lambda, exec_ctx)?;

        let tetraplet = &self[select_result.tetraplet_idx].tetraplet;
        let mut tetraplet = tetraplet.as_ref().clone();
        tetraplet.add_lambda(&format_ast(lambda));

        Ok((select_result.result, tetraplet))
    }

    fn as_jvalue(&self) -> Cow<'_, JValue> {
        let jvalue_array = self.iter().map(|r| r.result.deref().clone()).collect::<Vec<_>>();
        Cow::Owned(JValue::Array(jvalue_array))
    }

    fn into_jvalue(self: Box<Self>) -> JValue {
        let jvalue_array = self.iter().map(|r| r.result.deref().clone()).collect::<Vec<_>>();
        JValue::Array(jvalue_array)
    }

    fn as_tetraplets(&self) -> RSecurityTetraplets {
        self.iter().map(|r| r.tetraplet.clone()).collect::<Vec<_>>()
    }
}
