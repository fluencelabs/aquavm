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

use super::StepperOutcome;
use crate::air::ExecutableInstruction;
use crate::air::ExecutionCtx;
use crate::air::Instruction;
use crate::call_evidence::EvidenceState;
use crate::get_current_peer_id;
use crate::AquaData;
use crate::AquamarineError;
use crate::Result;

use crate::call_evidence::CallEvidenceCtx;

pub(crate) fn execute_aqua(init_user_id: String, aqua: String, data: String) -> StepperOutcome {
    log::info!(
        "stepper invoked with user_id = {}, aqua = {:?}, data = {:?}",
        init_user_id,
        aqua,
        data
    );

    execute_aqua_impl(init_user_id, aqua, data).unwrap_or_else(Into::into)
}

fn execute_aqua_impl(_init_user_id: String, aqua: String, data: String) -> Result<StepperOutcome> {
    let mut parsed_data: AquaData =
        serde_json::from_str(&data).map_err(AquamarineError::DataSerdeError)?;
    let formatted_aqua = format_aqua(aqua);
    let parsed_aqua = serde_sexpr::from_str::<Instruction>(&formatted_aqua)?;

    log::info!(
        "\nparsed_aqua: {:?}\nparsed_data: {:?}",
        parsed_aqua,
        parsed_data
    );

    let current_peer_id = get_current_peer_id()
        .map_err(|e| AquamarineError::CurrentPeerIdEnvError(e, String::from("CURRENT_PEER_ID")))?;

    let call_evidence_ctx_key: &str = "__call";
    let current_states: Vec<EvidenceState> = parsed_data
        .remove(call_evidence_ctx_key)
        .map(|v| serde_json::from_value(v).unwrap())
        .unwrap_or_default();

    let mut execution_ctx = ExecutionCtx::new(parsed_data, current_peer_id);
    let mut call_evidence_ctx = CallEvidenceCtx::new(current_states);

    parsed_aqua.execute(&mut execution_ctx, &mut call_evidence_ctx)?;

    let serialized_call_ctx = serde_json::to_value(call_evidence_ctx.new_states).unwrap();
    execution_ctx
        .data
        .insert(call_evidence_ctx_key.to_string(), serialized_call_ctx);

    let data =
        serde_json::to_string(&execution_ctx.data).map_err(AquamarineError::DataSerdeError)?;

    Ok(StepperOutcome {
        ret_code: 0,
        data,
        next_peer_pks: execution_ctx.next_peer_pks,
    })
}

/// Formats aqua script in a form of S-expressions to a form compatible with the serde_sexpr crate.
fn format_aqua(aqua: String) -> String {
    use std::iter::FromIterator;

    let mut formatted_aqua = Vec::with_capacity(aqua.len());
    // whether to skip the next whitespace
    let mut skip_next_whitespace = false;
    // whether c was a closing brace
    let mut was_cbr = false;

    for c in aqua.chars() {
        let is_whitespace = c == ' ';
        if (skip_next_whitespace && is_whitespace) || c == '\n' {
            continue;
        }

        let is_cbr = c == ')';

        skip_next_whitespace = is_whitespace || c == '(' || is_cbr;
        if was_cbr && !is_cbr {
            formatted_aqua.push(' ');
        }

        was_cbr = is_cbr;
        formatted_aqua.push(c)
    }

    String::from_iter(formatted_aqua.into_iter())
}

#[cfg(test)]
mod tests {
    #[test]
    fn format_aqua_test() {
        let aqua = format!(
            r#"(( ((  (seq (
            (call (%current_peer_id% (add_module ||) (module) module))
            (seq (
                (call (%current_peer_id% (add_blueprint ||) (blueprint) blueprint_id))
                (seq (
                    (call (%current_peer_id% (create ||) (blueprint_id) service_id))
                    (call ({} (|| ||) (service_id) client_result))
                )  )
            ) )
        ))"#,
            "abc"
        );

        let aqua = super::format_aqua(aqua);
        let formatted_aqua = String::from("(((((seq ((call (%current_peer_id% (add_module ||) (module) module)) (seq ((call (%current_peer_id% (add_blueprint ||) (blueprint) blueprint_id)) (seq ((call (%current_peer_id% (create ||) (blueprint_id) service_id)) (call (abc (|| ||) (service_id) client_result))))))))");

        assert_eq!(aqua, formatted_aqua);
    }
}
