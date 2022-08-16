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

use super::select_from_stream;
use super::ExecutionResult;
use super::JValuable;
use crate::execution_step::boxed_value::CanonStream;
use crate::execution_step::ExecutionCtx;
use crate::execution_step::RcSecurityTetraplets;
use crate::JValue;
use crate::LambdaAST;
use crate::SecurityTetraplet;

use air_lambda_ast::format_ast;

use std::borrow::Cow;
use std::ops::Deref;

impl JValuable for &CanonStream {
    fn apply_lambda<'i>(&self, lambda: &LambdaAST<'_>, exec_ctx: &ExecutionCtx<'i>) -> ExecutionResult<&JValue> {
        let iter = self.iter().map(|v| v.result.deref());
        let select_result = select_from_stream(iter, lambda, exec_ctx)?;

        Ok(select_result.result)
    }

    fn apply_lambda_with_tetraplets<'i>(
        &self,
        lambda: &LambdaAST<'_>,
        exec_ctx: &ExecutionCtx<'i>,
    ) -> ExecutionResult<(&JValue, SecurityTetraplet)> {
        let iter = self.iter().map(|v| v.result.deref());
        let select_result = select_from_stream(iter, lambda, exec_ctx)?;

        // unwrap is safe here because each value has a tetraplet and a lambda always returns a valid index
        let resolved_call = self.iter().nth(select_result.tetraplet_idx).unwrap();
        let mut tetraplet = resolved_call.tetraplet.as_ref().clone();
        tetraplet.add_lambda(&format_ast(lambda));

        Ok((select_result.result, tetraplet))
    }

    fn as_jvalue(&self) -> Cow<'_, JValue> {
        let jvalue = CanonStream::as_jvalue(self);
        Cow::Owned(jvalue)
    }

    fn into_jvalue(self: Box<Self>) -> JValue {
        self.as_jvalue()
    }

    fn as_tetraplets(&self) -> RcSecurityTetraplets {
        self.iter().map(|r| r.tetraplet.clone()).collect::<Vec<_>>()
    }
}
