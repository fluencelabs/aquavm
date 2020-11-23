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

use crate::ExecutedCallResult;
use crate::JValue;
use crate::Result;
use crate::SecurityTetraplet;

use std::borrow::Cow;
use std::rc::Rc;

pub(crate) trait JValuableResult {
    fn apply_json_path(&self, json_path: &str) -> Result<Vec<&JValue>>;

    fn apply_json_path_with_tetraplets(&self, json_path: &str) -> Result<(Vec<&JValue>, Vec<SecurityTetraplet>)>;

    fn as_jvalue(&self) -> Cow<'_, JValue>;

    fn as_tetraplets(&self) -> Vec<SecurityTetraplet>;
}

pub(crate) struct JValuableResultRef<'jvalue>(Vec<&'jvalue JValue>, Vec<usize>);

impl JValuableResult for (&JValue, &SecurityTetraplet) {
    fn apply_json_path(&self, json_path: &str) -> Result<Vec<&JValue>> {
        use jsonpath_lib::select;
        use AquamarineError::VariableNotInJsonPath as JsonPathError;

        let selected_jvalues =
            select(&self.0, json_path).map_err(|e| JsonPathError(jvalue.clone(), String::from(json_path), e))?;
        Ok(selected_jvalues)
    }

    fn apply_json_path_with_tetraplets(&self, json_path: &str) -> Result<(Vec<&JValue>, Vec<SecurityTetraplet>)> {
        use jsonpath_lib::select;
        use AquamarineError::VariableNotInJsonPath as JsonPathError;

        let selected_jvalues =
            select(&self.0, json_path).map_err(|e| JsonPathError(jvalue.clone(), String::from(json_path), e))?;
        Ok((selected_jvalues, vec![self.1.clone()]))
    }

    fn as_jvalue(&self) -> Cow<'_, JValue> {
        // this clone is needed because of rust-sdk allows passing arguments only by value
        Cow::Borrowed(&self.0)
    }

    fn as_tetraplets(&self) -> Vec<SecurityTetraplet> {
        // this clone is needed because of rust-sdk allows passing arguments only by value
        vec![self.1.clone()]
    }
}

impl JValuableResult for Rc<ExecutedCallResult> {
    fn apply_json_path(&self, json_path: &str) -> Result<Vec<&JValue>> {
        use jsonpath_lib::select;
        use AquamarineError::VariableNotInJsonPath as JsonPathError;

        let selected_jvalues =
            select(&self.result, json_path).map_err(|e| JsonPathError(jvalue.clone(), String::from(json_path), e))?;
        Ok(selected_jvalues)
    }

    fn apply_json_path_with_tetraplets(&self, json_path: &str) -> Result<(Vec<&JValue>, Vec<SecurityTetraplet>)> {
        use jsonpath_lib::select;
        use AquamarineError::VariableNotInJsonPath as JsonPathError;

        let selected_jvalues =
            select(&self.result, json_path).map_err(|e| JsonPathError(jvalue.clone(), String::from(json_path), e))?;
        Ok((selected_jvalues, vec![self.tetraplet.clone()]))
    }

    fn as_jvalue(&self) -> Cow<'_, JValue> {
        // this clone is needed because of rust-sdk allows passing arguments only by value
        Cow::Borrowed(&self.result)
    }

    fn as_tetraplets(&self) -> Vec<SecurityTetraplet> {
        // this clone is needed because of rust-sdk allows passing arguments only by value
        vec![self.tetraplet.clone()]
    }
}

impl JValuableResult for Vec<Rc<ExecutedCallResult>> {
    fn apply_json_path(&self, json_path: &str) -> Result<Vec<&JValue>> {
        use jsonpath_lib::select_with_iter;

        let (selected_values, _) = select_with_iter(self.iter().map(|r| r.result.as_ref()), json_path)?;

        Ok(selected_values)
    }

    fn apply_json_path_with_tetraplets(&self, json_path: &str) -> Result<(Vec<&JValue>, Vec<SecurityTetraplet>)> {
        use jsonpath_lib::select_with_iter;

        let (selected_values, tetraplet_indices) = select_with_iter(self.iter().map(|r| r.result.as_ref()), json_path)?;
        let tetraplets = tetraplet_indices
            .into_iter()
            .map(|id| self[id].tetraplet)
            // this cloned is needed because of rust-sdk allows passing arguments only by value
            .cloned()
            .collect::<Vec<_>>();

        Ok((selected_values, tetraplets))
    }

    fn as_jvalue(&self) -> Cow<'_, JValue> {
        // this cloned is needed because of rust-sdk allows passing arguments only by value
        let jvalue_array = self.iter().map(|r| r.result.as_ref()).cloned().collect::<Vec<_>>();
        Cow::Owned(JValue::Array(jvalue_array))
    }

    fn as_tetraplets(&self) -> Vec<SecurityTetraplet> {
        self.iter()
            .map(|r| r.tetraplet.as_ref())
            // this cloned is needed because of rust-sdk allows passing arguments only by value
            .cloned()
            .collect::<Vec<_>>()
    }
}
