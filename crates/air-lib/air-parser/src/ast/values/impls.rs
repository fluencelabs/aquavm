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

impl<'i> ScalarWithLambda<'i> {
    pub fn new(name: &'i str, lambda: Option<LambdaAST<'i>>) -> Self {
        Self { name, lambda }
    }

    pub(crate) fn from_raw_lambda(name: &'i str, lambda: Vec<ValueAccessor<'i>>) -> Self {
        let lambda = unsafe { LambdaAST::new_unchecked(lambda) };
        Self {
            name,
            lambda: Some(lambda),
        }
    }
}

impl<'i> StreamWithLambda<'i> {
    pub fn new(name: &'i str, lambda: Option<LambdaAST<'i>>) -> Self {
        Self { name, lambda }
    }

    #[allow(dead_code)]
    pub(crate) fn from_raw_lambda(name: &'i str, lambda: Vec<ValueAccessor<'i>>) -> Self {
        let lambda = unsafe { LambdaAST::new_unchecked(lambda) };
        Self {
            name,
            lambda: Some(lambda),
        }
    }
}

impl<'i> Scalar<'i> {
    pub fn new(name: &'i str) -> Self {
        Self { name }
    }
}

impl<'i> Stream<'i> {
    pub fn new(name: &'i str) -> Self {
        Self { name }
    }
}

impl<'i> Variable<'i> {
    pub fn scalar(name: &'i str) -> Self {
        Self::Scalar(Scalar { name })
    }

    pub fn stream(name: &'i str) -> Self {
        Self::Stream(Stream { name })
    }
}

impl<'i> VariableWithLambda<'i> {
    pub fn scalar(name: &'i str) -> Self {
        Self::Scalar(ScalarWithLambda { name, lambda: None })
    }

    pub fn scalar_wl(name: &'i str, lambda: LambdaAST<'i>) -> Self {
        Self::Scalar(ScalarWithLambda {
            name,
            lambda: Some(lambda),
        })
    }

    pub fn stream(name: &'i str) -> Self {
        Self::Stream(StreamWithLambda { name, lambda: None })
    }

    pub fn stream_wl(name: &'i str, lambda: LambdaAST<'i>) -> Self {
        Self::Stream(StreamWithLambda {
            name,
            lambda: Some(lambda),
        })
    }

    // This function is unsafe and lambda must be non-empty, although it's used only for tests
    #[allow(dead_code)]
    pub(crate) fn from_raw_lambda_scalar(name: &'i str, lambda: Vec<ValueAccessor<'i>>) -> Self {
        let scalar = ScalarWithLambda::from_raw_lambda(name, lambda);
        Self::Scalar(scalar)
    }

    // This function is unsafe and lambda must be non-empty, although it's used only for tests
    #[allow(dead_code)]
    pub(crate) fn from_raw_lambda_stream(name: &'i str, lambda: Vec<ValueAccessor<'i>>) -> Self {
        let stream = StreamWithLambda::from_raw_lambda(name, lambda);
        Self::Stream(stream)
    }
}
