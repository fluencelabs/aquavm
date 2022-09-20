/*
 * Copyright 2022 Fluence Labs Limited
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

use crate::Functor;
use crate::LambdaAST;
use crate::ValueAccessor;

pub use non_empty_vec::EmptyError;
use non_empty_vec::NonEmpty;

use std::convert::TryFrom;

impl<'input> LambdaAST<'input> {
    pub fn try_from_accessors(accessors: Vec<ValueAccessor<'input>>) -> Result<Self, EmptyError> {
        let value_path = NonEmpty::try_from(accessors)?;
        let lambda_ast = Self::ValuePath(value_path);

        Ok(lambda_ast)
    }

    pub fn from_functor(functor: Functor) -> Self {
        Self::Functor(functor)
    }
}
