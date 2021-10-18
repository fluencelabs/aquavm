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

use super::*;
use air_lambda_parser::ValueAccessor;

impl<'i> Ap<'i> {
    pub fn new(argument: ApArgument<'i>, result: AstVariable<'i>) -> Self {
        Self { argument, result }
    }
}

impl<'i> VariableWithLambda<'i> {
    pub fn new(variable: AstVariable<'i>, lambda: LambdaAST<'i>) -> Self {
        Self { variable, lambda }
    }

    // This function is unsafe and lambda must be non-empty, although it's used only for tests
    pub fn from_raw_algebras(variable: AstVariable<'i>, lambda: Vec<ValueAccessor<'i>>) -> Self {
        let lambda = unsafe { LambdaAST::new_unchecked(lambda) };
        Self { variable, lambda }
    }
}

impl<'i> IterableScalarValue<'i> {
    // This function is unsafe and lambda must be non-empty, although it's used only for tests
    pub fn new_vl(scalar_name: &'i str, lambda: Vec<ValueAccessor<'i>>) -> Self {
        let lambda = unsafe { LambdaAST::new_unchecked(lambda) };
        Self::VariableWithLambda {
            scalar_name,
            lambda,
        }
    }
}
