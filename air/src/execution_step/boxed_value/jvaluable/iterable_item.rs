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

use super::select;
use super::ExecutionResult;
use super::IterableItem;
use super::JValuable;
use super::LambdaAST;
use crate::execution_step::SecurityTetraplets;
use crate::execution_step::RSecurityTetraplet;
use crate::JValue;

use std::borrow::Cow;
use std::ops::Deref;

impl<'ctx> JValuable for IterableItem<'ctx> {
    fn apply_lambda(&self, lambda: &LambdaAST<'_>) -> ExecutionResult<&JValue> {
        use super::IterableItem::*;

        let jvalue = match self {
            RefRef((jvalue, ..)) => *jvalue,
            RefValue((jvalue, ..)) => jvalue,
            RcValue((jvalue, ..)) => jvalue.deref(),
        };

        let selected_value = select(jvalue, lambda.iter())?;
        Ok(selected_value)
    }

    fn apply_lambda_with_tetraplets(
        &self,
        lambda: &LambdaAST<'_>,
    ) -> ExecutionResult<(&JValue, RSecurityTetraplet)> {
        use super::IterableItem::*;

        let (jvalue, tetraplet) = match self {
            RefRef((jvalue, tetraplet, _)) => (*jvalue, *tetraplet),
            RefValue((jvalue, tetraplet, _)) => (*jvalue, tetraplet),
            RcValue((jvalue, tetraplet, _)) => (jvalue.deref(), tetraplet),
        };

        let selected_value = select(jvalue, lambda.iter())?;
        Ok((selected_value, tetraplet.clone()))
    }

    fn as_jvalue(&self) -> Cow<'_, JValue> {
        use super::IterableItem::*;

        match self {
            RefRef((jvalue, ..)) => Cow::Borrowed(jvalue),
            RefValue((jvalue, ..)) => Cow::Borrowed(jvalue),
            RcValue((jvalue, ..)) => Cow::Borrowed(jvalue.deref()),
        }
    }

    fn into_jvalue(self: Box<Self>) -> JValue {
        use super::IterableItem::*;

        match *self {
            RefRef((jvalue, ..)) => jvalue.deref().clone(),
            RefValue((jvalue, ..)) => jvalue.clone(),
            RcValue((jvalue, ..)) => jvalue.deref().clone(),
        }
    }

    fn as_tetraplets(&self) -> SecurityTetraplets {
        use super::IterableItem::*;

        // these clones are needed because rust-sdk allows passing arguments only by value
        match self {
            RefRef((_, tetraplet, _)) => {
                let tetraplet = tetraplet.deref().clone();
                vec![tetraplet]
            }
            RefValue((_, tetraplet, _)) => vec![tetraplet.clone()],
            RcValue((_, tetraplet, _)) => vec![tetraplet.clone()],
        }
    }
}
