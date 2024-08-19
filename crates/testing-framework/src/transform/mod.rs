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

pub(crate) mod parser;
pub mod walker;

use crate::asserts::ServiceDefinition;

type Triplet = (Sexp, Sexp, Sexp);

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Call {
    triplet: Triplet,
    args: Vec<Sexp>,
    var: Option<Box<Sexp>>,
    service_desc: Option<ServiceDefinition>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Canon {
    peer: Sexp,
    stream: Sexp,
    target: Sexp,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Embed {
    args: Vec<Sexp>,
    script: Sexp,
    var: Option<Box<Sexp>>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Sexp {
    Call(Box<Call>),
    Canon(Box<Canon>),
    Embed(Box<Embed>),
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

    pub(crate) fn canon(peer: Sexp, stream: Sexp, target: Sexp) -> Self {
        Self::Canon(Box::new(Canon {
            peer,
            stream,
            target,
        }))
    }

    pub(crate) fn embed(args: Vec<Sexp>, script: Sexp, var: Option<Box<Sexp>>) -> Self {
        Self::Embed(Box::new(Embed { args, script, var }))
    }

    pub(crate) fn inject(&mut self, service_definition: ServiceDefinition) -> Result<(), String> {
        match self {
            Sexp::Call(ref mut call) => {
                call.service_desc = Some(service_definition);
                Ok(())
            }
            Sexp::List(ref mut list) => match list.last_mut() {
                Some(last) => last.inject(service_definition),
                None => Err("cannot attach a service definition an empty list".to_owned()),
            },
            Sexp::Canon(_) => Err(format!(
                "cannot attach a service definition to a canon {self:?}"
            )),
            Sexp::Embed(_) => Err(format!(
                "cannot attach a service definition to an embed {self:?}"
            )),
            Sexp::Symbol(s) => Err(format!(
                "cannot attach a service definition to a symbol {s:?}"
            )),
            Sexp::String(ref s) => Err(format!(
                r#"cannot attach a service definition to a string: "{s:?}""#
            )),
        }
    }
}

impl std::fmt::Display for Sexp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use itertools::Itertools;

        match self {
            Sexp::Call(call) => {
                write!(
                    f,
                    "(call {peer} ({service} {func}) [{args}]{var})",
                    peer = call.triplet.0,
                    service = call.triplet.1,
                    func = call.triplet.2,
                    args = call.args.iter().format(" "),
                    var = match &call.var {
                        Some(var) => format!(" {var}"),
                        None => "".to_owned(),
                    }
                )
            }
            Sexp::Canon(canon) => {
                write!(
                    f,
                    "(canon {peer} {stream} {target})",
                    peer = canon.peer,
                    stream = canon.stream,
                    target = canon.target,
                )
            }
            Sexp::Embed(embed) => {
                write!(
                    f,
                    "(embed [{args}] #"{script}"#{var})",
                    args = embed.args.iter().format(" "),
                    script = embed.script,
                    var = match &embed.var {
                        Some(var) => format!(" {var}"),
                        None => "".to_owned(),
                    }
                )
            }
            Sexp::List(items) => write!(f, "({})", items.iter().format(" ")),
            Sexp::Symbol(symbol) => write!(f, "{symbol}"),
            Sexp::String(string) => write!(f, r#""{string}""#),
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
        assert_eq!(format!("{sexp}"), sexp_str);
    }

    #[test]
    fn test_parse_fmt_call_var() {
        let sexp_str = r#"(call "my_id" ("serv" "function") [other_peer_id "other_arg"] var)"#;
        let sexp = Sexp::from_str(sexp_str).unwrap();
        assert_eq!(format!("{sexp}"), sexp_str);
    }

    #[test]
    fn test_parse_canon() {
        let sexp_str = r#"(canon "my_id" $stream #canon)"#;
        let sexp = Sexp::from_str(sexp_str).unwrap();
        assert_eq!(
            sexp,
            Sexp::canon(
                Sexp::string("my_id"),
                Sexp::symbol("$stream"),
                Sexp::symbol("#canon"),
            )
        );
    }

    #[test]
    fn test_parse_fmt_symbol() {
        let sexp_str = "symbol";
        let sexp = Sexp::from_str(sexp_str).unwrap();
        assert_eq!(format!("{sexp}"), sexp_str);
    }

    #[test]
    fn test_parse_fmt_string() {
        let sexp_str = r#""my_id""#;
        let sexp = Sexp::from_str(sexp_str).unwrap();
        assert_eq!(format!("{sexp}"), sexp_str);
    }

    #[test]
    fn test_parse_fmt_sexp() {
        let sexp_str = r#"(par (ap x y) (fold x y (next)))"#;
        let sexp = Sexp::from_str(sexp_str).unwrap();
        assert_eq!(format!("{sexp}"), sexp_str);
    }
}
