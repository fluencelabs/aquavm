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

mod outcome;
mod preparation;

pub use preparation::parse;

use preparation::prepare;
use preparation::PrepareDescriptor;

use crate::air::ExecutableInstruction;
use stepper_interface::StepperOutcome;

use std::convert::identity;

pub fn execute_aqua(init_peer_id: String, aqua: String, prev_data: Vec<u8>, data: Vec<u8>) -> StepperOutcome {
    log::trace!(
        "aquamarine version is {}, init user id is {}",
        env!("CARGO_PKG_VERSION"),
        init_peer_id
    );

    execute_aqua_impl(init_peer_id, aqua, prev_data, data).unwrap_or_else(identity)
}

fn execute_aqua_impl(
    init_peer_id: String,
    aqua: String,
    prev_path: Vec<u8>,
    path: Vec<u8>,
) -> Result<StepperOutcome, StepperOutcome> {
    let PrepareDescriptor {
        mut exec_ctx,
        mut call_ctx,
        aqua,
    } = prepare(&prev_path, &path, aqua.as_str(), init_peer_id)
        // return the initial data in case of errors
        .map_err(|e| outcome::error_from_raw_data(path, e))?;

    aqua.execute(&mut exec_ctx, &mut call_ctx)
        // return new collected path in case of errors
        .map_err(|e| outcome::error_from_data(&call_ctx.new_path, exec_ctx.next_peer_pks.clone(), e))?;

    let outcome = outcome::success(&call_ctx.new_path, exec_ctx.next_peer_pks);

    Ok(outcome)
}
