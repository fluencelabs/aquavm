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

use crate::air::fold::IterableItemType;
use crate::JValue;
use crate::ResolvedCallResult;
use crate::Result;
use crate::SecurityTetraplet;

use jsonpath_lib::select;
use jsonpath_lib::select_with_iter;

use std::borrow::Cow;
use std::ops::Deref;

/// Represent a value that could be transform to a JValue with or without tetraplets.
pub(crate) trait JValuable {
    /// Applies json path to the internal value, produces JValue.
    fn apply_json_path(&self, json_path: &str) -> Result<Vec<&JValue>>;

    /// Applies json path to the internal value, produces JValue with tetraplet.
    fn apply_json_path_with_tetraplets(&self, json_path: &str) -> Result<(Vec<&JValue>, Vec<SecurityTetraplet>)>;

    /// Return internal value as borrowed if it's possible, owned otherwise.
    fn as_jvalue(&self) -> Cow<'_, JValue>;

    /// Convert this boxed value to an owned JValue.
    fn into_jvalue(self: Box<Self>) -> JValue;

    /// Return tetraplets associating with internal value.
    fn as_tetraplets(&self) -> Vec<SecurityTetraplet>;
}

impl<'ctx> JValuable for IterableItemType<'ctx> {
    fn apply_json_path(&self, json_path: &str) -> Result<Vec<&JValue>> {
        use crate::AquamarineError::JValueJsonPathError as JsonPathError;
        use IterableItemType::*;

        let jvalue = match self {
            RefRef((jvalue, _)) => *jvalue,
            RefValue((jvalue, _)) => jvalue,
            RcValue((jvalue, _)) => jvalue.deref(),
        };

        let selected_jvalues =
            select(jvalue, json_path).map_err(|e| JsonPathError(jvalue.clone(), String::from(json_path), e))?;
        Ok(selected_jvalues)
    }

    fn apply_json_path_with_tetraplets(&self, json_path: &str) -> Result<(Vec<&JValue>, Vec<SecurityTetraplet>)> {
        use super::fold::IterableItemType::*;
        use crate::AquamarineError::JValueJsonPathError as JsonPathError;

        let (jvalue, tetraplet) = match self {
            RefRef((jvalue, tetraplet)) => (*jvalue, *tetraplet),
            RefValue((jvalue, tetraplet)) => (*jvalue, tetraplet),
            RcValue((jvalue, tetraplet)) => (jvalue.deref(), tetraplet),
        };

        let selected_jvalues =
            select(jvalue, json_path).map_err(|e| JsonPathError(jvalue.clone(), String::from(json_path), e))?;
        Ok((selected_jvalues, vec![tetraplet.clone()]))
    }

    fn as_jvalue(&self) -> Cow<'_, JValue> {
        use IterableItemType::*;

        match self {
            RefRef((jvalue, _)) => Cow::Borrowed(jvalue),
            RefValue((jvalue, _)) => Cow::Borrowed(jvalue),
            RcValue((jvalue, _)) => Cow::Borrowed(jvalue.deref()),
        }
    }

    fn into_jvalue(self: Box<Self>) -> JValue {
        use IterableItemType::*;

        match *self {
            RefRef((jvalue, _)) => jvalue.deref().clone(),
            RefValue((jvalue, _)) => jvalue.clone(),
            RcValue((jvalue, _)) => jvalue.deref().clone(),
        }
    }

    fn as_tetraplets(&self) -> Vec<SecurityTetraplet> {
        use IterableItemType::*;

        // these clones are needed because rust-sdk allows passing arguments only by value
        match self {
            RefRef((_, tetraplet)) => {
                let tetraplet = tetraplet.deref().clone();
                vec![tetraplet]
            }
            RefValue((_, tetraplet)) => vec![(*tetraplet).clone()],
            RcValue((_, tetraplet)) => vec![(*tetraplet).clone()],
        }
    }
}

impl JValuable for ResolvedCallResult {
    fn apply_json_path(&self, json_path: &str) -> Result<Vec<&JValue>> {
        use crate::AquamarineError::JValueJsonPathError as JsonPathError;

        let selected_jvalues = select(&self.result, json_path)
            .map_err(|e| JsonPathError(self.result.deref().clone(), String::from(json_path), e))?;
        Ok(selected_jvalues)
    }

    fn apply_json_path_with_tetraplets(&self, json_path: &str) -> Result<(Vec<&JValue>, Vec<SecurityTetraplet>)> {
        use crate::AquamarineError::JValueJsonPathError as JsonPathError;

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

impl JValuable for std::cell::Ref<'_, Vec<ResolvedCallResult>> {
    fn apply_json_path(&self, json_path: &str) -> Result<Vec<&JValue>> {
        let (selected_values, _) = select_with_iter(self.iter().map(|r| r.result.deref()), json_path).unwrap();

        Ok(selected_values)
    }

    fn apply_json_path_with_tetraplets(&self, json_path: &str) -> Result<(Vec<&JValue>, Vec<SecurityTetraplet>)> {
        let (selected_values, tetraplet_indices) =
            select_with_iter(self.iter().map(|r| r.result.deref()), json_path).unwrap();
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
