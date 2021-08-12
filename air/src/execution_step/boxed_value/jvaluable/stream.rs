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

use super::ExecutionError::StreamJsonPathError;
use super::ExecutionResult;
use super::JValuable;
use crate::exec_err;
use crate::execution_step::boxed_value::Generation;
use crate::execution_step::boxed_value::Stream;
use crate::execution_step::SecurityTetraplets;
use crate::JValue;

use jsonpath_lib::select_with_iter;

use std::borrow::Cow;
use std::ops::Deref;

pub(crate) struct StreamJvaluableIngredients<'stream> {
    pub(crate) stream: std::cell::Ref<'stream, Stream>,
    pub(crate) generation: Generation,
}

// TODO: this will be deleted soon, because it would be impossible to use streams without
// canonicalization as an arg of a call
impl JValuable for StreamJvaluableIngredients<'_> {
    fn apply_json_path(&self, json_path: &str) -> ExecutionResult<Vec<&JValue>> {
        let iter = self.iter()?.map(|v| v.result.deref());
        let (selected_values, _) = select_with_iter(iter, json_path)
            .map_err(|e| StreamJsonPathError(self.stream.deref().clone(), json_path.to_string(), e))?;

        Ok(selected_values)
    }

    fn apply_json_path_with_tetraplets(&self, json_path: &str) -> ExecutionResult<(Vec<&JValue>, SecurityTetraplets)> {
        let iter = self.iter()?.map(|v| v.result.deref());

        let (selected_values, tetraplet_indices) = select_with_iter(iter, json_path)
            .map_err(|e| StreamJsonPathError(self.stream.deref().clone(), json_path.to_string(), e))?;

        let mut tetraplets = Vec::with_capacity(tetraplet_indices.len());

        for idx in tetraplet_indices.iter() {
            let resolved_call = self.iter()?.nth(*idx).unwrap();
            let tetraplet = resolved_call.tetraplet.clone();
            tetraplet.borrow_mut().add_json_path(json_path);
            tetraplets.push(tetraplet);
        }

        Ok((selected_values, tetraplets))
    }

    fn as_jvalue(&self) -> Cow<'_, JValue> {
        let jvalue = self.stream.deref().clone().as_jvalue(self.generation).unwrap();
        Cow::Owned(jvalue)
    }

    fn into_jvalue(self: Box<Self>) -> JValue {
        self.stream.as_jvalue(self.generation).unwrap()
    }

    fn as_tetraplets(&self) -> SecurityTetraplets {
        self.stream
            .iter(self.generation)
            .unwrap()
            .map(|r| r.tetraplet.clone())
            .collect::<Vec<_>>()
    }
}

use crate::execution_step::boxed_value::StreamIter;

impl<'stream> StreamJvaluableIngredients<'stream> {
    pub(crate) fn new(stream: std::cell::Ref<'stream, Stream>, generation: Generation) -> Self {
        Self { stream, generation }
    }

    pub(self) fn iter(&self) -> ExecutionResult<StreamIter<'_>> {
        use super::ExecutionError::StreamDontHaveSuchGeneration;

        match self.stream.iter(self.generation) {
            Some(iter) => Ok(iter),
            None => {
                let generation = match self.generation {
                    Generation::Nth(generation) => generation,
                    Generation::Last => unreachable!(),
                };

                exec_err!(StreamDontHaveSuchGeneration(
                    self.stream.deref().clone(),
                    generation as usize
                ))
            }
        }
    }
}
