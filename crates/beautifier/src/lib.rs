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

#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]

use air_parser::ast;
use itertools::Itertools as _;

use std::fmt::Display;
use std::io::{Error as IoError, Result as IoResult, Write};

pub const DEFAULT_INDENT_SIZE: usize = 4;

macro_rules! multiline {
    ($beautifier:expr, $indent:expr, $fmt1:literal $(, $arg1:expr)*; $nest1:expr) => ({
        let out = &mut $beautifier.output;
        $crate::fmt_indent(out, $indent)?;
        writeln!(out, $fmt1 $(, $arg1)*)?;
        $crate::Beautifier::beautify_walker($beautifier, $nest1, $indent + $beautifier.indent_size)
    });
    ($beautifier:expr, $indent:expr, $fmt1:literal $(, $arg1:expr)*; $nest1:expr; $fmt2:literal $(, $arg2:expr)*; $nest2:expr) => ({
        let step = $beautifier.indent_size;
        {
            let out = &mut $beautifier.output;
            $crate::fmt_indent(out, $indent)?;
            writeln!(out, $fmt1 $(, $arg1)*)?;
        }
        crate::Beautifier::beautify_walker(&mut *$beautifier, $nest1, $indent + step)?;
        {
            let out = &mut $beautifier.output;
            $crate::fmt_indent(out, $indent)?;
            writeln!(out, $fmt2 $(, $arg2)*)?;
        }
        crate::Beautifier::beautify_walker(&mut *$beautifier, $nest2, $indent + step)
    });
}

fn fmt_indent(output: &mut impl Write, indent: usize) -> IoResult<()> {
    write!(output, "{:indent$}", "", indent = indent)
}

struct BArgs<'a, 'b>(&'a [ast::Value<'b>]);

impl<'a, 'b> Display for BArgs<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0.iter().format(", ")))
    }
}

struct BTriplet<'a, 'b>(&'a ast::Triplet<'b>);

impl<'a, 'b> Display for BTriplet<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} ({}, {})",
            self.0.peer_pk, self.0.service_id, self.0.function_name
        ))
    }
}

/// Error produced by the Beautifier.
#[derive(Debug, thiserror::Error)]
pub enum BeautifyError {
    #[error("{0}")]
    Parse(String),
    #[error(transparent)]
    Io(#[from] IoError),
}

/// AIR beautifier.
pub struct Beautifier<W: Write> {
    output: W,
    indent_size: usize,
}

impl<W: Write> Beautifier<W> {
    /// Beautifier for the output with default indent size.
    pub fn new(output: W) -> Self {
        Self {
            output,
            indent_size: DEFAULT_INDENT_SIZE,
        }
    }

    /// Beautifier for the output with custom indent size.
    pub fn new_with_indent(output: W, indent_step: usize) -> Self {
        Self {
            output,
            indent_size: indent_step,
        }
    }

    /// Unwrap the Beautifier, returning the underlying writer.
    pub fn into_inner(self) -> W {
        self.output
    }

    /// Emit beautified code for the `air_script`.
    pub fn beautify(&mut self, air_script: &str) -> Result<(), BeautifyError> {
        let tree = air_parser::parse(air_script).map_err(BeautifyError::Parse)?;
        self.beautify_ast(tree)
    }

    /// Emit beautified code for the `ast`.
    pub fn beautify_ast<'a>(
        &mut self,
        ast: impl AsRef<ast::Instruction<'a>>,
    ) -> Result<(), BeautifyError> {
        Ok(self.beautify_walker(ast.as_ref(), 0)?)
    }

    fn beautify_walker(&mut self, node: &ast::Instruction, indent: usize) -> IoResult<()> {
        match node {
            ast::Instruction::Call(call) => self.beautify_call(call, indent),
            ast::Instruction::Ap(ap) => self.beautify_ap(ap, indent),
            ast::Instruction::Seq(seq) => self.beautify_seq(seq, indent),
            ast::Instruction::Par(par) => self.beautify_par(par, indent),
            ast::Instruction::Xor(xor) => self.beautify_xor(xor, indent),
            ast::Instruction::Match(match_) => self.beautify_match(match_, indent),
            ast::Instruction::MisMatch(mismatch) => self.beautify_mismatch(mismatch, indent),
            ast::Instruction::Fail(fail) => self.beautify_fail(fail, indent),
            ast::Instruction::FoldScalar(fold_scalar) => {
                self.beautify_fold_scalar(fold_scalar, indent)
            }
            ast::Instruction::FoldStream(fold_stream) => {
                self.beautify_fold_stream(fold_stream, indent)
            }
            ast::Instruction::New(new) => self.beautify_new(new, indent),
            ast::Instruction::Next(next) => self.beautify_next(next, indent),
            ast::Instruction::Null(null) => self.beautify_null(null, indent),
            ast::Instruction::Error => self.beautify_error(indent),
        }
    }

