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

use super::CanonStream;
use super::CanonStreamMap;
use super::CanonStreamMapWithLambda;
use super::CanonStreamWithLambda;
use super::ImmutableVariable;
use super::ImmutableVariableWithLambda;
use super::Scalar;
use super::ScalarWithLambda;
use super::Stream;
use super::StreamMap;

use air_lambda_ast::LambdaAST;

use serde::Deserialize;
use serde::Serialize;

/// Contains all variable variants that could be resolved to a peer id.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ResolvableToPeerIdVariable<'i> {
    InitPeerId,
    Literal(&'i str),
    Scalar(Scalar<'i>),
    ScalarWithLambda(ScalarWithLambda<'i>),
    // canon without lambda can't be resolved to a string, since it represents an array of values
    CanonStreamWithLambda(CanonStreamWithLambda<'i>),
}

/// Contains all variable variants that could be resolved to a string type.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ResolvableToStringVariable<'i> {
    Literal(&'i str),
    Scalar(Scalar<'i>),
    ScalarWithLambda(ScalarWithLambda<'i>),
    // canon without lambda can't be resolved to a string, since it represents an array of values
    CanonStreamWithLambda(CanonStreamWithLambda<'i>),
}

/// Triplet represents a location of the executable code in the network.
/// It is build from `PeerPart` and `FunctionPart` of a `Call` instruction.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Triplet<'i> {
    #[serde(borrow)]
    pub peer_id: ResolvableToPeerIdVariable<'i>,
    #[serde(borrow)]
    pub service_id: ResolvableToStringVariable<'i>,
    #[serde(borrow)]
    pub function_name: ResolvableToStringVariable<'i>,
}

/// Represents all immutable values that is possible to set in AIR scripts.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ImmutableValue<'i> {
    InitPeerId,
    LastError(Option<LambdaAST<'i>>),
    Timestamp,
    TTL,
    Literal(&'i str),
    Number(Number),
    Boolean(bool),
    EmptyArray, // only empty arrays are allowed now
    Variable(ImmutableVariable<'i>),
    VariableWithLambda(ImmutableVariableWithLambda<'i>),
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub enum CallOutputValue<'i> {
    #[serde(borrow)]
    Scalar(Scalar<'i>),
    #[serde(borrow)]
    Stream(Stream<'i>),
    None,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ApArgument<'i> {
    InitPeerId,
    Timestamp,
    TTL,
    LastError(Option<LambdaAST<'i>>),
    Literal(&'i str),
    Number(Number),
    Boolean(bool),
    EmptyArray,
    Scalar(Scalar<'i>),
    ScalarWithLambda(ScalarWithLambda<'i>),
    CanonStream(CanonStream<'i>),
    CanonStreamMap(CanonStreamMap<'i>),
    CanonStreamWithLambda(CanonStreamWithLambda<'i>),
    CanonStreamMapWithLambda(CanonStreamMapWithLambda<'i>),
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ApResult<'i> {
    #[serde(borrow)]
    Scalar(Scalar<'i>),
    #[serde(borrow)]
    Stream(Stream<'i>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ApMapKey<'i> {
    Literal(&'i str),
    Number(Number),
    Scalar(Scalar<'i>),
    ScalarWithLambda(ScalarWithLambda<'i>),
    CanonStreamWithLambda(CanonStreamWithLambda<'i>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Number {
    Int(i64),
    Float(f64),
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum FoldScalarIterable<'i> {
    #[serde(borrow)]
    Scalar(Scalar<'i>),
    #[serde(borrow)]
    ScalarWithLambda(ScalarWithLambda<'i>),
    // it's important not to have lambda here
    #[serde(borrow)]
    CanonStream(CanonStream<'i>),
    EmptyArray,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum NewArgument<'i> {
    #[serde(borrow)]
    Scalar(Scalar<'i>),
    #[serde(borrow)]
    Stream(Stream<'i>),
    #[serde(borrow)]
    StreamMap(StreamMap<'i>),
    #[serde(borrow)]
    CanonStream(CanonStream<'i>),
}
