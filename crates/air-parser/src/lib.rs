#![deny(unused_imports, unused_variables, dead_code)]

#[cfg(test)]
#[macro_use]
extern crate fstrings;

mod lalrpop {
    #[cfg(test)]
    pub mod tests;

    // aqua is auto-generated, so exclude it from `cargo fmt -- --check`
    #[rustfmt::skip]
    pub mod aqua;
    pub mod parser;
}
pub mod ast;

pub use lalrpop::parser::parse;

// #[cfg(test)]
pub use lalrpop::aqua::InstrParser;
