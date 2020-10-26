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
use super::ExecutionCtx;
use super::Instruction;
use crate::Result;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct Seq(Box<Instruction>, Box<Instruction>);

impl super::ExecutableInstruction for Seq {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'_>, call_ctx: &mut CallEvidenceCtx) -> Result<()> {
        log::info!("seq is called with contexts: {:?} {:?}", exec_ctx, call_ctx);

        exec_ctx.subtree_complete = true;
        self.0.execute(exec_ctx, call_ctx)?;

        if exec_ctx.subtree_complete {
            self.1.execute(exec_ctx, call_ctx)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use aqua_test_utils::create_aqua_vm;
    use aqua_test_utils::unit_call_service;

    use serde_json::json;

    #[test]
    fn seq_remote_remote() {
        let mut vm = create_aqua_vm(unit_call_service(), "");

        let script = String::from(
            r#"
            (seq (
                (call ("remote_peer_id_1" ("local_service_id" "local_fn_name") () result_name))
                (call ("remote_peer_id_2" ("service_id" "fn_name") () g))
            ))"#,
        );

        let res = vm
            .call(json!(["asd", script, "{}", "{}",]))
            .expect("call should be successful");

        assert_eq!(res.next_peer_pks, vec![String::from("remote_peer_id_1")]);

        let res = vm
            .call(json!([
                "asd",
                script,
                "{}",
                json!({
                    "__call": [{"call": "executed"}]
                    }
                )
                .to_string(),
            ]))
            .expect("call should be successful");

        assert_eq!(res.next_peer_pks, vec![String::from("remote_peer_id_2")]);
    }

    #[test]
    fn seq_local_remote() {
        let mut vm = create_aqua_vm(unit_call_service(), "");

        let script = String::from(
            r#"
            (seq (
                (call (%current_peer_id% ("local_service_id" "local_fn_name") () result_name))
                (call ("remote_peer_id_2" ("service_id" "fn_name") () g))
            ))"#,
        );

        let res = vm
            .call(json!(["asd", script, "{}", "{}",]))
            .expect("call should be successful");

        assert_eq!(res.next_peer_pks, vec![String::from("remote_peer_id_2")]);
    }
}
