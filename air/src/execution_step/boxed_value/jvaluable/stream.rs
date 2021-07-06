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
use crate::execution_step::boxed_value::Stream;
use crate::JValue;
use crate::SecurityTetraplet;

use jsonpath_lib::select_with_iter;

use std::borrow::Cow;
use std::ops::Deref;

// TODO: this will be deleted soon, because it would be impossible to use streams without
// canonicalization as an arg of a call
impl JValuable for std::cell::Ref<'_, Stream> {
    fn apply_json_path(&self, json_path: &str) -> ExecutionResult<Vec<&JValue>> {
        let iter = self.iter().map(|v| v.result.deref());
        let (selected_values, _) = select_with_iter(iter, json_path)
            .map_err(|e| StreamJsonPathError(self.deref().clone(), json_path.to_string(), e))?;

        Ok(selected_values)
    }

    fn apply_json_path_with_tetraplets(
        &self,
        json_path: &str,
    ) -> ExecutionResult<(Vec<&JValue>, Vec<SecurityTetraplet>)> {
        let iter = self.iter().map(|v| v.result.deref());

        let (selected_values, tetraplet_indices) = select_with_iter(iter, json_path)
            .map_err(|e| StreamJsonPathError(self.deref().clone(), json_path.to_string(), e))?;

        let mut tetraplets = Vec::with_capacity(tetraplet_indices.len());

        for idx in tetraplet_indices.iter() {
            let resolved_call = self.iter().nth(*idx).unwrap();
            let tetraplet = SecurityTetraplet {
                triplet: resolved_call.triplet.clone(),
                json_path: json_path.to_string(),
            };
            tetraplets.push(tetraplet);
        }

        Ok((selected_values, tetraplets))
    }

    fn as_jvalue(&self) -> Cow<'_, JValue> {
        let jvalue = self.deref().clone().into_jvalue();
        Cow::Owned(jvalue)
    }

    fn into_jvalue(self: Box<Self>) -> JValue {
        self.clone().into_jvalue()
    }

    fn as_tetraplets(&self) -> Vec<SecurityTetraplet> {
        self.0
            .iter()
            .flat_map(|g| {
                g.iter().map(|r| SecurityTetraplet {
                    triplet: r.triplet.clone(),
                    json_path: String::new(),
                })
            })
            .collect::<Vec<_>>()
    }
}
