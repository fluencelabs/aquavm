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

use crate::parser::lexer::AirPos;

use air_lambda_parser::LambdaAST;
use serde::Deserialize;
use serde::Serialize;

/// A scalar value without a lambda.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Scalar<'i> {
    pub name: &'i str,
    pub position: AirPos,
}

/// A scalar value with a lambda expression.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct ScalarWithLambda<'i> {
    pub name: &'i str,
    #[serde(borrow)]
    pub lambda: LambdaAST<'i>,
    pub position: AirPos,
}

/// A stream without a lambda.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Stream<'i> {
    pub name: &'i str,
    pub position: AirPos,
}

/// A canonicalized stream without a lambda.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct CanonStream<'i> {
    pub name: &'i str,
    pub position: AirPos,
}

/// A canonicalized stream map without a lambda.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct CanonStreamMap<'i> {
    pub name: &'i str,
    pub position: AirPos,
}

/// A canonicalized stream with a lambda.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct CanonStreamWithLambda<'i> {
    pub name: &'i str,
    #[serde(borrow)]
    pub lambda: LambdaAST<'i>,
    pub position: AirPos,
}

/// A canonicalized stream map with a lambda.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct CanonStreamMapWithLambda<'i> {
    pub name: &'i str,
    #[serde(borrow)]
    pub lambda: LambdaAST<'i>,
    pub position: AirPos,
}

/// A variable that could be either scalar or stream without lambda.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ImmutableVariable<'i> {
    #[serde(borrow)]
    Scalar(Scalar<'i>),
    #[serde(borrow)]
    CanonStream(CanonStream<'i>),
    #[serde(borrow)]
    CanonStreamMap(CanonStreamMap<'i>),
}

/// A variable that could be either scalar or stream with possible lambda expression.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ImmutableVariableWithLambda<'i> {
    #[serde(borrow)]
    Scalar(ScalarWithLambda<'i>),
    #[serde(borrow)]
    CanonStream(CanonStreamWithLambda<'i>),
    #[serde(borrow)]
    CanonStreamMap(CanonStreamMapWithLambda<'i>),
}

/// A map based on top of a stream.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct StreamMap<'i> {
    pub name: &'i str,
    pub position: AirPos,
}
