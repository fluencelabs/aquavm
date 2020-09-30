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

use crate::instructions::Instruction;
use crate::AquaData;
use crate::Result;

pub(crate) trait ExecutableInstruction {
    fn execute(self, data: &mut AquaData, next_peer_pks: &mut Vec<String>) -> Result<()>;
}

pub(crate) fn execute(instructions: Vec<Instruction>, data: &mut AquaData) -> Result<Vec<String>> {
    let mut next_peer_pks = Vec::new();

    for instruction in instructions {
        instruction.execute(data, &mut next_peer_pks)?;
    }

    Ok(next_peer_pks)
}
