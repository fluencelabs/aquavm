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
mod null;
mod par;
mod seq;

use crate::AquaData;
use crate::Result;
use call::Call;
use fold::Fold;
use fold::FoldState;
use fold::Next;
use null::Null;
use par::Par;
use seq::Seq;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::collections::HashMap;

#[derive(Clone, Default, Debug)]
pub(super) struct ExecutionContext {
    pub data: AquaData,
    pub next_peer_pks: Vec<String>,
    pub folds: HashMap<String, FoldState>,
}

impl ExecutionContext {
    pub(super) fn new(data: AquaData) -> Self {
        Self {
            data,
            next_peer_pks: vec![],
            folds: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Instruction {
    Null(Null),
    Call(Call),
    Fold(Fold),
    Next(Next),
    Par(Par),
    Seq(Seq),
}

pub(crate) trait ExecutableInstruction {
    fn execute(&self, ctx: &mut ExecutionContext) -> Result<()>;
}

impl ExecutableInstruction for Instruction {
    fn execute(&self, ctx: &mut ExecutionContext) -> Result<()> {
        match self {
            Instruction::Null(null) => null.execute(ctx),
            Instruction::Call(call) => call.execute(ctx),
            Instruction::Fold(fold) => fold.execute(ctx),
            Instruction::Next(next) => next.execute(ctx),
            Instruction::Par(par) => par.execute(ctx),
            Instruction::Seq(seq) => seq.execute(ctx),
        }
    }
}
