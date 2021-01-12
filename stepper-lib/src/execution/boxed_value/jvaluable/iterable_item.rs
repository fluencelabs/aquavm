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
use crate::contexts::execution::ResolvedCallResult;
use crate::JValue;
use crate::SecurityTetraplet;

use jsonpath::select;

impl<'ctx> JValuable for IterableItem<'ctx> {
    fn apply_json_path(&self, json_path: &str) -> ExecutionResult<Vec<&JValue>> {
        use crate::AquamarineError::JValueJsonPathError as JsonPathError;
        use IterableItem::*;

        let jvalue = match self {
            RefRef((jvalue, _)) => *jvalue,
            RefValue((jvalue, _)) => jvalue,
            RcValue((jvalue, _)) => jvalue.deref(),
        };

        let selected_jvalues =
            select(jvalue, json_path).map_err(|e| JsonPathError(jvalue.clone(), String::from(json_path), e))?;
        Ok(selected_jvalues)
    }

    fn apply_json_path_with_tetraplets(
        &self,
        json_path: &str,
    ) -> ExecutionResult<(Vec<&JValue>, Vec<SecurityTetraplet>)> {
        use super::fold::IterableItem::*;
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
        use IterableItem::*;

        match self {
            RefRef((jvalue, _)) => Cow::Borrowed(jvalue),
            RefValue((jvalue, _)) => Cow::Borrowed(jvalue),
            RcValue((jvalue, _)) => Cow::Borrowed(jvalue.deref()),
        }
    }

    fn into_jvalue(self: Box<Self>) -> JValue {
        use IterableItem::*;

        match *self {
            RefRef((jvalue, _)) => jvalue.deref().clone(),
            RefValue((jvalue, _)) => jvalue.clone(),
            RcValue((jvalue, _)) => jvalue.deref().clone(),
        }
    }

    fn as_tetraplets(&self) -> Vec<SecurityTetraplet> {
        use IterableItem::*;

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
