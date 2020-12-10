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
mod execution_context;
mod fold;
mod null;
mod par;
pub(crate) mod resolve;
mod seq;
mod xor;

pub(crate) use call::ResolvedTriplet;
pub(crate) use execution_context::ExecutionCtx;
pub(crate) use fold::FoldState;

pub(self) use crate::call_evidence::CallEvidenceCtx;
pub(self) use crate::call_evidence::EvidenceState;
pub(self) use jvaluable_result::JValuableResult;

use crate::Result;

use air_parser::ast::Instruction;

pub(crate) trait ExecutableInstruction<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, call_ctx: &mut CallEvidenceCtx) -> Result<()>;
}

impl<'i> ExecutableInstruction<'i> for Instruction<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, call_ctx: &mut CallEvidenceCtx) -> Result<()> {
        match self {
            Instruction::Call(call) => call.execute(exec_ctx, call_ctx),
            Instruction::Fold(fold) => fold.execute(exec_ctx, call_ctx),
            Instruction::Next(next) => next.execute(exec_ctx, call_ctx),
            Instruction::Null(null) => null.execute(exec_ctx, call_ctx),
            Instruction::Par(par) => par.execute(exec_ctx, call_ctx),
            Instruction::Seq(seq) => seq.execute(exec_ctx, call_ctx),
            Instruction::Xor(xor) => xor.execute(exec_ctx, call_ctx),
            Instruction::Error => unreachable!("should not execute if parsing failed. QED."),
        }
    }
}

#[macro_export]
macro_rules! log_instruction {
    ($instr_name:expr, $exec_ctx:expr, $call_ctx:expr) => {
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
            target: crate::log_targets::CALL_EVIDENCE_PATH,
            "  current call evidence path: {:?}",
            $call_ctx.current_path
        );
        log::trace!(
            target: crate::log_targets::SUBTREE_ELEMENTS,
            "  subtree elements count: {:?}",
            $call_ctx.current_subtree_size
        );
        log::debug!(
            target: crate::log_targets::NEW_CALL_EVIDENCE_PATH,
            "  new call evidence path: {:?}",
            $call_ctx.new_path
        );
    };
}
