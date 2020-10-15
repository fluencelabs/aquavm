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

        let (left_subtree_size, right_subtree_size) = extract_subtree_sizes(call_ctx)?;

        let pre_states_count = call_ctx.current_path.len();
        let pre_unused_elements = call_ctx.current_subtree_elements_count;

        let pre_new_states_count = call_ctx.new_path.len();
        call_ctx.new_path.push_back(EvidenceState::Par(0, 0));

        let new_left_subtree_size = execute_subtree(&self.0, left_subtree_size, exec_ctx, call_ctx)?;
        let new_right_subtree_size = execute_subtree(&self.1, right_subtree_size, exec_ctx, call_ctx)?;

        let new_par_evidence_state = EvidenceState::Par(new_left_subtree_size, new_right_subtree_size);
        log::info!("call evidence: adding new state {:?}", new_par_evidence_state);
        call_ctx.new_path[pre_new_states_count] = new_par_evidence_state;

        let post_states_count = call_ctx.current_path.len();
        call_ctx.current_subtree_elements_count = pre_unused_elements - (pre_states_count - post_states_count);

        Ok(())
    }
}

fn extract_subtree_sizes(call_ctx: &mut CallEvidenceCtx) -> Result<(usize, usize)> {
    use crate::AquamarineError::InvalidEvidenceState;

    if call_ctx.current_subtree_elements_count == 0 {
        return Ok((0, 0));
    }

    call_ctx.current_subtree_elements_count -= 1;

    log::info!(
        "call evidence: the previous state was found {:?}",
        call_ctx.current_path[0]
    );

    // unwrap is safe here because of length's been checked
    match call_ctx.current_path.pop_front().unwrap() {
        EvidenceState::Par(left, right) => Ok((left, right)),
        state => Err(InvalidEvidenceState(state, String::from("par"))),
    }
}

fn execute_subtree(
    subtree: &Instruction,
    subtree_size: usize,
    exec_ctx: &mut ExecutionCtx,
    call_ctx: &mut CallEvidenceCtx,
) -> Result<usize> {
    call_ctx.current_subtree_elements_count = subtree_size;
    let before_states_count = call_ctx.new_path.len();

    // execute subtree
    subtree.execute(exec_ctx, call_ctx)?;

    Ok(call_ctx.new_path.len() - before_states_count)
}

#[cfg(test)]
mod tests {
    use aqua_test_utils::create_aqua_vm;
    use aqua_test_utils::unit_call_service;

    use serde_json::json;

    #[test]
    fn par_remote_remote() {
        use std::collections::HashSet;

        let mut vm = create_aqua_vm(unit_call_service(), "");

        let script = String::from(
            r#"
            (par (
                (call ("remote_peer_id_1" ("local_service_id" "local_fn_name") () result_name))
                (call ("remote_peer_id_2" ("service_id" "fn_name") () g))
            ))"#,
        );

        let mut res = vm
            .call(json!([
                "asd",
                script,
                "{}",
                "{}",
            ]))
            .expect("call should be successful");

        let peers_result: HashSet<_> = res.next_peer_pks.drain(..).collect();
        let peers_right: HashSet<_> = vec![String::from("remote_peer_id_1"), String::from("remote_peer_id_2")]
            .drain(..)
            .collect();

        assert_eq!(peers_result, peers_right);
    }

    #[test]
    fn par_local_remote() {
        let mut vm = create_aqua_vm(unit_call_service(), "");

        let script = String::from(
            r#"
            (par (
                (call (%current_peer_id% ("local_service_id" "local_fn_name") () result_name))
                (call ("remote_peer_id_2" ("service_id" "fn_name") () g))
            ))"#,
        );

        let res = vm
            .call(json!([
                "asd",
                script,
                "{}",
                "{}",
            ]))
            .expect("call should be successful");

        assert_eq!(res.next_peer_pks, vec![String::from("remote_peer_id_2")]);
    }
}
