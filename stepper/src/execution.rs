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

mod epilog;
mod prolog;
mod utils;

use epilog::make_result_data;
use prolog::make_contexts;
use prolog::prepare;
use utils::dedup;

use crate::air::ExecutableInstruction;
use crate::Result;
use crate::StepperOutcome;
use crate::STEPPER_SUCCESS;

pub(self) const CALL_EVIDENCE_CTX_KEY: &str = "__call";

pub(crate) fn execute_aqua(init_user_id: String, aqua: String, prev_data: String, data: String) -> StepperOutcome {
    log::info!("aquamarine version is {}", env!("CARGO_PKG_VERSION"));

    log::info!(
        "stepper invoked with user_id = {}, aqua = {:?}, prev_data = {:?}, data = {:?}",
        init_user_id,
        aqua,
        prev_data,
        data
    );

    execute_aqua_impl(init_user_id, aqua, prev_data, data).unwrap_or_else(Into::into)
}

fn execute_aqua_impl(_init_user_id: String, aqua: String, prev_data: String, data: String) -> Result<StepperOutcome> {
    let (prev_data, data, aqua) = prepare(prev_data, data, aqua)?;
    let (mut exec_ctx, mut call_ctx) = make_contexts(prev_data, data)?;

    aqua.execute(&mut exec_ctx, &mut call_ctx)?;

    let data = make_result_data(exec_ctx.data, call_ctx)?;

    Ok(StepperOutcome {
        ret_code: STEPPER_SUCCESS,
        data,
        next_peer_pks: dedup(exec_ctx.next_peer_pks),
    })
}
