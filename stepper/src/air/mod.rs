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
mod seq;
mod xor;

pub(crate) use execution_context::ExecutionCtx;

pub(self) use crate::call_evidence::CallEvidenceCtx;
pub(self) use crate::call_evidence::EvidenceState;

use crate::Result;
use call::Call;
use fold::Fold;
use fold::Next;
use null::Null;
use par::Par;
use seq::Seq;
use xor::Xor;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use once_cell::sync::Lazy;
use std::collections::HashSet;

pub(self) static RESERVED_KEYWORDS: Lazy<HashSet<&str>> = Lazy::new(|| {
    let mut set = HashSet::new();
    set.insert("__call");
    set
});

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Instruction {
    Null(Null),
    Call(Call),
    Fold(Fold),
    Next(Next),
    Par(Par),
    Seq(Seq),
    Xor(Xor),
}

pub(crate) trait ExecutableInstruction {
    fn execute(&self, exec_ctx: &mut ExecutionCtx, call_ctx: &mut CallEvidenceCtx) -> Result<()>;
}

impl ExecutableInstruction for Instruction {
    fn execute(&self, exec_ctx: &mut ExecutionCtx, call_ctx: &mut CallEvidenceCtx) -> Result<()> {
        match self {
            Instruction::Null(null) => null.execute(exec_ctx, call_ctx),
            Instruction::Call(call) => call.execute(exec_ctx, call_ctx),
            Instruction::Fold(fold) => fold.execute(exec_ctx, call_ctx),
            Instruction::Next(next) => next.execute(exec_ctx, call_ctx),
            Instruction::Par(par) => par.execute(exec_ctx, call_ctx),
            Instruction::Seq(seq) => seq.execute(exec_ctx, call_ctx),
            Instruction::Xor(xor) => xor.execute(exec_ctx, call_ctx),
        }
    }
}
