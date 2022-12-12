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

use air_test_utils::prelude::*;

#[test]
// test for github.com/fluencelabs/aquavm/issues/348
fn issue_348() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let mut peer_vm_1 = create_avm(echo_call_service(), vm_peer_id_1);
    let vm_peer_id_2 = "vm_peer_id_2";
    let mut peer_vm_2 = create_avm(echo_call_service(), vm_peer_id_2);
    let vm_peer_id_3 = "vm_peer_id_3";
    let mut peer_vm_3 = create_avm(echo_call_service(), vm_peer_id_3);

    let script = f!(r#"
        (seq
            (seq
                (ap 1 $inner)
                (seq
                    (call "{vm_peer_id_2}" ("op" "noop") [1])
                    (ap 2 $inner)
                )
            )
            (seq
                (canon "{vm_peer_id_2}" $inner #inner)
                (seq
                    (call "{vm_peer_id_3}" ("op" "noop") [1])
                    (call "{vm_peer_id_2}" ("op" "noop") [1])
                )
            )
        )
    "#);

    let result11 = checked_call_vm!(peer_vm_1, <_>::default(), &script, "", "");
    let result21 = checked_call_vm!(peer_vm_2, <_>::default(), &script, "", result11.data);
    let result31 = checked_call_vm!(peer_vm_3, <_>::default(), &script, "", result21.data.clone());
    let _result22 = checked_call_vm!(peer_vm_2, <_>::default(), &script, result21.data, result31.data);
}
