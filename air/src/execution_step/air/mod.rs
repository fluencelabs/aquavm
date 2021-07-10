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
mod compare_matchable;
mod fold;
mod fold_scalar;
mod fold_stream;
mod match_;
mod mismatch;
mod next;
mod null;
mod par;
mod seq;
mod xor;

pub(crate) use fold::FoldState;

pub(self) use super::execution_context::*;
pub(self) use super::ExecutionCtx;
pub(self) use super::ExecutionError;
pub(self) use super::ExecutionResult;
pub(self) use crate::execution_step::TraceHandler;

use air_parser::ast::Instruction;

/// Executes instruction and updates last error if needed.
macro_rules! execute {
    ($self:expr, $instr:expr, $exec_ctx:ident, $trace_ctx:ident) => {
        match $instr.execute($exec_ctx, $trace_ctx) {
            Err(e) => {
                if !$exec_ctx.last_error_could_be_set {
                    return Err(e);
                }

                let instruction = format!("{}", $self);
                let last_error =
                    LastErrorDescriptor::new(e.clone(), instruction, $exec_ctx.current_peer_id.to_string(), None);
                $exec_ctx.last_error = Some(last_error);
                Err(e)
            }
            v => v,
        }
    };
}

/// Executes instruction, updates last error if needed, and call error_exit of TraceHandler.
macro_rules! execute_with_error_exit {
    ($self:expr, $instr:expr, $exec_ctx:ident, $trace_ctx:ident) => {
        match $instr.execute($exec_ctx, $trace_ctx) {
            Err(e) => {
                $trace_ctx.error_exit();

                if !$exec_ctx.last_error_could_be_set {
                    return Err(e);
                }

                let instruction = format!("{}", $self);
                let last_error =
                    LastErrorDescriptor::new(e.clone(), instruction, $exec_ctx.current_peer_id.to_string(), None);
                $exec_ctx.last_error = Some(last_error);
                Err(e)
            }
            v => v,
        }
    };
}

/// Executes match/mismatch instructions and updates last error if error type wasn't
/// MatchWithoutXorError or MismatchWithoutXorError.
macro_rules! execute_match_mismatch {
    ($self:expr, $instr:expr, $exec_ctx:ident, $trace_ctx:ident) => {
        match $instr.execute($exec_ctx, $trace_ctx) {
            Err(e) => {
                use std::borrow::Borrow;

                if !$exec_ctx.last_error_could_be_set
                    || matches!(&*e.borrow(), ExecutionError::MatchWithoutXorError)
                    || matches!(&*e.borrow(), ExecutionError::MismatchWithoutXorError)
                {
                    return Err(e);
                }

                let instruction = format!("{}", $self);
                let last_error =
                    LastErrorDescriptor::new(e.clone(), instruction, $exec_ctx.current_peer_id.to_string(), None);
                $exec_ctx.last_error = Some(last_error);
                Err(e)
            }
            v => v,
        }
    };
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

            Instruction::FoldScalar(fold) => execute!(self, fold, exec_ctx, trace_ctx),
            Instruction::FoldStream(fold) => execute_with_error_exit!(self, fold, exec_ctx, trace_ctx),
            Instruction::Next(next) => execute!(self, next, exec_ctx, trace_ctx),
            Instruction::Null(null) => execute!(self, null, exec_ctx, trace_ctx),
            Instruction::Par(par) => execute_with_error_exit!(self, par, exec_ctx, trace_ctx),
            Instruction::Seq(seq) => execute!(self, seq, exec_ctx, trace_ctx),
            Instruction::Xor(xor) => execute!(self, xor, exec_ctx, trace_ctx),

            // match/mismatch shouldn't rewrite last_error
            Instruction::Match(match_) => execute_match_mismatch!(self, match_, exec_ctx, trace_ctx),
            Instruction::MisMatch(mismatch) => execute_match_mismatch!(self, mismatch, exec_ctx, trace_ctx),

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

        log::trace!(
            target: crate::log_targets::SUBTREE_ELEMENTS,
            "  subtree elements count: {:?}",
            $trace_ctx.subtree_sizes()
        );
        log::debug!(
            target: crate::log_targets::NEW_EXECUTED_TRACE,
            "  new call executed trace: {:?}",
            $trace_ctx.as_result_trace()
        );
    };
}

/// This macro converts joinable errors to Ok and sets subtree complete to false.
#[macro_export]
macro_rules! joinable_call {
    ($cmd:expr, $exec_ctx:expr) => {
        match $cmd {
            Err(e) if e.is_joinable() => {
                $exec_ctx.subtree_complete = false;
                return Ok(());
            }
            v => v,
        }
    };
}

/// This macro converts joinable errors to Ok.
#[macro_export]
macro_rules! joinable {
    ($cmd:expr, $exec_ctx:expr) => {
        match $cmd {
            Err(e) if e.is_joinable() => {
                return Ok(());
            }
            v => v,
        }
    };
}
