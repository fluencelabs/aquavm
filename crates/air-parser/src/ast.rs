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

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction<'i> {
    Null,
    Call(Call<'i>),
    Seq(Seq<'i>),
    Error,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PeerPart<'i> {
    PeerPk(Value<'i>),
    PeerPkWithServiceId(Value<'i>, Value<'i>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum FunctionPart<'i> {
    FuncName(Value<'i>),
    ServiceIdWithFuncName(Value<'i>, Value<'i>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Call<'i> {
    pub peer: PeerPart<'i>,
    pub f: FunctionPart<'i>,
    pub args: Vec<Value<'i>>,
    pub output: CallOutput<'i>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Value<'i> {
    Variable(&'i str),
    Literal(&'i str),
}

#[derive(Debug, PartialEq, Eq)]
pub enum CallOutput<'i> {
    Scalar(&'i str),
    Accumulator(&'i str),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Seq<'i>(pub Box<Instruction<'i>>, pub Box<Instruction<'i>>);
