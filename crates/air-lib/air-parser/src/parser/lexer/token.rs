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

mod traits;

use super::LexerError;
use super::LexerResult;
use crate::LambdaAST;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Token<'input> {
    OpenRoundBracket,
    CloseRoundBracket,
    OpenSquareBracket,
    CloseSquareBracket,
    SquareBrackets, // [] symbolize empty array, it's possible to have it only in an argument position

    StringLiteral(&'input str),
    Scalar {
        name: &'input str,
    },
    ScalarWithLambda {
        name: &'input str,
        lambda: LambdaAST<'input>,
    },
    Stream {
        name: &'input str,
        position: usize,
    },
    StreamWithLambda {
        name: &'input str,
        lambda: LambdaAST<'input>,
        position: usize,
    },
    Number(Number),
    Boolean(bool),

    InitPeerId,
    LastError(LastErrorPath),

    Call,
    Ap,
    Seq,
    Par,
    Fold,
    Xor,
    New,
    Next,
    Null,
    Match,
    MisMatch,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum LastErrorPath {
    // %last_error%.instruction
    Instruction,
    // %last_error%.msg
    Message,
    // %last_error%.peer_id
    PeerId,
    // %last_error%
    None,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Number {
    Int(i64),
    Float(f64),
}

pub(crate) enum UnparsedNumber<'input> {
    // raw value and starting pos
    Int(&'input str, usize),
    Float(&'input str, usize),
}
