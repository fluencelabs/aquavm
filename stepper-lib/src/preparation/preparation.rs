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

pub(super) type PrepareResult<T> = Result<T, PrepareError>;
/// Represents result of the preparation step.
pub(super) struct PrepareDescriptor<'ctx, 'i> {
    pub(crate) exec_ctx: ExecutionCtx<'ctx>,
    pub(crate) call_ctx: CallEvidenceCtx,
    pub(crate) aqua: Instruction<'i>,
}

/// Parse and prepare supplied data and aqua script.
pub(super) fn prepare<'i>(
    raw_prev_path: &[u8],
    raw_path: &[u8],
    raw_aqua: &'i str,
    init_peer_id: String,
) -> Result<PrepareDescriptor<'static, 'i>> {
    fn to_evidence_path(raw_path: &[u8]) -> Result<CallEvidencePath> {
        use AquamarineError::CallEvidenceDeserializationError as CallDeError;

        // treat empty string as an empty call evidence path allows abstracting from
        // the internal format for empty data.
        if raw_path.is_empty() {
            Ok(CallEvidencePath::new())
        } else {
            serde_json::from_slice(&raw_path).map_err(|err| CallDeError(err, raw_path.to_vec()))
        }
    }

    let prev_path = to_evidence_path(raw_prev_path)?;
    let path = to_evidence_path(raw_path)?;

    let aqua: Instruction<'i> = *air_parser::parse(raw_aqua).map_err(AquamarineError::AIRParseError)?;

    log::trace!(
        target: RUN_PARAMS,
        "aqua: {:?}\nprev_path: {:?}\ncurrent_path: {:?}",
        aqua,
        prev_path,
        path
    );

    let (exec_ctx, call_ctx) = make_contexts(prev_path, path, init_peer_id)?;
    let result = PrepareDescriptor {
        exec_ctx,
        call_ctx,
        aqua,
    };

    Ok(result)
}

/// Make execution and call evidence contexts from supplied data.
/// Internally, it unites variable from previous and current data and merges call evidence paths.
fn make_contexts(
    prev_path: CallEvidencePath,
    path: CallEvidencePath,
    init_peer_id: String,
) -> Result<(ExecutionCtx<'static>, CallEvidenceCtx)> {
    use AquamarineError::CurrentPeerIdEnvError as EnvError;

    let current_peer_id = get_current_peer_id().map_err(|e| EnvError(e, String::from("CURRENT_PEER_ID")))?;
    log::trace!(target: RUN_PARAMS, "current peer id {}", current_peer_id);

    let exec_ctx = ExecutionCtx::new(current_peer_id, init_peer_id);
    let current_path = merge_call_paths(prev_path, path)?;
    let call_evidence_ctx = CallEvidenceCtx::new(current_path);

    Ok((exec_ctx, call_evidence_ctx))
}

/// Parse an AIR script to AST.
pub fn parse(script: &str) -> Result<Instruction<'_>> {
    let ast = air_parser::parse(script).map_err(AquamarineError::AIRParseError)?;
    Ok(*ast)
}
