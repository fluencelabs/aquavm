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

use super::ExecutionCtx;
use super::ExecutionResult;
use super::ExecutionTraceCtx;
use crate::log_instruction;

use air_parser::ast::Seq;

impl<'i> super::ExecutableInstruction<'i> for Seq<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut ExecutionTraceCtx) -> ExecutionResult<()> {
        log_instruction!(seq, exec_ctx, trace_ctx);

        exec_ctx.subtree_complete = true;
        self.0.execute(exec_ctx, trace_ctx)?;

        if exec_ctx.subtree_complete {
            self.1.execute(exec_ctx, trace_ctx)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use air_test_utils::call_vm;
    use air_test_utils::create_avm;
    use air_test_utils::unit_call_service;

    use serde_json::json;

    #[test]
    fn seq_remote_remote() {
        let mut vm = create_avm(unit_call_service(), "");

        let script = String::from(
            r#"
            (seq 
                (call "remote_peer_id_1" ("local_service_id" "local_fn_name") [] result_name)
                (call "remote_peer_id_2" ("service_id" "fn_name") [] g)
            )"#,
        );

        let res = call_vm!(vm, "asd", script.clone(), "[]", "[]");
        assert_eq!(res.next_peer_pks, vec![String::from("remote_peer_id_1")]);

        let res = call_vm!(vm, "asd", script, "[]", json!([{"call": {"executed": ""}}]).to_string());
        assert_eq!(res.next_peer_pks, vec![String::from("remote_peer_id_2")]);
    }

    #[test]
    fn seq_local_remote() {
        let local_peer_id = "local_peer_id";
        let remote_peer_id = String::from("remote_peer_id");
        let mut vm = create_avm(unit_call_service(), local_peer_id);

        let script = format!(
            r#"
            (seq 
                (call "{}" ("local_service_id" "local_fn_name") [] result_name)
                (call "{}" ("service_id" "fn_name") [] g)
            )"#,
            local_peer_id, remote_peer_id
        );

        let res = call_vm!(vm, "asd", script, "[]", "[]");
        assert_eq!(res.next_peer_pks, vec![remote_peer_id]);
    }
}
