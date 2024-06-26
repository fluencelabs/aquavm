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

/// An error wrapper with an optional lens.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct InstructionErrorAST<'lens> {
    #[serde(borrow)]
    pub lens: Option<LambdaAST<'lens>>,
}
