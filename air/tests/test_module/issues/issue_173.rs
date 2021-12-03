/*
 * Copyright 2021 Fluence Labs Limited
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
// test for github.com/fluencelabs/aquavm/issues/173
fn issue_173() {
    let set_variable_peer_id = "set_variable_peer_id";
    let local_vm_peer_id_1 = "local_vm_peer_id_1";
    let local_vm_peer_id_2 = "local_vm_peer_id_2";

    let mut local_vm_1 = create_avm(echo_call_service(), local_vm_peer_id_1);
    let mut local_vm_2 = create_avm(echo_call_service(), local_vm_peer_id_2);

    let variables_mapping = maplit::hashmap! {
        "1".to_string() => json!(1),
        "2".to_string() => json!(2),
    };
    let mut set_variable_vm = create_avm(
        set_variables_call_service(variables_mapping, VariableOptionSource::Argument(0)),
        set_variable_peer_id,
    );

    let script = f!(r#"
            (seq
                (seq
                    (call "{set_variable_peer_id}" ("" "") ["1"] $stream)
                    (call "{set_variable_peer_id}" ("" "") ["2"] $stream)
                )
                (fold $stream i
                    (par
                        (new $stream
                            (seq
                                (seq
                                    (call "{local_vm_peer_id_1}" ("" "") [i] $stream)
                                    (next i)
                                )
                                (call "{local_vm_peer_id_1}" ("" "") [$stream])
                            )
                        )
                        (call "{local_vm_peer_id_2}" ("" "") [$stream])
                    )
                )
            )"#);

    let result = checked_call_vm!(set_variable_vm, "", &script, "", "");
    let vm_1_result = checked_call_vm!(local_vm_1, "", &script, "", result.data);
    let vm_2_result = checked_call_vm!(local_vm_2, "", &script, "", vm_1_result.data.clone());

    let vm_1_result = checked_call_vm!(local_vm_1, "", &script, vm_1_result.data, vm_2_result.data.clone());
    let vm_2_result = checked_call_vm!(local_vm_2, "", script, vm_2_result.data, vm_1_result.data);

    let actual_trace = trace_from_result(&vm_2_result);
    let expected_trace = vec![
        executed_state::stream_number(1, 0),
        executed_state::stream_number(2, 0),
        executed_state::fold(vec![
            executed_state::subtrace_lore(0, SubTraceDesc::new(3, 2), SubTraceDesc::new(9, 2)),
            executed_state::subtrace_lore(1, SubTraceDesc::new(5, 2), SubTraceDesc::new(7, 2)),
        ]),
        executed_state::par(6, 1),
        executed_state::stream_number(1, 0),
        executed_state::par(2, 1),
        executed_state::stream_number(2, 0),
        executed_state::scalar(json!([2])),
        executed_state::scalar(json!([1, 2])),
        executed_state::scalar(json!([1])),
        executed_state::scalar(json!([1, 2])),
    ];
    assert_eq!(actual_trace, expected_trace);

    let data = data_from_result(&vm_2_result);
    let actual_restricted_streams = data.restricted_streams;
    let expected_restricted_streams = maplit::hashmap! {
        "$stream".to_string() => maplit::hashmap! {
            282 => vec![1,1]
        }
    };
    assert_eq!(actual_restricted_streams, expected_restricted_streams);
}
