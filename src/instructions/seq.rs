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

use super::ExecutionContext;
use super::Instruction;
use crate::Result;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct Seq(Box<Instruction>, Box<Instruction>);

impl super::ExecutableInstruction for Seq {
    fn execute(&self, ctx: &mut ExecutionContext) -> Result<()> {
        log::info!("seq is called with context: {:?}", ctx);

        let pks_count_before_call = ctx.next_peer_pks.len();
        self.0.execute(ctx)?;

        if pks_count_before_call == ctx.next_peer_pks.len() {
            self.1.execute(ctx)?;
        }

        Ok(())
    }
}
