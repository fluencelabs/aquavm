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

mod prolog;
mod utils;

use prolog::make_contexts;
use prolog::prepare;
use utils::dedup;

use crate::air::ExecutableInstruction;
use crate::AquamarineError::CallEvidenceSerializationError as CallSeError;
use crate::Result;
use crate::StepperOutcome;
use crate::STEPPER_SUCCESS;

pub use prolog::parse;

pub fn execute_aqua(init_peer_id: String, aqua: String, prev_data: String, data: String) -> StepperOutcome {
    log::info!(
        "aquamarine version is {}, init user id is {}",
        env!("CARGO_PKG_VERSION"),
        init_peer_id
    );

    execute_aqua_impl(init_peer_id, aqua, prev_data, data).unwrap_or_else(Into::into)
}

fn execute_aqua_impl(init_peer_id: String, aqua: String, prev_path: String, path: String) -> Result<StepperOutcome> {
    let (prev_path, path, aqua) = prepare(prev_path, path, aqua.as_str())?;
    let (mut exec_ctx, mut call_ctx) = make_contexts(prev_path, path, init_peer_id)?;

    aqua.execute(&mut exec_ctx, &mut call_ctx)?;

    let next_peer_pks = dedup(exec_ctx.next_peer_pks);
    let serialized_call_path = serde_json::to_string(&call_ctx.new_path).map_err(CallSeError)?;

    Ok(StepperOutcome {
        ret_code: STEPPER_SUCCESS,
        call_path: serialized_call_path,
        next_peer_pks,
    })
}
