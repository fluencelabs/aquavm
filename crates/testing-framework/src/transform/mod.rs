/*
 * Copyright 2022 Fluence Labs Limited
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

mod parser;
pub(crate) mod walker;

use crate::asserts::AssertionChain;

type Triplet = (Sexp, Sexp, Sexp);

#[derive(Debug, PartialEq)]
pub(crate) struct Call {
    triplet: Box<Triplet>,
    args: Vec<Sexp>,
    var: Option<Box<Sexp>>,
    annotation: Option<AssertionChain>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Sexp {
    Call(Call),
    List(Vec<Sexp>),
    Symbol(String),
    String(String),
}

impl Sexp {
    pub(crate) fn list(list: Vec<Self>) -> Self {
        Self::List(list)
    }

    pub(crate) fn symbol(name: impl ToString) -> Self {
        Self::Symbol(name.to_string())
    }

    pub(crate) fn string(value: impl ToString) -> Self {
        Self::String(value.to_string())
    }
}

impl std::fmt::Display for Sexp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use itertools::Itertools;

        match self {
            Sexp::Call(call) => {
                write!(
                    f,
                    "(call {peer_id} ({service} {func}) [{args}]{var})",
                    peer_id = call.triplet.0,
                    service = call.triplet.1,
                    func = call.triplet.2,
                    args = call.args.iter().format(" "),
                    var = match &call.var {
                        Some(var) => format!(" {}", var),
                        None => "".to_owned(),
                    }
                )
            }
            Sexp::List(items) => write!(f, "({})", items.iter().format(" ")),
            Sexp::Symbol(symbol) => write!(f, "{}", symbol),
            Sexp::String(string) => write!(f, r#""{}""#, string),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;

    #[test]
    fn test_parse_fmt_call() {
        let sexp_str = r#"(call "my_id" ("serv" "function") [other_peer_id "other_arg"])"#;
        let sexp = Sexp::from_str(sexp_str).unwrap();
        assert_eq!(format!("{}", sexp), sexp_str);
    }

    #[test]
    fn test_parse_fmt_call_var() {
        let sexp_str = r#"(call "my_id" ("serv" "function") [other_peer_id "other_arg"] var)"#;
        let sexp = Sexp::from_str(sexp_str).unwrap();
        assert_eq!(format!("{}", sexp), sexp_str);
    }

    #[test]
    fn test_parse_fmt_symbol() {
        let sexp_str = "symbol";
        let sexp = Sexp::from_str(sexp_str).unwrap();
        assert_eq!(format!("{}", sexp), sexp_str);
    }

    #[test]
    fn test_parse_fmt_string() {
        let sexp_str = r#""my_id""#;
        let sexp = Sexp::from_str(sexp_str).unwrap();
        assert_eq!(format!("{}", sexp), sexp_str);
    }

    #[test]
    fn test_parse_fmt_sexp() {
        let sexp_str = r#"(par (ap x y) (fold x y (next)))"#;
        let sexp = Sexp::from_str(sexp_str).unwrap();
        assert_eq!(format!("{}", sexp), sexp_str);
    }
}
