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

mod impls;
mod traits;

use air_lambda_parser::LambdaAST;

use serde::Deserialize;
use serde::Serialize;

/// A scalar value without lambda.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Scalar<'i> {
    pub name: &'i str,
    pub position: usize,
}

/// A scalar value with possible lambda expression.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct ScalarWithLambda<'i> {
    pub name: &'i str,
    #[serde(borrow)]
    pub lambda: Option<LambdaAST<'i>>,
    pub position: usize,
}

/// A stream without lambda.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Stream<'i> {
    pub name: &'i str,
    pub position: usize,
}

/// A stream with possible lambda expression.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct StreamWithLambda<'i> {
    pub name: &'i str,
    #[serde(borrow)]
    pub lambda: Option<LambdaAST<'i>>,
    pub position: usize,
}

/// A canonicalized stream without lambda.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CanonStream<'i> {
    pub name: &'i str,
    pub position: usize,
}

/// A canonicalized stream with lambda.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CanonStreamWithLambda<'i> {
    pub name: &'i str,
    #[serde(borrow)]
    pub lambda: Option<LambdaAST<'i>>,
    pub position: usize,
}

/// A variable that could be either scalar or stream without lambda.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Variable<'i> {
    #[serde(borrow)]
    Scalar(Scalar<'i>),
    #[serde(borrow)]
    Stream(Stream<'i>),
    #[serde(borrow)]
    CanonStream(CanonStream<'i>),
}

/// A variable that could be either scalar or stream with possible lambda expression.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum VariableWithLambda<'i> {
    #[serde(borrow)]
    Scalar(ScalarWithLambda<'i>),
    #[serde(borrow)]
    Stream(StreamWithLambda<'i>),
    #[serde(borrow)]
    CanonStream(CanonStreamWithLambda<'i>),
}
