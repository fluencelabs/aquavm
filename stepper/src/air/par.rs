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

use super::ExecutionContext;
use super::Instruction;
use super::NewEvidenceState;
use crate::Result;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct Par(Box<Instruction>, Box<Instruction>);

impl super::ExecutableInstruction for Par {
    fn execute(&self, ctx: &mut ExecutionContext) -> Result<()> {
        use super::EvidenceState;

        log::info!("par is called with context: {:?}", ctx);

        let current_left = ctx.call_evidence_ctx.left;
        let current_right = ctx.call_evidence_ctx.right;

        let (prev_left, prev_right) = match ctx.call_evidence_ctx.current_states[current_left] {
            EvidenceState::Par(left, right) => (left, right),
            _ => unreachable!(),
        };

        ctx.call_evidence_ctx.right = current_left + prev_left;
        ctx.call_evidence_ctx
            .new_states
            .push(NewEvidenceState::LeftPar(current_left));
        self.0.execute(ctx)?;

        ctx.call_evidence_ctx.left = current_left + prev_left;
        ctx.call_evidence_ctx.right = current_left + prev_left + prev_right;
        ctx.call_evidence_ctx
            .new_states
            .push(NewEvidenceState::RightPar(current_left));
        self.1.execute(ctx)?;

        ctx.call_evidence_ctx.left = ctx.call_evidence_ctx.right;
        ctx.call_evidence_ctx.right = current_right;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use aqua_test_utils::create_aqua_vm;
    use aqua_test_utils::unit_call_service;
    use aquamarine_vm::StepperOutcome;

    use serde_json::json;

    #[test]
    fn par() {
        let mut vm = create_aqua_vm(unit_call_service());

        let script = String::from(
            r#"
            (par (
                (call ("remote_peer_id_1" ("local_service_id" "local_fn_name") () result_name))
                (call ("remote_peer_id_2" ("service_id" "fn_name") () g))
            ))"#,
        );

        let res = vm
            .call(json!([String::from("asd"), script, String::from("{}"),]))
            .expect("call should be successful");

        assert_eq!(
            res,
            StepperOutcome {
                data: String::from("{}"),
                next_peer_pks: vec![
                    String::from("remote_peer_id_1"),
                    String::from("remote_peer_id_2")
                ]
            }
        );
    }
}
