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

use super::*;

use serde::Serialize;

use std::rc::Rc;

// TODO: sort instruction in alphanumeric order
#[allow(clippy::large_enum_variant)] // for Null and Error variants
#[derive(Serialize, Debug, PartialEq)]
pub enum Instruction<'i> {
    Call(Call<'i>),
    Ap(Ap<'i>),
    ApMap(ApMap<'i>),
    Canon(Canon<'i>),
    Seq(Seq<'i>),
    Par(Par<'i>),
    Xor(Xor<'i>),
    Match(Match<'i>),
    MisMatch(MisMatch<'i>),
    Fail(Fail<'i>),
    FoldScalar(FoldScalar<'i>),
    FoldStream(FoldStream<'i>),
    FoldStreamMap(FoldStreamMap<'i>),
    Never(Never),
    New(New<'i>),
    Next(Next<'i>),
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

/// (ap (key value) %map)
#[derive(Serialize, Debug, PartialEq)]
pub struct ApMap<'i> {
    pub key: ApMapKey<'i>,
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

/// (seq instruction instruction)
#[derive(Serialize, Debug, PartialEq)]
pub struct Seq<'i>(pub Box<Instruction<'i>>, pub Box<Instruction<'i>>);

/// (par instruction instruction)
#[derive(Serialize, Debug, PartialEq)]
pub struct Par<'i>(pub Box<Instruction<'i>>, pub Box<Instruction<'i>>);

/// (xor instruction instruction)
#[derive(Serialize, Debug, PartialEq)]
pub struct Xor<'i>(pub Box<Instruction<'i>>, pub Box<Instruction<'i>>);

/// (match left_value right_value instruction)
#[derive(Serialize, Debug, PartialEq)]
pub struct Match<'i> {
    pub left_value: ImmutableValue<'i>,
    pub right_value: ImmutableValue<'i>,
    pub instruction: Box<Instruction<'i>>,
}

/// (mismatch left_value right_value instruction)
#[derive(Serialize, Debug, PartialEq)]
pub struct MisMatch<'i> {
    pub left_value: ImmutableValue<'i>,
    pub right_value: ImmutableValue<'i>,
    pub instruction: Box<Instruction<'i>>,
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

/// (fold stream_iterable iterator instruction)
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
    pub instruction: Box<Instruction<'i>>,
    pub span: Span,
}

/// (null)
#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Null;
