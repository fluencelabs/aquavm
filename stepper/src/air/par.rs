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

use super::CallEvidenceCtx;
use super::EvidenceState;
use super::ExecutableInstruction;
use super::ExecutionCtx;
use super::Instruction;
use crate::Result;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct Par(Box<Instruction>, Box<Instruction>);

impl ExecutableInstruction for Par {
    fn execute(&self, exec_ctx: &mut ExecutionCtx, call_ctx: &mut CallEvidenceCtx) -> Result<()> {
        log::info!("par is called with context: {:?} {:?}", exec_ctx, call_ctx);

        let (left_subtree_size, right_subtree_size) = extract_subtree_sizes(call_ctx);
        let pre_new_states_count = call_ctx.new_states.len();
        call_ctx.new_states.push(EvidenceState::Par(0, 0));

        let new_left_subtree_size =
            execute_subtree(&self.0, left_subtree_size, exec_ctx, call_ctx)?;
        let new_right_subtree_size =
            execute_subtree(&self.1, right_subtree_size, exec_ctx, call_ctx)?;

        call_ctx.new_states[pre_new_states_count] =
            EvidenceState::Par(new_left_subtree_size, new_right_subtree_size);

        call_ctx.used_states_in_subtree = 0;
        call_ctx.subtree_size = call_ctx.current_states.len();

        Ok(())
    }
}

fn extract_subtree_sizes(call_ctx: &CallEvidenceCtx) -> (usize, usize) {
    let used_states_in_subtree = call_ctx.used_states_in_subtree;
    let subtree_size = call_ctx.subtree_size;

    if used_states_in_subtree < subtree_size {
        match call_ctx.current_states[used_states_in_subtree] {
            EvidenceState::Par(left, right) => (left, right),
            _ => unreachable!(),
        }
    } else {
        (0, 0)
    }
}

fn execute_subtree(
    subtree: &Box<Instruction>,
    subtree_size: usize,
    exec_ctx: &mut ExecutionCtx,
    call_ctx: &mut CallEvidenceCtx,
) -> Result<usize> {
    call_ctx.used_states_in_subtree = 0;
    call_ctx.subtree_size = subtree_size;
    let before_states_count = call_ctx.new_states.len();

    // execute subtree
    subtree.execute(exec_ctx, call_ctx)?;

    Ok(call_ctx.new_states.len() - before_states_count)
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
