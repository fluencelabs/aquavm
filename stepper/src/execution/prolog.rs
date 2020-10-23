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
use super::CALL_EVIDENCE_CTX_KEY;
use crate::air::ExecutionCtx;
use crate::air::Instruction;
use crate::call_evidence::merge_call_paths;
use crate::call_evidence::CallEvidenceCtx;
use crate::call_evidence::EvidenceState;
use crate::get_current_peer_id;
use crate::AquaData;
use crate::AquamarineError;
use crate::Result;

use std::collections::VecDeque;

/// Parse and prepare supplied data and aqua script.
pub(super) fn prepare(prev_data: String, data: String, aqua: String) -> Result<(AquaData, AquaData, Instruction)> {
    use AquamarineError::DataDeserializationError as DataDeError;

    let parsed_prev_data: AquaData = serde_json::from_str(&prev_data).map_err(DataDeError)?;
    let parsed_data: AquaData = serde_json::from_str(&data).map_err(DataDeError)?;

    let formatted_aqua = format_aqua(aqua);
    let parsed_aqua: Instruction = serde_sexpr::from_str(&formatted_aqua)?;

    log::info!(
        "\nparsed aqua: {:?}\nparsed prev_data: {:?}\nparsed data: {:?}",
        parsed_aqua,
        parsed_prev_data,
        parsed_data
    );

    Ok((parsed_prev_data, parsed_data, parsed_aqua))
}

/// Make execution and call evidence contexts from supplied data.
/// Internally, it unites variable from previous and current data and merges call evidence paths.
pub(super) fn make_contexts(mut prev_data: AquaData, mut data: AquaData) -> Result<(ExecutionCtx, CallEvidenceCtx)> {
    use AquamarineError::CallEvidenceDeserializationError as CallDeError;
    use AquamarineError::CurrentPeerIdEnvError as EnvError;

    let current_peer_id = get_current_peer_id().map_err(|e| EnvError(e, String::from("CURRENT_PEER_ID")))?;

    let prev_states: VecDeque<EvidenceState> = match prev_data.remove(CALL_EVIDENCE_CTX_KEY) {
        Some(jvalue) => serde_json::from_value(jvalue).map_err(CallDeError)?,
        None => VecDeque::new(),
    };

    let states: VecDeque<EvidenceState> = match data.remove(CALL_EVIDENCE_CTX_KEY) {
        Some(jvalue) => serde_json::from_value(jvalue).map_err(CallDeError)?,
        None => VecDeque::new(),
    };

    let data = merge_data(prev_data, data)?;
    let current_path = merge_call_paths(prev_states, states)?;

    let execution_ctx = ExecutionCtx::new(data, current_peer_id);
    let call_evidence_ctx = CallEvidenceCtx::new(current_path);

    Ok((execution_ctx, call_evidence_ctx))
}

fn merge_data(mut prev_data: AquaData, data: AquaData) -> Result<AquaData> {
    use boolinator::Boolinator;
    use std::collections::hash_map::Entry::{Occupied, Vacant};
    use AquamarineError::MultipleVariablesFound;

    for (key, value) in data {
        match prev_data.entry(key) {
            Vacant(entry) => {
                entry.insert(value);
            }
            // check that data has equal values for the same key
            Occupied(entry) => {
                entry
                    .get()
                    .eq(&value)
                    .ok_or_else(|| MultipleVariablesFound(entry.key().clone()))?;
            }
        }
    }

    Ok(prev_data)
}
