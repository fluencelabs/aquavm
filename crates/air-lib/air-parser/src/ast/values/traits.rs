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
use air_lambda_ast::format_lambda_ast;
use std::fmt;

impl fmt::Display for Scalar<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for ScalarWithLambda<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.lambda {
            Some(lambda) => write!(f, "{}.${}", self.name, format_lambda_ast(lambda)),
            None => write!(f, "{}", self.name),
        }
    }
}

impl fmt::Display for Stream<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for StreamWithLambda<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.lambda {
            Some(lambda) => write!(f, "{}.${}", self.name, format_lambda_ast(lambda)),
            None => write!(f, "{}", self.name),
        }
    }
}

impl fmt::Display for Variable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Variable::*;

        match self {
            Scalar(scalar) => write!(f, "{}", scalar),
            Stream(stream) => write!(f, "{}", stream),
        }
    }
}

impl fmt::Display for VariableWithLambda<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use VariableWithLambda::*;

        match self {
            Scalar(scalar) => write!(f, "{}", scalar),
            Stream(stream) => write!(f, "{}", stream),
        }
    }
}
