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

use super::merge_call_paths;
use super::ExecutionCtx;
use super::PreparationError;
use crate::call_evidence::ExecutionTraceCtx;
use crate::get_current_peer_id;
use crate::log_targets::RUN_PARAMS;
use crate::CallEvidencePath;

use air_parser::ast::Instruction;

type PreparationResult<T> = Result<T, PreparationError>;
/// Represents result of the preparation step.
pub(crate) struct PreparationDescriptor<'ctx, 'i> {
    pub(crate) exec_ctx: ExecutionCtx<'ctx>,
    pub(crate) trace_ctx: ExecutionTraceCtx,
    pub(crate) aqua: Instruction<'i>,
}

/// Parse and prepare supplied data and aqua script.
pub(crate) fn prepare<'i>(
    prev_data: &[u8],
    data: &[u8],
    raw_aqua: &'i str,
    init_peer_id: String,
) -> PreparationResult<PreparationDescriptor<'static, 'i>> {
    fn to_evidence_path(raw_data: &[u8]) -> PreparationResult<CallEvidencePath> {
        use PreparationError::CallEvidenceDeError as CallDeError;

        // treat empty string as an empty call evidence path allows abstracting from
        // the internal format for empty data.
        if raw_data.is_empty() {
            Ok(CallEvidencePath::new())
        } else {
            serde_json::from_slice(&raw_data).map_err(|err| CallDeError(err, raw_data.to_vec()))
        }
    }

    let prev_path = to_evidence_path(prev_data)?;
    let path = to_evidence_path(data)?;

    let aqua: Instruction<'i> = *air_parser::parse(raw_aqua).map_err(AquamarineError::AIRParseError)?;

    log::trace!(
        target: RUN_PARAMS,
        "aqua: {:?}\nprev_path: {:?}\ncurrent_path: {:?}",
        aqua,
        prev_path,
        path
    );

    let (exec_ctx, trace_ctx) = make_contexts(prev_path, path, init_peer_id)?;
    let result = PreparationDescriptor {
        exec_ctx,
        trace_ctx,
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
) -> PreparationResult<(ExecutionCtx<'static>, ExecutionTraceCtx)> {
    use crate::build_targets::CURRENT_PEER_ID_ENV_NAME;
    use PreparationError::CurrentPeerIdEnvError as EnvError;

    let current_peer_id = get_current_peer_id().map_err(|e| EnvError(e, String::from(CURRENT_PEER_ID_ENV_NAME)))?;
    log::trace!(target: RUN_PARAMS, "current peer id {}", current_peer_id);

    let exec_ctx = ExecutionCtx::new(current_peer_id, init_peer_id);
    let current_path = merge_call_paths(prev_path, path)?;
    let call_evidence_ctx = ExecutionTraceCtx::new(current_path);

    Ok((exec_ctx, call_evidence_ctx))
}

/// Parse an AIR script to AST.
pub fn parse(script: &str) -> PreparationResult<Instruction<'_>> {
    let ast = air_parser::parse(script).map_err(PreparationError::AIRParseError)?;
    Ok(*ast)
}
