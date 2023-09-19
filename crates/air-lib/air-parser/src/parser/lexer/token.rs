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

use super::AirPos;
use crate::LambdaAST;

use serde::Deserialize;
use serde::Serialize;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Token<'input> {
    OpenRoundBracket,
    CloseRoundBracket,
    OpenSquareBracket,
    CloseSquareBracket,

    Scalar {
        name: &'input str,
        position: AirPos,
    },
    ScalarWithLambda {
        name: &'input str,
        lambda: LambdaAST<'input>,
        position: AirPos,
    },
    Stream {
        name: &'input str,
        position: AirPos,
    },
    StreamWithLambda {
        name: &'input str,
        lambda: LambdaAST<'input>,
        position: AirPos,
    },
    StreamMapWithLambda {
        name: &'input str,
        lambda: LambdaAST<'input>,
        position: AirPos,
    },
    CanonStream {
        name: &'input str,
        position: AirPos,
    },
    CanonStreamWithLambda {
        name: &'input str,
        lambda: LambdaAST<'input>,
        position: AirPos,
    },
    StreamMap {
        name: &'input str,
        position: AirPos,
    },
    CanonStreamMap {
        name: &'input str,
        position: AirPos,
    },
    CanonStreamMapWithLambda {
        name: &'input str,
        lambda: LambdaAST<'input>,
        position: AirPos,
    },

    StringLiteral(&'input str),
    I64(i64),
    F64(f64),
    Boolean(bool),

    InitPeerId,
    LastError,
    Error,
    LastErrorWithLambda(LambdaAST<'input>),
    ErrorWithLambda(LambdaAST<'input>),
    Timestamp,
    TTL,

    Call,
    Canon,
    Ap,
    Seq,
    Par,
    Fail,
    Fold,
    Xor,
    Never,
    New,
    Next,
    Null,
    Match,
    MisMatch,
}
