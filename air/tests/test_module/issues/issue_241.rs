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

use air_interpreter_data::ExecutionTrace;
use air_test_utils::prelude::*;

use pretty_assertions::assert_eq;

#[test]
// test for github.com/fluencelabs/aquavm/issues/241
fn issue_241() {
    let peer_1_id = "peer_1_id";
    let array_1_content = json!(["1", "2"]);
    let mut peer_1_vm = create_avm(set_variable_call_service(array_1_content.clone()), peer_1_id);

    let some_peer_id = "some_peer_id";
    let mut some_peer_vm = create_avm(unit_call_service(), some_peer_id);

    let set_array_0_peer_id = "set_array_0_peer_id";
    let peer_2_id = "peer_2_id";
    let peers = json!([peer_1_id, peer_2_id]);
    let mut set_array_0_vm = create_avm(set_variable_call_service(peers.clone()), set_array_0_peer_id);

    let script = f!(r#"
        (seq
            (call "{set_array_0_peer_id}" ("" "") [] array-0)
            (fold array-0 array-0-iterator
                (par
                    (call array-0-iterator ("" "") [] array-1)
                    (seq
                        (fold array-1 array-1-iterator
                            (seq
                                (call "{some_peer_id}" ("" "") [])
                                (next array-1-iterator)
                            )
                        )
                        (next array-0-iterator)
                    )
                )
            )
        )
    "#);

    let result = checked_call_vm!(set_array_0_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(peer_1_vm, <_>::default(), &script, "", result.data);
    let result = checked_call_vm!(some_peer_vm, <_>::default(), &script, "", result.data);
    let actual_trace = trace_from_result(&result);

    let expected_trace = ExecutionTrace::from(vec![
        scalar!(peers, peer = set_array_0_peer_id),
        executed_state::par(1, 4),
        scalar!(array_1_content, peer = peer_1_id),
        unused!("result from unit_call_service", peer = some_peer_id),
        unused!("result from unit_call_service", peer = some_peer_id),
        executed_state::par(1, 0),
        // before 0.22.0 scalar!s wasn't clear after end of a fold block and here was more states
        // from the second iteration of fold over array-1
        executed_state::request_sent_by(some_peer_id),
    ]);
    assert_eq!(actual_trace, expected_trace);
}
