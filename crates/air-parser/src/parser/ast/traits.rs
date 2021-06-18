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

use super::*;

use std::fmt;

impl fmt::Display for CallInstrArgValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CallInstrArgValue::*;

        match self {
            InitPeerId => write!(f, "%init_peer_id%"),
            LastError(json_path) => write!(f, "%last_error%{}", json_path),
            Literal(str) => write!(f, r#""{}""#, str),
            Number(number) => write!(f, "{}", number),
            Boolean(bool) => write!(f, "{}", bool),
            Variable(str) => write!(f, "{}", str),
            JsonPath {
                variable,
                path,
                should_flatten,
            } => print_json_path(variable, path, should_flatten, f),
        }
    }
}

fn print_json_path<'a>(
    variable: &Variable<'a>,
    path: &str,
    should_flatten: &bool,
    f: &mut fmt::Formatter,
) -> fmt::Result {
    let maybe_flatten_char = if *should_flatten { "!" } else { "" };

    write!(f, "{}.{}{}", variable, path, maybe_flatten_char)
}

impl fmt::Display for CallInstrValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CallInstrValue::*;

        match self {
            InitPeerId => write!(f, "%init_peer_id%"),
            Literal(str) => write!(f, r#""{}""#, str),
            Variable(str) => write!(f, "{}", str),
            JsonPath {
                variable,
                path,
                should_flatten,
            } => print_json_path(variable, path, should_flatten, f),
        }
    }
}

impl fmt::Display for IterableValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::parser::lexer;
        use IterableValue::*;

        match self {
            Variable(str) => write!(f, "{}", str),
            JsonPath {
                scalar_name,
                path,
                should_flatten,
            } => print_json_path(
                &lexer::Variable::Scalar(scalar_name),
                path,
                should_flatten,
                f,
            ),
        }
    }
}

impl fmt::Display for MatchableValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use MatchableValue::*;

        match self {
            InitPeerId => write!(f, "%init_peer_id%"),
            Literal(str) => write!(f, r#""{}""#, str),
            Number(number) => write!(f, "{}", number),
            Boolean(bool) => write!(f, "{}", bool),
            Variable(str) => write!(f, "{}", str),
            JsonPath {
                variable,
                path,
                should_flatten,
            } => print_json_path(variable, path, should_flatten, f),
        }
    }
}

impl fmt::Display for CallOutputValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CallOutputValue::*;

        match self {
            Variable(variable) => write!(f, "{}", variable),
            None => Ok(()),
        }
    }
}

impl fmt::Display for PeerPart<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PeerPart::*;

        match self {
            PeerPk(peer_pk) => write!(f, "{}", peer_pk),
            PeerPkWithServiceId(peer_pk, service_id) => write!(f, "({} {})", peer_pk, service_id),
        }
    }
}

impl fmt::Display for FunctionPart<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use FunctionPart::*;

        match self {
            FuncName(func_name) => write!(f, "{}", func_name),
            ServiceIdWithFuncName(service_id, func_name) => {
                write!(f, "({} {})", service_id, func_name)
            }
        }
    }
}

impl fmt::Display for Instruction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Instruction::*;

        match self {
            Null(null) => write!(f, "{}", null),
            Call(call) => write!(f, "{}", call),
            Seq(seq) => write!(f, "{}", seq),
            Par(par) => write!(f, "{}", par),
            Xor(xor) => write!(f, "{}", xor),
            Match(match_) => write!(f, "{}", match_),
            MisMatch(mismatch) => write!(f, "{}", mismatch),
            Fold(fold) => write!(f, "{}", fold),
            Next(next) => write!(f, "{}", next),
            Error => Ok(()),
        }
    }
}

impl fmt::Display for Call<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use itertools::Itertools;

        write!(f, "call {} {}", self.peer_part, self.function_part)?;

        let args = self.args.iter().map(|arg| format!("{}", arg)).join(" ");
        write!(f, " [{}]", args)?;

        write!(f, " {}", self.output)
    }
}

impl fmt::Display for Fold<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fold {} {}", self.iterable, self.iterator)
    }
}

impl fmt::Display for Seq<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "seq")
    }
}

impl fmt::Display for Par<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "par")
    }
}

impl fmt::Display for Null {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "null")
    }
}

impl fmt::Display for Xor<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "xor")
    }
}

impl fmt::Display for Match<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "match {} {}", self.left_value, self.right_value)
    }
}

impl fmt::Display for MisMatch<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mismatch {} {}", self.left_value, self.right_value)
    }
}

impl fmt::Display for Next<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "next")
    }
}
