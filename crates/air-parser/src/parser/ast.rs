/*
 * Copyright 2020 Fluence Labs Limited
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

use serde::Deserialize;
use serde::Serialize;

use std::rc::Rc;

#[allow(clippy::large_enum_variant)] // for Null and Error variants
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum Instruction<'i> {
    Null(Null),
    Call(Call<'i>),
    Seq(Seq<'i>),
    Par(Par<'i>),
    Xor(Xor<'i>),
    Match(Match<'i>),
    Fold(Fold<'i>),
    Next(Next<'i>),
    Error,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum PeerPart<'i> {
    PeerPk(CallArgValue<'i>),
    PeerPkWithServiceId(CallArgValue<'i>, CallArgValue<'i>),
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum FunctionPart<'i> {
    FuncName(CallArgValue<'i>),
    ServiceIdWithFuncName(CallArgValue<'i>, CallArgValue<'i>),
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Call<'i> {
    pub peer_part: PeerPart<'i>,
    pub function_part: FunctionPart<'i>,
    pub args: Rc<Vec<CallArgValue<'i>>>,
    pub output: CallOutputValue<'i>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum CallArgValue<'i> {
    InitPeerId,
    Literal(&'i str),
    Variable(&'i str),
    JsonPath { variable: &'i str, path: &'i str },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum IterableValue<'i> {
    Variable(&'i str),
    JsonPath { variable: &'i str, path: &'i str },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum MatchableValue<'i> {
    Literal(&'i str),
    Variable(&'i str),
    JsonPath { variable: &'i str, path: &'i str },
}

#[derive(Serialize, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CallOutputValue<'i> {
    Scalar(&'i str),
    Accumulator(&'i str),
    None,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Seq<'i>(pub Box<Instruction<'i>>, pub Box<Instruction<'i>>);

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Par<'i>(pub Box<Instruction<'i>>, pub Box<Instruction<'i>>);

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Xor<'i>(pub Box<Instruction<'i>>, pub Box<Instruction<'i>>);

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Match<'i>(
    pub MatchableValue<'i>,
    pub MatchableValue<'i>,
    pub Box<Instruction<'i>>,
);

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Fold<'i> {
    pub iterable: IterableValue<'i>,
    pub iterator: &'i str,
    pub instruction: Rc<Instruction<'i>>,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Next<'i>(pub &'i str);

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Null;
