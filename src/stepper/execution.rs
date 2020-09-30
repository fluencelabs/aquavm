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

use super::Result;
use super::StepperOutcome;
use crate::instructions::Instruction;

pub(crate) fn execute_aqua(init_user_id: String, aqua: String, data: String) -> StepperOutcome {
    log::info!(
        "stepper invoked with user_id = {}, aqua = {:?}, data = {:?}",
        init_user_id,
        aqua,
        data
    );

    execute_aqua_impl(init_user_id, aqua, data).unwrap_or_else(Into::into)
}

fn execute_aqua_impl(init_user_id: String, aqua: String, data: String) -> Result<StepperOutcome> {
    let parsed_aqua = serde_sexpr::from_str::<Vec<Instruction>>(&aqua)?;

    log::info!("parsed_aqua: {:?}", parsed_aqua);
    super::stepper::execute(parsed_aqua);

    Ok(StepperOutcome {
        ret_code: 0,
        data,
        next_peer_pks: vec![init_user_id],
    })
}
