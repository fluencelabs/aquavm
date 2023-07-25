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

use super::*;

use std::fmt;

impl fmt::Display for Instruction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Instruction::*;

        match self {
            Call(call) => write!(f, "{call}"),
            Canon(canon) => write!(f, "{canon}"),
            CanonMap(canon_map) => write!(f, "{canon_map}"),
            CanonStreamMapScalar(canon_stream_map_scalar) => write!(f, "{canon_stream_map_scalar}"),
            Ap(ap) => write!(f, "{ap}"),
            ApMap(ap_map) => write!(f, "{ap_map}"),
            Seq(seq) => write!(f, "{seq}"),
            Par(par) => write!(f, "{par}"),
            Xor(xor) => write!(f, "{xor}"),
            Match(match_) => write!(f, "{match_}"),
            MisMatch(mismatch) => write!(f, "{mismatch}"),
            Fail(fail) => write!(f, "{fail}"),
            FoldScalar(fold) => write!(f, "{fold}"),
            FoldStream(fold) => write!(f, "{fold}"),
            FoldStreamMap(fold) => write!(f, "{fold}"),
            Never(never) => write!(f, "{never}"),
            Next(next) => write!(f, "{next}"),
            New(new) => write!(f, "{new}"),
            Null(null) => write!(f, "{null}"),
            Error => write!(f, "error"),
            FoldCanonStreamMap(fold) => write!(f, "{fold}"),
        }
    }
}

impl fmt::Display for Call<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use itertools::Itertools;

        let args = self.args.iter().map(|arg| format!("{arg}")).join(" ");
        write!(f, "call {} [{}] {}", self.triplet, args, self.output)
    }
}

impl fmt::Display for Canon<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "canon {} {} {}",
            self.peer_id, self.stream, self.canon_stream
        )
    }
}

impl fmt::Display for CanonMap<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "canon {} {} {}",
            self.peer_id, self.stream_map, self.canon_stream_map
        )
    }
}

impl fmt::Display for CanonStreamMapScalar<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "canon {} {} {}",
            self.peer_id, self.stream_map, self.scalar
        )
    }
}

impl fmt::Display for Ap<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ap {} {}", self.argument, self.result)
    }
}

impl fmt::Display for ApMap<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ap ({} {}) {}", self.key, self.value, self.map)
    }
}

impl fmt::Display for Fail<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fail::Scalar(scalar) => write!(f, "fail {scalar}"),
            Fail::ScalarWithLambda(scalar) => write!(f, "fail {scalar}"),
            Fail::Literal {
                ret_code,
                error_message,
            } => write!(f, r#"fail {ret_code} "{error_message}""#),
            Fail::CanonStreamWithLambda(stream) => {
                write!(f, "fail {stream}")
            }
            Fail::LastError => write!(f, "fail %last_error%"),
        }
    }
}

impl fmt::Display for FoldScalar<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fold {} {}", self.iterable, self.iterator)
    }
}

impl fmt::Display for FoldStream<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fold {} {}", self.iterable, self.iterator)
    }
}

impl fmt::Display for FoldStreamMap<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fold {} {}", self.iterable, self.iterator)
    }
}

impl fmt::Display for FoldCanonStreamMap<'_> {
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

impl fmt::Display for Never {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "never")
    }
}

impl fmt::Display for Next<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "next {}", self.iterator)
    }
}

impl fmt::Display for New<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "new {}", self.argument)
    }
}
