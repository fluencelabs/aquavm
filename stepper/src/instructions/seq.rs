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
use crate::Result;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct Seq(Box<Instruction>, Box<Instruction>);

impl super::ExecutableInstruction for Seq {
    fn execute(&self, ctx: &mut ExecutionContext) -> Result<()> {
        log::info!("seq is called with context: {:?}", ctx);

        let pks_count_before_call = ctx.next_peer_pks.len();
        self.0.execute(ctx)?;

        if pks_count_before_call == ctx.next_peer_pks.len() {
            self.1.execute(ctx)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use aquamarine_vm::StepperOutcome;
    use aquamarine_vm::HostExportedFunc;
    use aquamarine_vm::IValue;
    use aquamarine_vm::vec1::Vec1;
    use aqua_test_utils::create_aqua_vm;

    use serde_json::json;

    #[test]
    fn par() {
        let call_service: HostExportedFunc = Box::new(|_, _| -> Option<IValue> {
            Some(IValue::Record(
                Vec1::new(vec![
                    IValue::S32(0),
                    IValue::String(String::from("\"test\"")),
                ])
                    .unwrap(),
            ))
        });
        let mut vm = create_aqua_vm(call_service);

        let script = String::from(r#"
            (seq (
                (call (remote_peer_id_1 (local_service_id local_fn_name) () result_name))
                (call (remote_peer_id_2 (service_id fn_name) () g))
            ))"#,
        );

        let res = vm.call(json!([String::from("asd"), script, String::from("{}"),])).expect("call should be successful");

        assert_eq!(res, StepperOutcome {
            data: String::from("{}"),
            next_peer_pks: vec![String::from("remote_peer_id_1")]
        });
    }
}
