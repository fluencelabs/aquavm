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

use crate::air::ExecutionCtx;
use crate::call_evidence::merge_call_paths;
use crate::call_evidence::CallEvidenceCtx;
use crate::get_current_peer_id;
use crate::log_targets::RUN_PARAMS;
use crate::AquamarineError;
use crate::CallEvidencePath;
use crate::Result;

use air_parser::ast::Instruction;

/// Parse and prepare supplied data and aqua script.
pub(super) fn prepare<'i>(
    raw_prev_path: String,
    raw_path: String,
    raw_aqua: &'i str,
) -> Result<(CallEvidencePath, CallEvidencePath, Instruction<'i>)> {
    fn to_evidence_path(raw_path: String) -> Result<CallEvidencePath> {
        use AquamarineError::CallEvidenceDeserializationError as CallDeError;

        if raw_path.is_empty() {
            Ok(CallEvidencePath::new())
        } else {
            serde_json::from_str(&raw_path).map_err(|err| CallDeError(err, raw_path))
        }
    }

    let prev_path = to_evidence_path(raw_prev_path)?;
    let path = to_evidence_path(raw_path)?;

    let aqua: Instruction<'i> = *air_parser::parse(raw_aqua).map_err(|msg| AquamarineError::AIRParseError(msg))?;

    log::info!(
        target: RUN_PARAMS,
        "aqua: {:?}\nprev_path: {:?}\ncurrent_path: {:?}",
        aqua,
        prev_path,
        path
    );

    Ok((prev_path, path, aqua))
}

/// Make execution and call evidence contexts from supplied data.
/// Internally, it unites variable from previous and current data and merges call evidence paths.
pub(super) fn make_contexts(
    prev_path: CallEvidencePath,
    path: CallEvidencePath,
) -> Result<(ExecutionCtx<'static>, CallEvidenceCtx)> {
    use AquamarineError::CurrentPeerIdEnvError as EnvError;

    let current_peer_id = get_current_peer_id().map_err(|e| EnvError(e, String::from("CURRENT_PEER_ID")))?;
    log::info!(target: RUN_PARAMS, "current peer id {}", current_peer_id);

    let exec_ctx = ExecutionCtx::new(current_peer_id);
    let current_path = merge_call_paths(prev_path, path)?;
    let call_evidence_ctx = CallEvidenceCtx::new(current_path);

    Ok((exec_ctx, call_evidence_ctx))
}
