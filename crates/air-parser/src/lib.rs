#![allow(unused_imports, unused_variables, dead_code)]

#[macro_use]
extern crate fstrings;

mod ast;
mod lalrpop {
    mod aqua;
    mod parser;
}
