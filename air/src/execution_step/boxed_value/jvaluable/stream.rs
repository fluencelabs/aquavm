/*
 * Copyright 2021 Fluence Labs Limited
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
use crate::execution_step::boxed_value::populate_tetraplet_with_lambda;
use crate::execution_step::boxed_value::Generation;
use crate::execution_step::boxed_value::Stream;
use crate::execution_step::ExecutionCtx;
use crate::execution_step::RcSecurityTetraplets;
use crate::JValue;
use crate::LambdaAST;
use crate::SecurityTetraplet;

use std::borrow::Cow;
use std::ops::Deref;

#[derive(Debug)]
pub(crate) struct StreamJvaluableIngredients<'stream> {
    pub(crate) stream: &'stream Stream,
    pub(crate) generation: Generation,
}

// TODO: this will be deleted soon, because it would be impossible to use streams without
// canonicalization as an arg of a call
impl JValuable for StreamJvaluableIngredients<'_> {
    fn apply_lambda(&self, lambda: &LambdaAST<'_>, exec_ctx: &ExecutionCtx<'_>) -> ExecutionResult<Cow<'_, JValue>> {
        let iter = self.iter()?.map(|v| v.result.deref());
        let select_result = select_by_lambda_from_stream(iter, lambda, exec_ctx)?;

        Ok(select_result.result)
    }

    fn apply_lambda_with_tetraplets(
        &self,
        lambda: &LambdaAST<'_>,
        exec_ctx: &ExecutionCtx<'_>,
    ) -> ExecutionResult<(Cow<'_, JValue>, SecurityTetraplet)> {
        let iter = self.iter()?.map(|v| v.result.deref());
        let select_result = select_by_lambda_from_stream(iter, lambda, exec_ctx)?;

        let tetraplet = match select_result.tetraplet_idx {
            Some(idx) => {
                let resolved_call = self
                    .iter()?
                    .nth(idx)
                    .expect(crate::execution_step::TETRAPLET_IDX_CORRECT);
                resolved_call.tetraplet.as_ref().clone()
            }
            None => SecurityTetraplet::new(exec_ctx.run_parameters.current_peer_id.to_string(), "", "", ""),
        };
        let tetraplet = populate_tetraplet_with_lambda(tetraplet, lambda);

        Ok((select_result.result, tetraplet))
    }

    fn as_jvalue(&self) -> Cow<'_, JValue> {
        let jvalue = self.stream.as_jvalue(self.generation).unwrap();
        Cow::Owned(jvalue)
    }

    fn into_jvalue(self: Box<Self>) -> JValue {
        self.stream.as_jvalue(self.generation).unwrap()
    }

    fn as_tetraplets(&self) -> RcSecurityTetraplets {
        self.stream
            .iter(self.generation)
            .unwrap()
            .map(|r| r.tetraplet.clone())
            .collect::<Vec<_>>()
    }
}

use crate::execution_step::boxed_value::stream::StreamIter;

#[allow(dead_code)]
impl<'stream> StreamJvaluableIngredients<'stream> {
    pub(crate) fn new(stream: &'stream Stream, generation: Generation) -> Self {
        Self { stream, generation }
    }

    pub(self) fn iter(&self) -> ExecutionResult<StreamIter<'_>> {
        use crate::execution_step::UncatchableError::StreamDontHaveSuchGeneration;

        match self.stream.iter(self.generation) {
            Some(iter) => Ok(iter),
            None => Err(StreamDontHaveSuchGeneration {
                stream: self.stream.clone(),
                generation: self.generation,
            }
            .into()),
        }
    }
}
