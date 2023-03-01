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
// test for github.com/fluencelabs/aquavm/issues/300
fn issue_300() {
    let peer_id_1 = "peer_id_1";
    let mut peer_vm_1 = create_avm(echo_call_service(), peer_id_1);
    let peer_id_2 = "peer_id_2";
    let mut peer_vm_2 = create_avm(echo_call_service(), peer_id_2);

    let script = f!(r#"
        (new $stream
            (par
                (call "{peer_id_1}" ("" "") [2] $stream)
                (call "{peer_id_2}" ("" "") [1] $stream)
            )
        )
    "#);

    let result_1 = checked_call_vm!(peer_vm_2, <_>::default(), &script, "", "");
    let result_2 = checked_call_vm!(peer_vm_1, <_>::default(), &script, "", result_1.data);
    let actual_trace = trace_from_result(&result_2);

    let expected_trace = vec![
        executed_state::par(1, 1),
        stream!(2, 1, peer = peer_id_1, args = vec![2]),
        stream!(1, 0, peer = peer_id_2, args = vec![1]),
    ];
    assert_eq!(actual_trace, expected_trace);
}
