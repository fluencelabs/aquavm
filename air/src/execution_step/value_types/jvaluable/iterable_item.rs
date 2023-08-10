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

use super::select_by_lambda_from_scalar;
use super::ExecutionResult;
use super::IterableItem;
use super::JValuable;
use super::LambdaAST;
use crate::execution_step::value_types::populate_tetraplet_with_lambda;
use crate::execution_step::ExecutionCtx;
use crate::execution_step::RcSecurityTetraplets;
use crate::JValue;
use crate::SecurityTetraplet;

use air_interpreter_data::Provenance;

use std::borrow::Cow;
use std::ops::Deref;

impl<'ctx> JValuable for IterableItem<'ctx> {
    fn apply_lambda(&self, lambda: &LambdaAST<'_>, exec_ctx: &ExecutionCtx<'_>) -> ExecutionResult<Cow<'_, JValue>> {
        use super::IterableItem::*;

        let jvalue = match self {
            RefValue((jvalue, ..)) => jvalue,
            RcValue((jvalue, ..)) => jvalue.deref(),
        };

        let selected_value = select_by_lambda_from_scalar(jvalue, lambda, exec_ctx)?;
        Ok(selected_value)
    }

    fn apply_lambda_with_tetraplets(
        &self,
        lambda: &LambdaAST<'_>,
        exec_ctx: &ExecutionCtx<'_>,
        _root_provenance: &Provenance,
    ) -> ExecutionResult<(Cow<'_, JValue>, SecurityTetraplet, Provenance)> {
        use super::IterableItem::*;

        let (jvalue, tetraplet, provenance) = match self {
            RefValue((jvalue, tetraplet, _, provenance)) => (*jvalue, tetraplet, provenance),
            RcValue((jvalue, tetraplet, _, provenance)) => (jvalue.deref(), tetraplet, provenance),
        };

        let selected_value = select_by_lambda_from_scalar(jvalue, lambda, exec_ctx)?;
        let tetraplet = populate_tetraplet_with_lambda(tetraplet.as_ref().clone(), lambda);

        Ok((selected_value, tetraplet, provenance.clone()))
    }

    fn as_jvalue(&self) -> Cow<'_, JValue> {
        use super::IterableItem::*;

        match self {
            RefValue((jvalue, ..)) => Cow::Borrowed(jvalue),
            RcValue((jvalue, ..)) => Cow::Borrowed(jvalue.deref()),
        }
    }

    fn into_jvalue(self: Box<Self>) -> JValue {
        use super::IterableItem::*;

        match *self {
            RefValue((jvalue, ..)) => jvalue.clone(),
            RcValue((jvalue, ..)) => jvalue.deref().clone(),
        }
    }

    fn as_tetraplets(&self) -> RcSecurityTetraplets {
        use super::IterableItem::*;

        // these clones are needed because rust-sdk allows passing arguments only by value
        match self {
            RefValue((_, tetraplet, _, _)) => vec![tetraplet.clone()],
            RcValue((_, tetraplet, _, _)) => vec![tetraplet.clone()],
        }
    }
}
