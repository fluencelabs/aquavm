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

mod preparation;

use preparation::prepare;
use preparation::PrepareResult;

use crate::air::ExecutableInstruction;
use crate::StepperOutcome;

pub use preparation::parse;

pub fn execute_aqua(init_peer_id: String, aqua: String, prev_data: String, data: String) -> StepperOutcome {
    log::trace!(
        "aquamarine version is {}, init user id is {}",
        env!("CARGO_PKG_VERSION"),
        init_peer_id
    );

    execute_aqua_impl(init_peer_id, aqua, prev_data, data).unwrap_or_else(Into::into)
}

fn execute_aqua_impl(
    init_peer_id: String,
    aqua: String,
    prev_path: String,
    path: String,
) -> Result<StepperOutcome, StepperOutcome> {
    let PrepareResult {
        mut exec_ctx,
        mut call_ctx,
        aqua,
    } = prepare(prev_path, path.clone(), aqua.as_str(), init_peer_id).map_err(|e| StepperOutcome::error_from_data(path, e))?;

    aqua.execute(&mut exec_ctx, &mut call_ctx)
        .map_err(|e| StepperOutcome::error_from_ctxs(exec_ctx.clone(), &call_ctx, e))?;

    let outcome = StepperOutcome::success(exec_ctx, &call_ctx);

    Ok(outcome)
}
