/*
 * Copyright 2022 Fluence Labs Limited
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

use air_interpreter_interface::INTERPRETER_SUCCESS;
use air_test_utils::prelude::*;

#[tokio::test]
// test for github.com/fluencelabs/aquavm/issues/346
async fn issue_346() {
    let vm_peer_id = "peer_id_1";
    let mut peer_vm = create_avm(echo_call_service(), vm_peer_id).await;

    let script = format!(
        r#"
        (par
            (call "unknown_peer" ("" "") [] $stream) ; to make validator happy
            (xor
                (canon "{vm_peer_id}" $stream #canon_stream) ; it returns a catchable error
                (call "{vm_peer_id}" ("" "") [""])
            )
        )
    "#
    );

    let result = call_vm!(peer_vm, <_>::default(), &script, "", "");
    assert_eq!(result.ret_code, INTERPRETER_SUCCESS);
}
