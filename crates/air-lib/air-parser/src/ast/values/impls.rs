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
use air_lambda_parser::ValueAccessor;
use air_parser_utils::Identifier;

impl<'i> ScalarWithLambda<'i> {
    pub fn new(name: Identifier<'i>, lambda: Option<LambdaAST<'i>>, position: usize) -> Self {
        Self {
            name,
            lambda,
            position,
        }
    }

    // it's unsafe method that should be used only for tests
    pub(crate) fn from_raw_lambda(
        name: Identifier<'i>,
        lambda: Vec<ValueAccessor<'i>>,
        position: usize,
    ) -> Self {
        let lambda = unsafe { LambdaAST::new_unchecked(lambda) };
        Self {
            name,
            lambda: Some(lambda),
            position,
        }
    }
}

impl<'i> StreamWithLambda<'i> {
    pub fn new(name: Identifier<'i>, lambda: Option<LambdaAST<'i>>, position: usize) -> Self {
        Self {
            name,
            lambda,
            position,
        }
    }

    // it's unsafe method that should be used only for tests
    #[allow(dead_code)]
    pub(crate) fn from_raw_lambda(
        name: Identifier<'i>,
        lambda: Vec<ValueAccessor<'i>>,
        position: usize,
    ) -> Self {
        let lambda = unsafe { LambdaAST::new_unchecked(lambda) };
        Self {
            name,
            lambda: Some(lambda),
            position,
        }
    }
}

impl<'i> CanonStream<'i> {
    pub fn new(name: Identifier<'i>, position: usize) -> Self {
        Self { name, position }
    }
}

impl<'i> CanonStreamWithLambda<'i> {
    pub fn new(name: Identifier<'i>, lambda: Option<LambdaAST<'i>>, position: usize) -> Self {
        Self {
            name,
            lambda,
            position,
        }
    }
}

impl<'i> Scalar<'i> {
    pub fn new(name: Identifier<'i>, position: usize) -> Self {
        Self { name, position }
    }
}

impl<'i> Stream<'i> {
    pub fn new(name: Identifier<'i>, position: usize) -> Self {
        Self { name, position }
    }
}

impl<'i> Variable<'i> {
    pub fn scalar(name: Identifier<'i>, position: usize) -> Self {
        Self::Scalar(Scalar::new(name, position))
    }

    pub fn stream(name: Identifier<'i>, position: usize) -> Self {
        Self::Stream(Stream::new(name, position))
    }

    pub fn name(&self) -> Identifier<'i> {
        match self {
            Variable::Scalar(scalar) => scalar.name,
            Variable::Stream(stream) => stream.name,
            Variable::CanonStream(stream) => stream.name,
        }
    }
}

impl<'i> VariableWithLambda<'i> {
    pub fn scalar(name: Identifier<'i>, position: usize) -> Self {
        Self::Scalar(ScalarWithLambda::new(name, None, position))
    }

    pub fn scalar_wl(name: Identifier<'i>, lambda: LambdaAST<'i>, position: usize) -> Self {
        Self::Scalar(ScalarWithLambda::new(name, Some(lambda), position))
    }

    pub fn stream(name: Identifier<'i>, position: usize) -> Self {
        Self::Stream(StreamWithLambda::new(name, None, position))
    }

    pub fn stream_wl(name: Identifier<'i>, lambda: LambdaAST<'i>, position: usize) -> Self {
        Self::Stream(StreamWithLambda::new(name, Some(lambda), position))
    }

    pub fn canon_stream(name: Identifier<'i>, position: usize) -> Self {
        Self::CanonStream(CanonStreamWithLambda::new(name, None, position))
    }

    pub fn canon_stream_wl(name: Identifier<'i>, lambda: LambdaAST<'i>, position: usize) -> Self {
        Self::CanonStream(CanonStreamWithLambda::new(name, Some(lambda), position))
    }

    pub fn name(&self) -> Identifier<'i> {
        match self {
            VariableWithLambda::Scalar(scalar) => scalar.name,
            VariableWithLambda::Stream(stream) => stream.name,
            VariableWithLambda::CanonStream(canon_stream) => canon_stream.name,
        }
    }

    pub fn lambda(&self) -> &Option<LambdaAST<'i>> {
        match self {
            VariableWithLambda::Scalar(scalar) => &scalar.lambda,
            VariableWithLambda::Stream(stream) => &stream.lambda,
            VariableWithLambda::CanonStream(canon_stream) => &canon_stream.lambda,
        }
    }

    // This function is unsafe and lambda must be non-empty, although it's used only for tests
    #[allow(dead_code)]
    pub(crate) fn from_raw_lambda_scalar(
        name: Identifier<'i>,
        lambda: Vec<ValueAccessor<'i>>,
        position: usize,
    ) -> Self {
        let scalar = ScalarWithLambda::from_raw_lambda(name, lambda, position);
        Self::Scalar(scalar)
    }

    // This function is unsafe and lambda must be non-empty, although it's used only for tests
    #[allow(dead_code)]
    pub(crate) fn from_raw_lambda_stream(
        name: Identifier<'i>,
        lambda: Vec<ValueAccessor<'i>>,
        position: usize,
    ) -> Self {
        let stream = StreamWithLambda::from_raw_lambda(name, lambda, position);
        Self::Stream(stream)
    }
}
