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
use super::Scalar;
use super::ScalarWithLambda;
use super::Stream;
use super::VariableWithLambda;

use air_lambda_ast::LambdaAST;

use crate::ast::CanonStreamWithLambda;
use serde::Deserialize;
use serde::Serialize;

// TODO: rename CallInstrValue, since it'd used by the canon instruction
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum CallInstrValue<'i> {
    InitPeerId,
    Literal(&'i str),
    Variable(VariableWithLambda<'i>),
}

/// Triplet represents a location of the executable code in the network.
/// It is build from `PeerPart` and `FunctionPart` of a `Call` instruction.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Triplet<'i> {
    #[serde(borrow)]
    pub peer_pk: CallInstrValue<'i>,
    #[serde(borrow)]
    pub service_id: CallInstrValue<'i>,
    #[serde(borrow)]
    pub function_name: CallInstrValue<'i>,
}

/// Represents all values that is possible to set in AIR scripts.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Value<'i> {
    InitPeerId,
    LastError(Option<LambdaAST<'i>>),
    Timestamp,
    TTL,
    Literal(&'i str),
    Number(Number),
    Boolean(bool),
    EmptyArray, // only empty arrays are allowed now
    Variable(VariableWithLambda<'i>),
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
    Scalar(ScalarWithLambda<'i>),
    CanonStream(CanonStreamWithLambda<'i>),
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ApResult<'i> {
    #[serde(borrow)]
    Scalar(Scalar<'i>),
    #[serde(borrow)]
    Stream(Stream<'i>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Number {
    Int(i64),
    Float(f64),
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum FoldScalarIterable<'i> {
    #[serde(borrow)]
    Scalar(ScalarWithLambda<'i>),
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
    CanonStream(CanonStream<'i>),
}
