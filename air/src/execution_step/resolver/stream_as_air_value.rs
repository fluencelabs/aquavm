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

use crate::execution_step::ExecutionResult;

use air_lambda_ast::{format_lambda, AIRLambda};
use air_values::boxed_value::AIRValueAlgebra;
use air_values::stream::Generation;
use air_values::stream::Stream;
use air_values::stream::StreamIter;

use crate::execution_step::lambda_applier::select_from_stream;
use crate::execution_step::RcSecurityTetraplets;
use crate::SecurityTetraplet;
use crate::{ExecutionError, JValue};

use air_values::boxed_value::BoxedValue;
use std::borrow::Cow;
use std::ops::Deref;

#[derive(Debug)]
pub(crate) struct StreamValueAlgebraIngredients<'stream> {
    pub(crate) stream: &'stream Stream,
    pub(crate) generation: Generation,
}

// TODO: this will be deleted soon, because it would be impossible to use streams without
// canonicalization as an arg of a call
impl AIRValueAlgebra for StreamValueAlgebraIngredients<'_> {
    type Error = ExecutionError<JValue>;

    fn apply_lambda<'value>(&'value self, lambda: &AIRLambda<'_>) -> ExecutionResult<&'value dyn BoxedValue> {
        let iter = self.iter()?.map(|v| v.value.deref());
        let select_result = select_from_stream(iter, lambda)?;

        Ok(select_result.result)
    }

    fn apply_lambda_with_tetraplets<'value>(
        &'value self,
        lambda: &AIRLambda<'_>,
    ) -> ExecutionResult<(&'value dyn BoxedValue, SecurityTetraplet)> {
        let iter = self.iter()?.map(|v| v.value.deref());
        let select_result = select_from_stream(iter, lambda)?;

        // unwrap is safe here because each value has a tetraplet and a lambda always returns a valid index
        let resolved_call = self.iter()?.nth(select_result.tetraplet_idx).unwrap();
        let mut tetraplet = resolved_call.tetraplet.as_ref().clone();
        tetraplet.add_lambda(&format_lambda(lambda));

        Ok((select_result.result, tetraplet))
    }

    fn as_value(&self) -> &dyn BoxedValue {
        self.stream.as_value(self.generation).unwrap()
    }

    fn as_tetraplets(&self) -> RcSecurityTetraplets {
        self.stream
            .iter(self.generation)
            .unwrap()
            .map(|r| r.tetraplet.clone())
            .collect::<Vec<_>>()
    }
}

impl<'stream> StreamValueAlgebraIngredients<'stream> {
    pub(crate) fn new(stream: &'stream Stream, generation: Generation) -> Self {
        Self { stream, generation }
    }

    pub(self) fn iter(&self) -> ExecutionResult<StreamIter<'_>> {
        use crate::execution_step::CatchableError::StreamDontHaveSuchGeneration;

        match self.stream.iter(self.generation) {
            Some(iter) => Ok(iter),
            None => {
                let generation = match self.generation {
                    Generation::Nth(generation) => generation,
                    Generation::Last => unreachable!(),
                };

                Err(StreamDontHaveSuchGeneration(self.stream.clone(), generation as usize).into())
            }
        }
    }
}
