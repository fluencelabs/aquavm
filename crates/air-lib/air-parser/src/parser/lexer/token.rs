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

use crate::LambdaAST;

use air_parser_utils::Identifier;
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
        name: Identifier<'input>,
        position: usize,
    },
    ScalarWithLambda {
        name: Identifier<'input>,
        lambda: LambdaAST<'input>,
        position: usize,
    },
    Stream {
        name: Identifier<'input>,
        position: usize,
    },
    StreamWithLambda {
        name: Identifier<'input>,
        lambda: LambdaAST<'input>,
        position: usize,
    },
    CanonStream {
        name: Identifier<'input>,
        position: usize,
    },
    CanonStreamWithLambda {
        name: Identifier<'input>,
        lambda: LambdaAST<'input>,
        position: usize,
    },

    StringLiteral(&'input str),
    I64(i64),
    F64(f64),
    Boolean(bool),

    InitPeerId,
    LastError,
    LastErrorWithLambda(LambdaAST<'input>),
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
    New,
    Next,
    Null,
    Match,
    MisMatch,
}
