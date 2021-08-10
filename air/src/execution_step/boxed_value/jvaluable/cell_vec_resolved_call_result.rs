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

use super::ExecutionError::GenerationStreamJsonPathError;
use super::ExecutionResult;
use super::JValuable;
use super::ResolvedCallResult;
use crate::JValue;
use crate::SecurityTetraplet;

use jsonpath_lib::select_with_iter;

use std::borrow::Cow;
use std::ops::Deref;

impl JValuable for std::cell::Ref<'_, Vec<ResolvedCallResult>> {
    fn apply_json_path(&self, json_path: &str) -> ExecutionResult<Vec<&JValue>> {
        let acc_iter = self.iter().map(|r| r.result.deref());
        let (selected_values, _) = select_with_iter(acc_iter, json_path).map_err(|e| {
            GenerationStreamJsonPathError(self.iter().cloned().collect::<Vec<_>>(), json_path.to_string(), e)
        })?;

        Ok(selected_values)
    }

    fn apply_json_path_with_tetraplets(
        &self,
        json_path: &str,
    ) -> ExecutionResult<(Vec<&JValue>, Vec<SecurityTetraplet>)> {
        let acc_iter = self.iter().map(|r| r.result.deref());

        let (selected_values, tetraplet_indices) = select_with_iter(acc_iter, json_path).map_err(|e| {
            GenerationStreamJsonPathError(self.iter().cloned().collect::<Vec<_>>(), json_path.to_string(), e)
        })?;

        let tetraplets = tetraplet_indices
            .into_iter()
            .map(|id| SecurityTetraplet {
                triplet: self[id].triplet.clone(),
                json_path: json_path.to_string(),
            })
            .collect::<Vec<_>>();

        Ok((selected_values, tetraplets))
    }

    fn as_jvalue(&self) -> Cow<'_, JValue> {
        let jvalue_array = self.iter().map(|r| r.result.deref().clone()).collect::<Vec<_>>();
        Cow::Owned(JValue::Array(jvalue_array))
    }

    fn into_jvalue(self: Box<Self>) -> JValue {
        let jvalue_array = self.iter().map(|r| r.result.deref().clone()).collect::<Vec<_>>();
        JValue::Array(jvalue_array)
    }

    fn as_tetraplets(&self) -> Vec<SecurityTetraplet> {
        self.iter()
            .map(|r| SecurityTetraplet {
                triplet: r.triplet.clone(),
                json_path: String::new(),
            })
            .collect::<Vec<_>>()
    }
}
