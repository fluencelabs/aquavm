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

use air_parser::ast;

use std::fmt::Display;
use std::io;

pub const DEFAULT_INDENT_STEP: usize = 4;

macro_rules! multiline {
    ($beautifier:expr, $indent:expr $(; $fmt1:literal $(, $arg1:expr)*; $nest:expr)+) => ({
        let indent_step = $beautifier.indent_step;
        $({
              let out = &mut $beautifier.output;
              $crate::beautifier::fmt_indent(out, $indent)?;
              writeln!(out, $fmt1 $(, $arg1)*)?;
          }
          $crate::beautifier::Beautifier::beautify_walker($beautifier, $nest, $indent + indent_step)?;
        )+
    });
}

macro_rules! compound {
    ($beautifier:expr, $indent:expr, $instruction:expr) => ({
        multiline!(
            $beautifier, $indent;
            "{}:", $instruction;
            &$instruction.instruction
        )
    });
}

fn fmt_indent(output: &mut impl io::Write, indent: usize) -> io::Result<()> {
    write!(output, "{:indent$}", "", indent = indent)
}

struct CallArgs<'ctx, 'i>(&'ctx [ast::ImmutableValue<'i>]);

impl<'ctx, 'i> Display for CallArgs<'ctx, 'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use itertools::Itertools as _;

        f.write_fmt(format_args!("{}", self.0.iter().format(", ")))
    }
}

struct CallTriplet<'ctx, 'i>(&'ctx ast::Triplet<'i>);

impl<'ctx, 'i> Display for CallTriplet<'ctx, 'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} ({}, {})",
            self.0.peer_id, self.0.service_id, self.0.function_name
        ))
    }
}

/// Error produced by the Beautifier.
#[derive(Debug, thiserror::Error)]
pub enum BeautifyError {
    #[error("{0}")]
    Parse(String),
    #[error(transparent)]
    Io(#[from] io::Error),
}

/// AIR beautifier.
pub struct Beautifier<W: io::Write> {
    output: W,
    indent_step: usize,
}

impl<W: io::Write> Beautifier<W> {
    /// Beautifier for the output with default indentation step.
    pub fn new(output: W) -> Self {
        Self {
            output,
            indent_step: DEFAULT_INDENT_STEP,
        }
    }

    /// Beautifier for the output with custom indentation step.
    pub fn new_with_indent(output: W, indent_step: usize) -> Self {
        Self {
            output,
            indent_step,
        }
    }

    /// Unwrap the Beautifier into the underlying writer.
    pub fn into_inner(self) -> W {
        self.output
    }

    /// Emit beautified code for the `air_script`.
    pub fn beautify(&mut self, air_script: &str) -> Result<(), BeautifyError> {
        let tree = air_parser::parse(air_script).map_err(BeautifyError::Parse)?;
        self.beautify_ast(&tree)
    }

    /// Emit beautified code for the `ast`.
    pub fn beautify_ast(&mut self, ast: &ast::Instruction<'_>) -> Result<(), BeautifyError> {
        Ok(self.beautify_walker(ast, 0)?)
    }

    fn beautify_walker(&mut self, node: &ast::Instruction<'_>, indent: usize) -> io::Result<()> {
        match node {
            ast::Instruction::Call(call) => self.beautify_call(call, indent),
            ast::Instruction::Ap(ap) => self.beautify_simple(ap, indent),
            ast::Instruction::ApMap(ap_map) => self.beautify_simple(ap_map, indent),
            ast::Instruction::Canon(canon) => self.beautify_simple(canon, indent),
            ast::Instruction::CanonMap(canon_map) => self.beautify_simple(canon_map, indent),
            ast::Instruction::CanonStreamMapScalar(canon) => self.beautify_simple(canon, indent),
            ast::Instruction::Seq(seq) => self.beautify_seq(seq, indent),
            ast::Instruction::Par(par) => self.beautify_par(par, indent),
            ast::Instruction::Xor(xor) => self.beautify_xor(xor, indent),
            ast::Instruction::Match(match_) => self.beautify_match(match_, indent),
            ast::Instruction::MisMatch(mismatch) => self.beautify_mismatch(mismatch, indent),
            ast::Instruction::Fail(fail) => self.beautify_simple(fail, indent),
            ast::Instruction::FoldScalar(fold_scalar) => {
                self.beautify_fold_scalar(fold_scalar, indent)
            }
            ast::Instruction::FoldStream(fold_stream) => {
                self.beautify_fold_stream(fold_stream, indent)
            }
            ast::Instruction::FoldStreamMap(fold_stream_map) => {
                self.beautify_fold_stream_map(fold_stream_map, indent)
            }
            ast::Instruction::Never(never) => self.beautify_simple(never, indent),
            ast::Instruction::New(new) => self.beautify_new(new, indent),
            ast::Instruction::Next(next) => self.beautify_simple(next, indent),
            ast::Instruction::Null(null) => self.beautify_simple(null, indent),
            ast::Instruction::Error => self.beautify_simple("error", indent),
        }
    }

