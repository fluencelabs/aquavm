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
use crate::log_instruction;
use crate::log_targets::EVIDENCE_CHANGING;
use crate::Result;

use air_parser::ast::Par;

enum SubtreeType {
    Left,
    Right,
}

impl std::fmt::Display for SubtreeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => write!(f, "left"),
            Self::Right => write!(f, "right"),
        }
    }
}

impl<'i> ExecutableInstruction<'i> for Par<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, call_ctx: &mut CallEvidenceCtx) -> Result<()> {
        use SubtreeType::*;

        log_instruction!(par, exec_ctx, call_ctx);

        let (left_subtree_size, right_subtree_size) = extract_subtree_sizes(call_ctx)?;

        let par_pos = call_ctx.new_path.len();
        call_ctx.new_path.push_back(EvidenceState::Par(0, 0));

        // execute a left subtree of this par
        execute_subtree(&self.0, left_subtree_size, exec_ctx, call_ctx, par_pos, Left)?;
        let left_subtree_complete = exec_ctx.subtree_complete;

        // execute a right subtree of this par
        execute_subtree(&self.1, right_subtree_size, exec_ctx, call_ctx, par_pos, Right)?;
        let right_subtree_complete = exec_ctx.subtree_complete;

        // par is completed if at least one of its subtrees is completed
        exec_ctx.subtree_complete = left_subtree_complete || right_subtree_complete;

        Ok(())
    }
}

fn extract_subtree_sizes(call_ctx: &mut CallEvidenceCtx) -> Result<(usize, usize)> {
    use crate::AquamarineError::InvalidEvidenceState;

    if call_ctx.current_subtree_size == 0 {
        return Ok((0, 0));
    }

    call_ctx.current_subtree_size -= 1;

    log::trace!(
        target: EVIDENCE_CHANGING,
        "  previous call evidence state was found {:?}",
        call_ctx.current_path[0]
    );

    // unwrap is safe here because of length's been checked
    match call_ctx.current_path.pop_front().unwrap() {
        EvidenceState::Par(left, right) => Ok((left, right)),
        state => Err(InvalidEvidenceState(state, String::from("par"))),
    }
}

/// Execute provided subtree and update Par state in call_ctx.new_path.
fn execute_subtree<'i>(
    subtree: &Instruction<'i>,
    subtree_size: usize,
    exec_ctx: &mut ExecutionCtx<'i>,
    call_ctx: &mut CallEvidenceCtx,
    current_par_pos: usize,
    subtree_type: SubtreeType,
) -> Result<()> {
    use crate::AquamarineError::LocalServiceError;

    let before_subtree_size = call_ctx.current_subtree_size;
    call_ctx.current_subtree_size = subtree_size;
    let before_new_path_len = call_ctx.new_path.len();

    exec_ctx.subtree_complete = determine_subtree_complete(&subtree);

    // execute a subtree
    match subtree.execute(exec_ctx, call_ctx) {
        res @ Ok(_) => {
            update_par_state(call_ctx, subtree_type, current_par_pos, before_new_path_len);
            call_ctx.current_subtree_size = before_subtree_size - subtree_size;
            res
        }
        // if there is a service error, update already added Par state
        // and then bubble the error up
        err @ Err(LocalServiceError(_)) => {
            update_par_state(call_ctx, subtree_type, current_par_pos, before_new_path_len);
            call_ctx.current_subtree_size = before_subtree_size - subtree_size;
            err
        }
        err @ Err(_) => err,
    }
}

fn determine_subtree_complete(next_instruction: &Instruction<'_>) -> bool {
    // this is needed to prevent situation when on such pattern
    // (fold (Iterable i
    //    (par
    //       (call ..)
    //       (next i)
    //    )
    // )
    // par will be completed after the last next that wouldn't change subtree_complete
    !matches!(next_instruction, Instruction::Next(_))
}

/// Set left or right fields of a Par identified by current_par_pos.
fn update_par_state(
    call_ctx: &mut CallEvidenceCtx,
    subtree_type: SubtreeType,
    current_par_pos: usize,
    before_new_path_len: usize,
) {
    let new_subtree_size = call_ctx.new_path.len() - before_new_path_len;

    // unwrap is safe here, because this par is added at the beginning of this par instruction.
    let par_state = call_ctx.new_path.get_mut(current_par_pos).unwrap();
    match par_state {
        EvidenceState::Par(left, right) => {
            if let SubtreeType::Left = subtree_type {
                *left = new_subtree_size;
            } else {
                *right = new_subtree_size;
            }

            log::trace!(
                target: EVIDENCE_CHANGING,
                "  set {} par subtree size to {}",
                subtree_type,
                new_subtree_size
            );
        }
        _ => unreachable!("current_pas_pos must point to a par state"),
    }
}

#[cfg(test)]
mod tests {
    use aqua_test_utils::call_vm;
    use aqua_test_utils::create_aqua_vm;
    use aqua_test_utils::unit_call_service;

    #[test]
    fn par_remote_remote() {
        use std::collections::HashSet;

        let mut vm = create_aqua_vm(unit_call_service(), "");

        let script = String::from(
            r#"
            (par 
                (call "remote_peer_id_1" ("local_service_id" "local_fn_name") [] result_name)
                (call "remote_peer_id_2" ("service_id" "fn_name") [] g)
            )"#,
        );

        let mut res = call_vm!(vm, "", script, "[]", "[]");

        let peers_result: HashSet<_> = res.next_peer_pks.drain(..).collect();
        let peers_right: HashSet<_> =
            maplit::hashset!(String::from("remote_peer_id_1"), String::from("remote_peer_id_2"));

        assert_eq!(peers_result, peers_right);
    }

    #[test]
    fn par_local_remote() {
        let mut vm = create_aqua_vm(unit_call_service(), "");

        let script = String::from(
            r#"
            (par 
                (call %current_peer_id% ("local_service_id" "local_fn_name") [] result_name)
                (call "remote_peer_id_2" ("service_id" "fn_name") [] g)
            )"#,
        );

        let res = call_vm!(vm, "", script, "[]", "[]");

        assert_eq!(res.next_peer_pks, vec![String::from("remote_peer_id_2")]);
    }
}
