/*
 * Copyright 2023 Fluence Labs Limited
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
use crate::execution_step::boxed_value::Generation;
use crate::execution_step::boxed_value::StreamMap;
use crate::execution_step::ExecutionCtx;
use crate::execution_step::RcSecurityTetraplets;
use crate::JValue;
use crate::LambdaAST;
use crate::SecurityTetraplet;

use std::borrow::Cow;

#[derive(Debug)]
pub(crate) struct StreamMapJvaluableIngredients<'stream> {
    pub(crate) stream_map: &'stream StreamMap,
    pub(crate) generation: Generation,
}

// TODO: this will be deleted soon, because it would be impossible to use streams without
// canonicalization as an arg of a call
impl JValuable for StreamMapJvaluableIngredients<'_> {
    fn apply_lambda(&self, _lambda: &LambdaAST<'_>, _exec_ctx: &ExecutionCtx<'_>) -> ExecutionResult<Cow<'_, JValue>> {
        unimplemented!("No such method for StreamMap");
    }

    fn apply_lambda_with_tetraplets(
        &self,
        _lambda: &LambdaAST<'_>,
        _exec_ctx: &ExecutionCtx<'_>,
    ) -> ExecutionResult<(Cow<'_, JValue>, SecurityTetraplet)> {
        unimplemented!("No such method for StreamMap");
    }

    fn as_jvalue(&self) -> Cow<'_, JValue> {
        unimplemented!("No such method for StreamMap");
    }

    fn into_jvalue(self: Box<Self>) -> JValue {
        unimplemented!("No such method for StreamMap");
    }

    fn as_tetraplets(&self) -> RcSecurityTetraplets {
        unimplemented!("No such method for StreamMap");
    }
}

use crate::execution_step::boxed_value::StreamIter;

#[allow(dead_code)]
impl<'stream> StreamMapJvaluableIngredients<'stream> {
    pub(crate) fn new(stream_map: &'stream StreamMap, generation: Generation) -> Self {
        Self { stream_map, generation }
    }

    pub(self) fn iter(&self) -> ExecutionResult<StreamIter<'_>> {
        use crate::execution_step::UncatchableError::StreamMapDontHaveSuchGeneration;

        match self.stream_map.iter(self.generation) {
            Some(iter) => Ok(iter),
            None => Err(StreamMapDontHaveSuchGeneration {
                stream_map: self.stream_map.clone(),
                generation: self.generation,
            }
            .into()),
        }
    }
}