    fn beautify_call(&mut self, call: &ast::Call<'_>, indent: usize) -> io::Result<()> {
        fmt_indent(&mut self.output, indent)?;
        match &call.output {
            ast::CallOutputValue::Scalar(v) => write!(&mut self.output, "{v} <- ")?,
            ast::CallOutputValue::Stream(v) => write!(&mut self.output, "{v} <- ")?,
            ast::CallOutputValue::None => {}
        }
        writeln!(
            &mut self.output,
            "call {} [{}]",
            CallTriplet(&call.triplet),
            CallArgs(call.args.as_slice())
        )
    }

    fn beautify_simple(&mut self, instruction: impl Display, indent: usize) -> io::Result<()> {
        fmt_indent(&mut self.output, indent)?;
        writeln!(&mut self.output, "{instruction}")
    }

    fn beautify_seq(&mut self, seq: &ast::Seq<'_>, indent: usize) -> io::Result<()> {
        self.beautify_walker(&seq.0, indent)?;
        self.beautify_walker(&seq.1, indent)
    }

    fn beautify_par(&mut self, par: &ast::Par<'_>, indent: usize) -> io::Result<()> {
        multiline!(
            self, indent;
            "par:";
            &par.0;
            "|";  // TODO: SHOULD BE UNINDENTED AS PER SPEC; OR WE MAY CHANGE THE SPEC
            &par.1
        );
        Ok(())
    }

    fn beautify_xor(&mut self, xor: &ast::Xor<'_>, indent: usize) -> io::Result<()> {
        multiline!(
            self, indent;
            "try:";
            &xor.0;
            "catch:";
            &xor.1
        );
        Ok(())
    }

    fn beautify_match(&mut self, match_: &ast::Match<'_>, indent: usize) -> io::Result<()> {
        compound!(self, indent, match_);
        Ok(())
    }

    fn beautify_mismatch(&mut self, mismatch: &ast::MisMatch<'_>, indent: usize) -> io::Result<()> {
        compound!(self, indent, mismatch);
        Ok(())
    }

    fn beautify_fold_scalar(
        &mut self,
        fold: &ast::FoldScalar<'_>,
        indent: usize,
    ) -> io::Result<()> {
        compound!(self, indent, fold);
        if let Some(last_instruction) = &fold.last_instruction {
            multiline!(
                self, indent;
                "last:";
                last_instruction);
        }
        Ok(())
    }

    fn beautify_fold_stream(
        &mut self,
        fold: &ast::FoldStream<'_>,
        indent: usize,
    ) -> io::Result<()> {
        compound!(self, indent, fold);
        if let Some(last_instruction) = &fold.last_instruction {
            multiline!(
                self, indent;
                "last:";
                last_instruction
            );
        }
        Ok(())
    }

    fn beautify_fold_stream_map(
        &mut self,
        fold: &ast::FoldStreamMap<'_>,
        indent: usize,
    ) -> io::Result<()> {
        compound!(self, indent, fold);
        if let Some(last_instruction) = &fold.last_instruction {
            multiline!(
                self, indent;
                "last:";
                last_instruction
            );
        }
        Ok(())
    }

    fn beautify_new(&mut self, new: &ast::New<'_>, indent: usize) -> io::Result<()> {
        compound!(self, indent, new);
        Ok(())
    }
}
