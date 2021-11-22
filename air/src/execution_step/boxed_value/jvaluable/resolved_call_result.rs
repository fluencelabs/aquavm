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

use super::select;
use super::ExecutionResult;
use super::JValuable;
use super::LambdaAST;
use super::ValueAggregate;
use crate::execution_step::RSecurityTetraplet;
use crate::execution_step::SecurityTetraplets;
use crate::JValue;

use air_lambda_ast::format_ast;

use std::borrow::Cow;
use std::ops::Deref;

impl JValuable for ValueAggregate {
    fn apply_lambda(&self, lambda: &LambdaAST<'_>) -> ExecutionResult<&JValue> {
        let selected_value = select(&self.result, lambda.iter())?;
        Ok(selected_value)
    }

    fn apply_lambda_with_tetraplets(&self, lambda: &LambdaAST<'_>) -> ExecutionResult<(&JValue, RSecurityTetraplet)> {
        let selected_value = select(&self.result, lambda.iter())?;
        let tetraplet = self.tetraplet.clone();
        tetraplet.borrow_mut().add_lambda(&format_ast(lambda));

        Ok((selected_value, tetraplet))
    }

    fn as_jvalue(&self) -> Cow<'_, JValue> {
        Cow::Borrowed(&self.result)
    }

    fn into_jvalue(self: Box<Self>) -> JValue {
        self.result.deref().clone()
    }

    fn as_tetraplets(&self) -> SecurityTetraplets {
        vec![self.tetraplet.clone()]
    }
}
