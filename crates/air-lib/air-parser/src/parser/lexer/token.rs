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
    EmbeddedScript(&'input str),
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
