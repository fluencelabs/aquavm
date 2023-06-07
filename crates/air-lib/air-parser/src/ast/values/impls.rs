/*
 * Copyright 2021 Fluence Labs Limited
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
use air_lambda_parser::LambdaAST;

impl<'i> ScalarWithLambda<'i> {
    pub fn new(name: &'i str, lambda: LambdaAST<'i>, position: AirPos) -> Self {
        Self {
            name,
            lambda,
            position,
        }
    }

    #[cfg(test)]
    pub(crate) fn from_raw_lambda(
        name: &'i str,
        lambda: Vec<air_lambda_parser::ValueAccessor<'i>>,
        position: AirPos,
    ) -> Self {
        let lambda = LambdaAST::try_from_accessors(lambda).unwrap();
        Self {
            name,
            lambda,
            position,
        }
    }
}

impl<'i> CanonStream<'i> {
    pub fn new(name: &'i str, position: AirPos) -> Self {
        Self { name, position }
    }
}

impl<'i> CanonStreamWithLambda<'i> {
    pub fn new(name: &'i str, lambda: LambdaAST<'i>, position: AirPos) -> Self {
        Self {
            name,
            lambda,
            position,
        }
    }
}

impl<'i> Scalar<'i> {
    pub fn new(name: &'i str, position: AirPos) -> Self {
        Self { name, position }
    }
}

impl<'i> Stream<'i> {
    pub fn new(name: &'i str, position: AirPos) -> Self {
        Self { name, position }
    }
}

impl<'i> ImmutableVariable<'i> {
    pub fn scalar(name: &'i str, position: AirPos) -> Self {
        Self::Scalar(Scalar::new(name, position))
    }

    pub fn canon_stream(name: &'i str, position: AirPos) -> Self {
        Self::CanonStream(CanonStream::new(name, position))
    }

    pub fn name(&self) -> &'i str {
        match self {
            ImmutableVariable::Scalar(scalar) => scalar.name,
            ImmutableVariable::CanonStream(stream) => stream.name,
        }
    }
}

impl<'i> ImmutableVariableWithLambda<'i> {
    pub fn scalar(name: &'i str, lambda: LambdaAST<'i>, position: AirPos) -> Self {
        Self::Scalar(ScalarWithLambda::new(name, lambda, position))
    }

    pub fn canon_stream(name: &'i str, lambda: LambdaAST<'i>, position: AirPos) -> Self {
        Self::CanonStream(CanonStreamWithLambda::new(name, lambda, position))
    }

    pub fn name(&self) -> &'i str {
        match self {
            ImmutableVariableWithLambda::Scalar(scalar) => scalar.name,
            ImmutableVariableWithLambda::CanonStream(canon_stream) => canon_stream.name,
        }
    }

    pub fn lambda(&self) -> &LambdaAST<'i> {
        match self {
            ImmutableVariableWithLambda::Scalar(scalar) => &scalar.lambda,
            ImmutableVariableWithLambda::CanonStream(canon_stream) => &canon_stream.lambda,
        }
    }

    #[cfg(test)]
    pub(crate) fn from_raw_value_path(
        name: &'i str,
        lambda: Vec<air_lambda_parser::ValueAccessor<'i>>,
        position: AirPos,
    ) -> Self {
        let scalar = ScalarWithLambda::from_raw_lambda(name, lambda, position);
        Self::Scalar(scalar)
    }
}

impl<'i> StreamMap<'i> {
    pub fn new(name: &'i str, position: AirPos) -> Self {
        Self { name, position }
    }
}
