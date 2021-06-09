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
use crate::contexts::execution::ResolvedCallResult;
use crate::JValue;
use crate::SecurityTetraplet;

use jsonpath_lib::select;

use std::borrow::Cow;
use std::ops::Deref;

impl JValuable for ResolvedCallResult {
    fn apply_json_path(&self, json_path: &str) -> ExecutionResult<Vec<&JValue>> {
        use super::ExecutionError::JValueJsonPathError as JsonPathError;

        let selected_jvalues = select(&self.result, json_path)
            .map_err(|e| JsonPathError(self.result.deref().clone(), String::from(json_path), e))?;
        Ok(selected_jvalues)
    }

    fn apply_json_path_with_tetraplets(
        &self,
        json_path: &str,
    ) -> ExecutionResult<(Vec<&JValue>, Vec<SecurityTetraplet>)> {
        use super::ExecutionError::JValueJsonPathError as JsonPathError;

        is_json_path_allowed(&self.result)?;
        let selected_jvalues = select(&self.result, json_path)
            .map_err(|e| JsonPathError(self.result.deref().clone(), String::from(json_path), e))?;

        let tetraplet = SecurityTetraplet {
            triplet: self.triplet.clone(),
            json_path: json_path.to_string(),
        };

        Ok((selected_jvalues, vec![tetraplet]))
    }

    fn as_jvalue(&self) -> Cow<'_, JValue> {
        Cow::Borrowed(&self.result)
    }

    fn into_jvalue(self: Box<Self>) -> JValue {
        self.result.deref().clone()
    }

    fn as_tetraplets(&self) -> Vec<SecurityTetraplet> {
        let tetraplet = SecurityTetraplet {
            triplet: self.triplet.clone(),
            json_path: String::new(),
        };

        vec![tetraplet]
    }
}

fn is_json_path_allowed(value: &JValue) -> ExecutionResult<()> {
    use super::ExecutionError;
    use crate::exec_err;

    match value {
        JValue::Array(_) => return Ok(()),
        JValue::Object(_) => return Ok(()),
        value => exec_err!(ExecutionError::JsonPathVariableTypeError(value.clone())),
    }
}
