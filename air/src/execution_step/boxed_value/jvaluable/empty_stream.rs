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

use super::ExecutionResult;
use super::JValuable;
use super::LambdaAST;
use super::LambdaError::EmptyStream;
use crate::execution_step::CatchableError::LambdaApplierError;
use crate::execution_step::ExecutionCtx;
use crate::execution_step::RSecurityTetraplets;
use crate::JValue;
use crate::SecurityTetraplet;

use std::borrow::Cow;

impl JValuable for () {
    fn apply_lambda<'i>(&self, _lambda: &LambdaAST<'_>, _exec_ctx: &ExecutionCtx<'i>) -> ExecutionResult<&JValue> {
        // applying lambda to an empty stream will produce a join behaviour
        Err(LambdaApplierError(EmptyStream).into())
    }

    fn apply_lambda_with_tetraplets<'i>(
        &self,
        _lambda: &LambdaAST<'_>,
        _exec_ctx: &ExecutionCtx<'i>,
    ) -> ExecutionResult<(&JValue, SecurityTetraplet)> {
        // applying lambda to an empty stream will produce a join behaviour
        Err(LambdaApplierError(EmptyStream).into())
    }

    fn as_jvalue(&self) -> Cow<'_, JValue> {
        Cow::Owned(JValue::Array(vec![]))
    }

    fn into_jvalue(self: Box<Self>) -> JValue {
        JValue::Array(vec![])
    }

    fn as_tetraplets(&self) -> RSecurityTetraplets {
        vec![]
    }
}
