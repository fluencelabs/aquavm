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

use super::utils::format_aqua;
use crate::air::ExecutionCtx;
use crate::air::Instruction;
use crate::call_evidence::merge_call_paths;
use crate::call_evidence::CallEvidenceCtx;
use crate::get_current_peer_id;
use crate::AquamarineError;
use crate::CallEvidencePath;
use crate::Result;

/// Parse and prepare supplied data and aqua script.
pub(super) fn prepare(
    raw_prev_path: String,
    raw_path: String,
    raw_aqua: String,
) -> Result<(CallEvidencePath, CallEvidencePath, Instruction)> {
    use AquamarineError::DataDeserializationError as DataDeError;

    let prev_path: CallEvidencePath = serde_json::from_str(&raw_prev_path).map_err(DataDeError)?;
    let path: CallEvidencePath = serde_json::from_str(&raw_path).map_err(DataDeError)?;

    let formatted_aqua = format_aqua(raw_aqua);
    let aqua: Instruction = serde_sexpr::from_str(&formatted_aqua)?;

    Ok((prev_path, path, aqua))
}

/// Make execution and call evidence contexts from supplied data.
/// Internally, it unites variable from previous and current data and merges call evidence paths.
pub(super) fn make_contexts<'a>(
    prev_path: CallEvidencePath,
    path: CallEvidencePath,
) -> Result<(ExecutionCtx<'a>, CallEvidenceCtx)> {
    use AquamarineError::CurrentPeerIdEnvError as EnvError;

    let current_peer_id = get_current_peer_id().map_err(|e| EnvError(e, String::from("CURRENT_PEER_ID")))?;
    let exec_ctx = ExecutionCtx::new(current_peer_id);

    let current_path = merge_call_paths(prev_path, path)?;

    let call_evidence_ctx = CallEvidenceCtx::new(current_path);

    Ok((exec_ctx, call_evidence_ctx))
}
