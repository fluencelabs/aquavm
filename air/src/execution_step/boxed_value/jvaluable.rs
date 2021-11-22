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

mod cell_vec_resolved_call_result;
mod empty_stream;
mod iterable_item;
mod resolved_call_result;
mod stream;

use super::iterable::IterableItem;
use super::ExecutionError;
use super::ExecutionResult;
use super::ValueAggregate;
use crate::execution_step::lambda_applier::*;
use crate::execution_step::SecurityTetraplets;
use crate::execution_step::RSecurityTetraplet;
use crate::JValue;
use crate::LambdaAST;

pub(crate) use stream::StreamJvaluableIngredients;

use std::borrow::Cow;

/// Represent a value that could be transform to a JValue with or without tetraplets.
pub(crate) trait JValuable {
    /// Applies lambda to the internal value, produces JValue.
    fn apply_lambda(&self, lambda: &LambdaAST<'_>) -> ExecutionResult<&JValue>;

    /// Applies lambda to the internal value, produces JValue with tetraplet.
    fn apply_lambda_with_tetraplets(
        &self,
        lambda: &LambdaAST<'_>,
    ) -> ExecutionResult<(&JValue, RSecurityTetraplet)>;

    /// Return internal value as borrowed if it's possible, owned otherwise.
    fn as_jvalue(&self) -> Cow<'_, JValue>;

    /// Convert this boxed value to an owned JValue.
    fn into_jvalue(self: Box<Self>) -> JValue;

    /// Return tetraplets associating with internal value.
    fn as_tetraplets(&self) -> SecurityTetraplets;
}
