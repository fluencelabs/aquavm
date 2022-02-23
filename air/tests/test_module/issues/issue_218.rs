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

use fstrings::f;
use fstrings::format_args_f;

#[test]
// test for github.com/fluencelabs/aquavm/issues/218
fn issue_218() {
    let peer_1_id = "peer_1_id";
    let peer_2_id = "peer_2_id";
    let peer_3_id = "peer_3_id";
    let join_1_id = "join_1_id";
    let join_2_id = "join_2_id";
    let resulted_join_id = "resulted_join_id";
    let set_variable_id = "set_variable_id";

    let mut peer_1 = create_avm(set_variable_call_service(json!("peer_1_value")), peer_1_id);
    let mut peer_2 = create_avm(set_variable_call_service(json!("peer_2_value")), peer_2_id);
    let mut peer_3 = create_avm(set_variable_call_service(json!("peer_3_value")), peer_3_id);
    let mut join_1 = create_avm(echo_call_service(), join_1_id);
    let mut join_2 = create_avm(echo_call_service(), join_2_id);
    let mut resulted_join = create_avm(set_variable_call_service(json!("")), resulted_join_id);
    let mut set_variable = create_avm(set_variable_call_service(json!([peer_1_id, peer_2_id, peer_3_id])), set_variable_id);

    let script = f!(r#"
        (seq
            (seq
                (seq
                    ;; let's peers be an array of two values [peer_1_id, peer_2_id]
                    (call "{set_variable_id}" ("" "") [] peers)
                    (fold peers peer
                        (par
                            (seq
                                (call peer ("" "") [] value)
                                ;; it's crucial to reproduce this bug to add value to stream
                                ;; with help of ap instruction
                                (ap value $stream)
                            )
                            (next peer)
                        )
                    )
                )
                ;; join streams on join_1/join_2 peers in such a way that will have different state:
                ;; join_1 $stream: [peer_1_value, peer_2_value]
                ;; join_2 $stream: [peer_2_value, peer_1_value]
                (fold $stream iterator
                    (par
                        (par
                            (call "{join_1_id}" ("" "") [iterator])
                            (call "{join_2_id}" ("" "") [iterator])
                        )
                        (next iterator)
                    )
                )
            )
            ;; then we'll obtain incompatible state error from trace handler here,
            ;; because ap instruction doesn't update internal correspondence between position in
            ;; a new trace and previous. And because of that fold can't shuffle its lores accoring
            ;; to what was really executed.
            (call "{resulted_join_id}" ("" "") [])
        )
    "#);

    let result = checked_call_vm!(set_variable, "", &script, "", "");
    let peer_1_result = checked_call_vm!(peer_1, "", &script, "", result.data.clone());
    let peer_2_result = checked_call_vm!(peer_2, "", &script, "", result.data.clone());

    let join_1_result = checked_call_vm!(join_1, "", &script, "", peer_1_result.data.clone());
    let join_1_result = checked_call_vm!(join_1, "", &script, join_1_result.data, peer_2_result.data.clone());
    let join_2_result = checked_call_vm!(join_2, "", &script, "", peer_2_result.data.clone());
    print_trace(&join_2_result, "join_2_result");
    print_trace(&peer_1_result, "peer_1_result");
    let join_2_result = checked_call_vm!(join_2, "", &script, join_2_result.data, peer_1_result.data.clone());

    let join_1_result = checked_call_vm!(join_1, "", &script, join_1_result.data, join_2_result.data.clone());
    let join_2_result = checked_call_vm!(join_2, "", &script, join_2_result.data, join_1_result.data.clone());

    let result = checked_call_vm!(resulted_join, "", &script, "", join_1_result.data);
    let result = checked_call_vm!(resulted_join, "", &script, result.data, join_2_result.data);

    print_trace(&result, "result");
    /*
    let result = checked_call_vm!(some_peer, client_id, &script, "", "");
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        executed_state::scalar(json!([])),
        executed_state::scalar_string(error_message),
    ];
    assert_eq!(actual_trace, expected_trace);
     */
}
