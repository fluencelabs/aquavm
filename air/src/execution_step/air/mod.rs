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

mod ap;
mod call;
mod compare_matchable;
mod fail;
mod fold;
mod fold_scalar;
mod fold_stream;
mod match_;
mod mismatch;
mod new;
mod next;
mod null;
mod par;
mod seq;
mod xor;

pub(crate) use fold::FoldState;

use super::boxed_value::ScalarRef;
use super::boxed_value::ValueAggregate;
use super::ExecutionCtx;
use super::ExecutionError;
use super::ExecutionResult;
use crate::execution_step::TraceHandler;

use air_parser::ast::Instruction;

// TODO: move all error set logic from macros into the execution context

/// Executes instruction and updates last error if needed.
macro_rules! execute {
    ($self:expr, $instr:expr, $exec_ctx:ident, $trace_ctx:ident) => {{
        match $instr.execute($exec_ctx, $trace_ctx) {
            Err(e) => {
                $exec_ctx.last_error_descriptor.try_to_set_from_error(
                    &e,
                    // TODO: avoid excess copying here
                    &$instr.to_string(),
                    $exec_ctx.run_parameters.current_peer_id.as_ref(),
                    None,
                );
                Err(e)
            }
            v => v,
        }
    }};
}

pub(crate) trait ExecutableInstruction<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()>;
}

impl<'i> ExecutableInstruction<'i> for Instruction<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        match self {
            // call isn't wrapped by the execute macro because
            // it internally sets last_error with resolved triplet
            Instruction::Call(call) => call.execute(exec_ctx, trace_ctx),

            Instruction::Ap(ap) => execute!(self, ap, exec_ctx, trace_ctx),
            Instruction::Fail(fail) => execute!(self, fail, exec_ctx, trace_ctx),
            Instruction::FoldScalar(fold) => execute!(self, fold, exec_ctx, trace_ctx),
            Instruction::FoldStream(fold) => execute!(self, fold, exec_ctx, trace_ctx),
            Instruction::New(new) => execute!(self, new, exec_ctx, trace_ctx),
            Instruction::Next(next) => execute!(self, next, exec_ctx, trace_ctx),
            Instruction::Null(null) => execute!(self, null, exec_ctx, trace_ctx),
            Instruction::Par(par) => execute!(self, par, exec_ctx, trace_ctx),
            Instruction::Seq(seq) => execute!(self, seq, exec_ctx, trace_ctx),
            Instruction::Xor(xor) => execute!(self, xor, exec_ctx, trace_ctx),
            Instruction::Match(match_) => execute!(self, match_, exec_ctx, trace_ctx),
            Instruction::MisMatch(mismatch) => execute!(self, mismatch, exec_ctx, trace_ctx),

            Instruction::Error => unreachable!("should not execute if parsing succeeded. QED."),
        }
    }
}

#[macro_export]
macro_rules! log_instruction {
    ($instr_name:expr, $exec_ctx:expr, $trace_ctx:expr) => {
        log::debug!(target: air_log_targets::INSTRUCTION, "> {}", stringify!($instr_name));

        log::trace!(
            target: air_log_targets::DATA_CACHE,
            "  scalars:
    {}
  streams:
    {}",
            $exec_ctx.scalars,
            $exec_ctx.streams
        );
        log::trace!(
            target: air_log_targets::NEXT_PEER_PKS,
            "  next peers pk: {:?}",
            $exec_ctx.next_peer_pks
        );
        log::trace!(
            target: air_log_targets::SUBGRAPH_COMPLETE,
            "  subgraph complete: {}",
            $exec_ctx.subgraph_complete
        );

        log::trace!(
            target: air_log_targets::SUBGRAPH_ELEMENTS,
            "  subgraph elements count: {:?}",
            $trace_ctx.subgraph_sizes()
        );
        log::debug!(
            target: air_log_targets::NEW_EXECUTED_TRACE,
            "  new call executed trace: {:?}",
            $trace_ctx.as_result_trace()
        );
    };
}

/// This macro converts joinable errors to Ok and sets subgraph complete to false.
#[macro_export]
macro_rules! joinable {
    ($cmd:expr, $exec_ctx:expr) => {
        match $cmd {
            Err(e) if e.is_joinable() => {
                $exec_ctx.subgraph_complete = false;
                return Ok(());
            }
            v => v,
        }
    };
}
