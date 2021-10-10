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
use super::ResolvedCallResult;
use crate::execution_step::SecurityTetraplets;
use crate::JValue;
use crate::LambdaAST;

use air_lambda_ast::format_ast;

use std::borrow::Cow;
use std::ops::Deref;

impl JValuable for std::cell::Ref<'_, Vec<ResolvedCallResult>> {
    fn apply_lambda(&self, lambda: &LambdaAST<'_>) -> ExecutionResult<Vec<&JValue>> {
        let stream_iter = self.iter().map(|r| r.result.deref());
        let select_result = select_from_stream(stream_iter, lambda)?;
        Ok(vec![select_result.result])
    }

    fn apply_lambda_with_tetraplets(
        &self,
        lambda: &LambdaAST<'_>,
    ) -> ExecutionResult<(Vec<&JValue>, SecurityTetraplets)> {
        let stream_iter = self.iter().map(|r| r.result.deref());
        let select_result = select_from_stream(stream_iter, lambda)?;

        let tetraplet = self[select_result.tetraplet_idx].tetraplet.clone();
        tetraplet.borrow_mut().add_lambda(&format_ast(lambda));

        Ok((vec![select_result.result], vec![tetraplet]))
    }

    fn as_jvalue(&self) -> Cow<'_, JValue> {
        let jvalue_array = self.iter().map(|r| r.result.deref().clone()).collect::<Vec<_>>();
        Cow::Owned(JValue::Array(jvalue_array))
    }

    fn into_jvalue(self: Box<Self>) -> JValue {
        let jvalue_array = self.iter().map(|r| r.result.deref().clone()).collect::<Vec<_>>();
        JValue::Array(jvalue_array)
    }

    fn as_tetraplets(&self) -> SecurityTetraplets {
        self.iter().map(|r| r.tetraplet.clone()).collect::<Vec<_>>()
    }
}
