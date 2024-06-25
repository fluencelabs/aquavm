/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

mod impls;
mod traits;

use super::CanonStream;
use super::CanonStreamMap;
use super::CanonStreamMapWithLambda;
use super::CanonStreamWithLambda;
use super::ImmutableVariable;
use super::ImmutableVariableWithLambda;
use super::InstructionErrorAST;
use super::Scalar;
use super::ScalarWithLambda;
use super::Stream;
use super::StreamMap;

use air_interpreter_value::JsonString;
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
    CanonStreamMapWithLambda(CanonStreamMapWithLambda<'i>),
}

/// Contains all variable variants that could be resolved to a string type.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ResolvableToStringVariable<'i> {
    Literal(&'i str),
    Scalar(Scalar<'i>),
    ScalarWithLambda(ScalarWithLambda<'i>),
    // canon without lambda can't be resolved to a string, since it represents an array of values
    CanonStreamWithLambda(CanonStreamWithLambda<'i>),
    CanonStreamMapWithLambda(CanonStreamMapWithLambda<'i>),
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
    Error(#[serde(borrow)] InstructionErrorAST<'i>),
    LastError(Option<LambdaAST<'i>>),
    Timestamp,
    TTL,
    Literal(JsonString),
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
    Error(#[serde(borrow)] InstructionErrorAST<'i>),
    LastError(Option<LambdaAST<'i>>),
    Literal(JsonString),
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
pub enum StreamMapKeyClause<'i> {
    Literal(JsonString),
    Int(i64),
    Scalar(#[serde(borrow)] Scalar<'i>),
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
    CanonStreamMap(CanonStreamMap<'i>),
    CanonStreamMapWithLambda(CanonStreamMapWithLambda<'i>),
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
    #[serde(borrow)]
    CanonStreamMap(CanonStreamMap<'i>),
}
