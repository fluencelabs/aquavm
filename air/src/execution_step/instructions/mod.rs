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

mod ap;
mod ap_map;
mod call;
mod canon;
mod canon_map;
mod canon_stream_map_scalar;
mod canon_utils;
mod compare_matchable;
mod fail;
mod fold;
mod fold_scalar;
mod fold_stream;
mod fold_stream_map;
mod match_;
mod mismatch;
mod never;
mod new;
mod next;
mod null;
mod par;
mod seq;
mod xor;

pub(crate) use call::triplet::resolve_peer_id_to_string;
pub(crate) use fold::FoldState;

use super::value_types::ScalarRef;
use super::ExecutionCtx;
use super::ExecutionError;
use super::ExecutionResult;
use crate::execution_step::TraceHandler;

use air_parser::ast::Instruction;
use air_parser::ast::PeerIDErrorLogable;

/// Executes an instruction and updates %last_error% and :error: if necessary.
macro_rules! execute {
    ($self:expr, $instr:expr, $exec_ctx:ident, $trace_ctx:ident) => {{
        match $instr.execute($exec_ctx, $trace_ctx) {
            Err(e) => {
                $exec_ctx.set_errors(&e, &$instr.to_string(), None, $instr.log_errors_with_peer_id());
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
            // it internally maps some Catchables into %last_error%/:error: using resolved triplet.
            // Both canons and call set :error:.$.peer_id whilst other instructions do not.
            Instruction::Call(call) => call.execute(exec_ctx, trace_ctx),

            Instruction::Canon(canon) => execute!(self, canon, exec_ctx, trace_ctx),
            Instruction::CanonMap(canon_map) => execute!(self, canon_map, exec_ctx, trace_ctx),
            Instruction::CanonStreamMapScalar(canon) => execute!(self, canon, exec_ctx, trace_ctx),
            Instruction::Ap(ap) => execute!(self, ap, exec_ctx, trace_ctx),
            Instruction::ApMap(ap_map) => execute!(self, ap_map, exec_ctx, trace_ctx),
            Instruction::Fail(fail) => execute!(self, fail, exec_ctx, trace_ctx),
            Instruction::FoldScalar(fold) => execute!(self, fold, exec_ctx, trace_ctx),
            Instruction::FoldStream(fold) => execute!(self, fold, exec_ctx, trace_ctx),
            Instruction::FoldStreamMap(fold) => execute!(self, fold, exec_ctx, trace_ctx),
            Instruction::Never(never) => execute!(self, never, exec_ctx, trace_ctx),
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
        log::debug!(
            target: air_log_targets::INSTRUCTION,
            "> {}",
            stringify!($instr_name)
        );

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
            $exec_ctx.is_subgraph_complete()
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
    ($cmd:expr, $exec_ctx:expr, $ok_result:expr) => {
        match $cmd {
            Err(e) if e.is_joinable() => {
                $exec_ctx.make_subgraph_incomplete();
                return Ok($ok_result);
            }
            v => v,
        }
    };
}
