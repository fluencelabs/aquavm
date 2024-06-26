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

use super::*;

use serde::Serialize;

use std::rc::Rc;

// TODO: sort instruction in alphanumeric order
#[allow(clippy::large_enum_variant)] // for Null and Error variants
#[derive(Serialize, Debug, PartialEq)]
pub enum Instruction<'i> {
    Call(Box<Call<'i>>),
    Ap(Box<Ap<'i>>),
    ApMap(Box<ApMap<'i>>),
    Canon(Box<Canon<'i>>),
    CanonMap(Box<CanonMap<'i>>),
    CanonStreamMapScalar(Box<CanonStreamMapScalar<'i>>),
    Seq(Box<Seq<'i>>),
    Par(Box<Par<'i>>),
    Xor(Box<Xor<'i>>),
    Match(Box<Match<'i>>),
    MisMatch(Box<MisMatch<'i>>),
    Fail(Box<Fail<'i>>),
    FoldScalar(Box<FoldScalar<'i>>),
    FoldStream(Box<FoldStream<'i>>),
    FoldStreamMap(Box<FoldStreamMap<'i>>),
    Never(Never),
    New(Box<New<'i>>),
    Next(Box<Next<'i>>),
    Null(Null),
    Error,
}

/// (call (peer part of a triplet: PeerPart) (function part of a triplet: FunctionPart) [arguments] output)
#[derive(Serialize, Debug, PartialEq)]
pub struct Call<'i> {
    pub triplet: Triplet<'i>,
    pub args: Rc<Vec<ImmutableValue<'i>>>,
    pub output: CallOutputValue<'i>,
}

/// (ap argument result)
#[derive(Serialize, Debug, PartialEq)]
pub struct Ap<'i> {
    pub argument: ApArgument<'i>,
    pub result: ApResult<'i>,
}

/// (ap key value %map)
#[derive(Serialize, Debug, PartialEq)]
pub struct ApMap<'i> {
    pub key: StreamMapKeyClause<'i>,
    pub value: ApArgument<'i>,
    pub map: StreamMap<'i>,
}

/// (canon peer_id $stream #canon_stream)
#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Canon<'i> {
    pub peer_id: ResolvableToPeerIdVariable<'i>,
    pub stream: Stream<'i>,
    pub canon_stream: CanonStream<'i>,
}

/// (canon peer_id %stream_map #%canon_stream_map)
#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct CanonMap<'i> {
    pub peer_id: ResolvableToPeerIdVariable<'i>,
    pub stream_map: StreamMap<'i>,
    pub canon_stream_map: CanonStreamMap<'i>,
}

/// (canon peer_id %stream_map scalar)
#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct CanonStreamMapScalar<'i> {
    pub peer_id: ResolvableToPeerIdVariable<'i>,
    pub stream_map: StreamMap<'i>,
    pub scalar: Scalar<'i>,
}

/// (seq instruction instruction)
#[derive(Serialize, Debug, PartialEq)]
pub struct Seq<'i>(pub Instruction<'i>, pub Instruction<'i>);

/// (par instruction instruction)
#[derive(Serialize, Debug, PartialEq)]
pub struct Par<'i>(pub Instruction<'i>, pub Instruction<'i>);

/// (xor instruction instruction)
#[derive(Serialize, Debug, PartialEq)]
pub struct Xor<'i>(pub Instruction<'i>, pub Instruction<'i>);

/// (match left_value right_value instruction)
#[derive(Serialize, Debug, PartialEq)]
pub struct Match<'i> {
    pub left_value: ImmutableValue<'i>,
    pub right_value: ImmutableValue<'i>,
    pub instruction: Instruction<'i>,
}

/// (mismatch left_value right_value instruction)
#[derive(Serialize, Debug, PartialEq)]
pub struct MisMatch<'i> {
    pub left_value: ImmutableValue<'i>,
    pub right_value: ImmutableValue<'i>,
    pub instruction: Instruction<'i>,
}

/// (fail 1337 "error message")
/// (fail %last_error%)
/// (fail value)
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum Fail<'i> {
    Scalar(Scalar<'i>),
    ScalarWithLambda(ScalarWithLambda<'i>),
    Literal {
        ret_code: i64,
        error_message: &'i str,
    },
    CanonStreamWithLambda(CanonStreamWithLambda<'i>),
    LastError,
    Error,
}

/// (fold scalar_iterable iterator instruction)
#[derive(Serialize, Debug, PartialEq)]
pub struct FoldScalar<'i> {
    #[serde(borrow)]
    pub iterable: FoldScalarIterable<'i>,
    #[serde(borrow)]
    pub iterator: Scalar<'i>,
    pub instruction: Rc<Instruction<'i>>,
    // option is needed to provide a graceful period of adoption
    pub last_instruction: Option<Rc<Instruction<'i>>>,
    pub span: Span,
}

/// (fold stream_iterable iterator instruction)
#[derive(Serialize, Debug, PartialEq)]
pub struct FoldStream<'i> {
    pub iterable: Stream<'i>,
    #[serde(borrow)]
    pub iterator: Scalar<'i>,
    pub instruction: Rc<Instruction<'i>>,
    // option is needed to provide a graceful period of adoption
    pub last_instruction: Option<Rc<Instruction<'i>>>,
    pub span: Span,
}

/// (fold stream_map_iterable iterator instruction)
#[derive(Serialize, Debug, PartialEq)]
pub struct FoldStreamMap<'i> {
    pub iterable: StreamMap<'i>,
    #[serde(borrow)]
    pub iterator: Scalar<'i>,
    pub instruction: Rc<Instruction<'i>>,
    // option is needed to provide a graceful period of adoption
    pub last_instruction: Option<Rc<Instruction<'i>>>,
    pub span: Span,
}

/// (fold stream_iterable iterator instruction)
#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Next<'i> {
    pub iterator: Scalar<'i>,
}

/// (never)
#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Never;

/// (new variable instruction)
#[derive(Serialize, Debug, PartialEq)]
pub struct New<'i> {
    pub argument: NewArgument<'i>,
    pub instruction: Instruction<'i>,
    pub span: Span,
}

/// (null)
#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Null;

pub trait PeerIDErrorLogable {
    fn log_errors_with_peer_id(&self) -> bool;
}
