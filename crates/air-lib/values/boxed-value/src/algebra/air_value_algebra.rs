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

use crate::BoxedValue;
use crate::RcBoxedValue;
use crate::RcSecurityTetraplet;
use crate::RcSecurityTetraplets;

use air_lambda_ast::AIRLambda;

/// Represent a value that could be transform to a JValue with or without tetraplets.
pub trait AIRValueAlgebra {
    type Error;

    /// Applies lambda to the internal value, produces JValue.
    fn apply_lambda<'value>(
        &'value self,
        lambda: &AIRLambda<'_>,
    ) -> Result<&'value dyn BoxedValue, Self::Error>;

    /// Applies lambda to the internal value, produces JValue with tetraplet.
    fn apply_lambda_with_tetraplets<'value>(
        &'value self,
        lambda: &AIRLambda<'_>,
    ) -> Result<ValueWithTetraplet<'value, 'value>, Self::Error>;

    /// Return internal value as borrowed if it's possible, owned otherwise.
    fn as_value(&self) -> &RcBoxedValue;

    /// Return tetraplets associating with an internal value.
    fn as_tetraplets(&self) -> RcSecurityTetraplets;
}

pub struct ValueWithTetraplet<'value, 'tetraplet> {
    pub value: &'value dyn BoxedValue,
    pub tetraplet: &'tetraplet RcSecurityTetraplet,
}
