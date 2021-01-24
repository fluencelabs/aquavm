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

mod call;
mod fold;
mod match_;
mod null;
mod par;
mod seq;
mod xor;

pub(crate) use fold::FoldState;

pub(self) use super::ExecutionError;
pub(self) use super::ExecutionResult;
pub(self) use crate::contexts::execution::ExecutionCtx;
pub(self) use crate::contexts::execution_trace::ExecutionTraceCtx;

use air_parser::ast::Instruction;

pub(crate) trait ExecutableInstruction<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut ExecutionTraceCtx) -> ExecutionResult<()>;
}

impl<'i> ExecutableInstruction<'i> for Instruction<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut ExecutionTraceCtx) -> ExecutionResult<()> {
        match self {
            Instruction::Call(call) => call.execute(exec_ctx, trace_ctx),
            Instruction::Fold(fold) => fold.execute(exec_ctx, trace_ctx),
            Instruction::Next(next) => next.execute(exec_ctx, trace_ctx),
            Instruction::Null(null) => null.execute(exec_ctx, trace_ctx),
            Instruction::Par(par) => par.execute(exec_ctx, trace_ctx),
            Instruction::Seq(seq) => seq.execute(exec_ctx, trace_ctx),
            Instruction::Xor(xor) => xor.execute(exec_ctx, trace_ctx),
            Instruction::Match(match_) => match_.execute(exec_ctx, trace_ctx),
            Instruction::Error => unreachable!("should not execute if parsing succeeded. QED."),
        }
    }
}

#[macro_export]
macro_rules! log_instruction {
    ($instr_name:expr, $exec_ctx:expr, $trace_ctx:expr) => {
        log::debug!(target: crate::log_targets::INSTRUCTION, "> {}", stringify!($instr_name));

        let mut data_cache_log = String::from("  data cache:");
        if $exec_ctx.data_cache.is_empty() {
            data_cache_log.push_str(" empty");
        }
        for (key, value) in $exec_ctx.data_cache.iter() {
            data_cache_log.push_str(&format!("\n    {} => {}", key, value));
        }

        log::trace!(target: crate::log_targets::DATA_CACHE, "{}", data_cache_log);
        log::trace!(
            target: crate::log_targets::NEXT_PEER_PKS,
            "  next peers pk: {:?}",
            $exec_ctx.next_peer_pks
        );
        log::trace!(
            target: crate::log_targets::SUBTREE_COMPLETE,
            "  subtree complete: {}",
            $exec_ctx.subtree_complete
        );

        log::debug!(
            target: crate::log_targets::EXECUTED_TRACE,
            "  current call executed trace: {:?}",
            $trace_ctx.current_trace
        );
        log::trace!(
            target: crate::log_targets::SUBTREE_ELEMENTS,
            "  subtree elements count: {:?}",
            $trace_ctx.current_subtree_size
        );
        log::debug!(
            target: crate::log_targets::NEW_EXECUTED_TRACE,
            "  new call executed trace: {:?}",
            $trace_ctx.new_trace
        );
    };
}