    fn beautify_call(&mut self, call: &ast::Call, indent: usize) -> IoResult<()> {
        fmt_indent(&mut self.output, indent)?;
        match &call.output {
            ast::CallOutputValue::Variable(v) => write!(&mut self.output, "{} <- ", v)?,
            ast::CallOutputValue::None => {}
        }
        writeln!(
            &mut self.output,
            "call {} [{}]",
            BTriplet(&call.triplet),
            BArgs(call.args.as_slice())
        )
    }

    fn beautify_ap(&mut self, ap: &ast::Ap, indent: usize) -> IoResult<()> {
        fmt_indent(&mut self.output, indent)?;
        writeln!(&mut self.output, "{}", ap)
    }

    fn beautify_seq(&mut self, seq: &ast::Seq, indent: usize) -> IoResult<()> {
        // please note that seq uses same indendation intentionally
        self.beautify_walker(&seq.0, indent)?;
        self.beautify_walker(&seq.1, indent)
    }

    fn beautify_par(&mut self, par: &ast::Par, indent: usize) -> IoResult<()> {
        multiline!(
            self, indent,
            "par:";
            &par.0;
            "|";  // TODO: SHOULD BE UNINDENTED AS PER SPEC; OR WE MAY CHANGE THE SPEC
            &par.1
        )
    }

    fn beautify_xor(&mut self, xor: &ast::Xor, indent: usize) -> IoResult<()> {
        multiline!(
            self, indent,
            "try:";
            &xor.0;
            "catch:";
            &xor.1
        )
    }

    fn beautify_match(&mut self, match_: &ast::Match, indent: usize) -> IoResult<()> {
        multiline!(
            self, indent,
            "{}:", match_;
            &match_.instruction
        )
    }

    fn beautify_mismatch(&mut self, mismatch: &ast::MisMatch, indent: usize) -> IoResult<()> {
        multiline!(
            self, indent,
            "{}:", mismatch;
            &mismatch.instruction
        )
    }

    fn beautify_fail(&mut self, fail: &ast::Fail, indent: usize) -> IoResult<()> {
        fmt_indent(&mut self.output, indent)?;
        writeln!(&mut self.output, "{}", fail)
    }

    fn beautify_fold_scalar(&mut self, fold: &ast::FoldScalar, indent: usize) -> IoResult<()> {
        multiline!(
            self, indent,
            "fold {} {}:", fold.iterable, fold.iterator;
            &fold.instruction
        )
    }

    fn beautify_fold_stream(&mut self, fold: &ast::FoldStream, indent: usize) -> IoResult<()> {
        multiline!(
            self, indent,
            "fold {} {}:", fold.iterable, fold.iterator;
            &fold.instruction
        )
    }

    fn beautify_new(&mut self, new: &ast::New, indent: usize) -> IoResult<()> {
        multiline!(
            self, indent,
            "{}:", new;
            &new.instruction
        )
    }

    fn beautify_next(&mut self, next: &ast::Next, indent: usize) -> IoResult<()> {
        fmt_indent(&mut self.output, indent)?;
        writeln!(&mut self.output, "next {}", next.iterator.name)
    }

    fn beautify_null(&mut self, null: &ast::Null, indent: usize) -> IoResult<()> {
        fmt_indent(&mut self.output, indent)?;
        // emits correct text
        writeln!(&mut self.output, "{}", null)
    }

    fn beautify_error(&mut self, indent: usize) -> IoResult<()> {
        fmt_indent(&mut self.output, indent)?;
        writeln!(&mut self.output, "error")
    }
}

#[cfg(test)]
mod tests;
