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

pub(self) use crate::stepper::ExecutableInstruction;
use crate::AquaData;
use crate::Result;
pub(crate) use call::Call;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Instruction {
    Null,
    Call(Call),
    /*
    Par(Box<Instruction>, Box<Instruction>),
    Seq(Box<Instruction>, Box<Instruction>),

     */
}

impl ExecutableInstruction for Instruction {
    fn execute(self, data: &mut AquaData) -> Result<()> {
        match self {
            Instruction::Null => Ok(()),
            Instruction::Call(call) => call.execute(data),
        }
    }
}
