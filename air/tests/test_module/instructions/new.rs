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

use air_test_utils::prelude::*;

#[test]
fn new_with_global_streams_seq() {
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

    let script = format!(
        r#"
            (seq
                (seq
                    (call "{0}" ("" "") ["1"] $stream)
                    (call "{0}" ("" "") ["2"] $stream)
                )
                (fold $stream i
                    (seq
                        (new $stream
                            (seq
                                (seq
                                    (call "{1}" ("" "") [i] $stream)
                                    (next i)
                                )
                                (call "{1}" ("" "") [$stream])
                            )
                        )
                        (call "{2}" ("" "") [$stream])
                    )
                )
            )"#,
        set_variable_peer_id, local_vm_peer_id_1, local_vm_peer_id_2
    );

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
            executed_state::subtrace_lore(0, SubTraceDesc::new(3, 1), SubTraceDesc::new(7, 2)),
            executed_state::subtrace_lore(1, SubTraceDesc::new(4, 1), SubTraceDesc::new(5, 2)),
        ]),
        executed_state::stream_number(1, 0),
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

#[test]
fn several_restrictions() {
    let vm_peer_id = "vm_peer_id";
    let mut vm = create_avm(echo_call_service(), vm_peer_id);

    let script = format!(
        r#"
            (new $stream
                (seq
                    (new $stream
                        (call "{0}" ("" "") ["test"] $stream)
                    )
                    (call "{0}" ("" "") [$stream])
                )
            )"#,
        vm_peer_id
    );

    let result = checked_call_vm!(vm, "", script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![
        executed_state::stream_string("test", 0),
        executed_state::scalar(json!([])),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn check_influence_to_not_restricted() {
    let vm_peer_id = "vm_peer_id";
    let mut vm = create_avm(echo_call_service(), vm_peer_id);

    let script = format!(
        r#"
    (seq
        (new $a
            (seq
                (seq
                    (seq
                        (ap "push more" $a0)
                        (ap "push more" $a1)
                    )
                    (ap "more" $a)
                )
                (call "{0}" ("op" "identity") [$a] a-fix)
            )
        )
        (seq
            (seq
                (call "{0}" ("callbackSrv" "response") [$a0]) ;; should be non-empty
                (call "{0}" ("callbackSrv" "response") [$a1]) ;; should be non-empty
            )
            (seq
                (call "{0}" ("callbackSrv" "response") [$a])  ;; should be empty
                (call "{0}" ("callbackSrv" "response") [a-fix])  ;; should be empty
            )
        )
    )
    "#,
        vm_peer_id
    );

    let result = checked_call_vm!(vm, "", script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![
        executed_state::ap(Some(0)),
        executed_state::ap(Some(0)),
        executed_state::ap(Some(0)),
        executed_state::scalar(json!(["more"])),
        executed_state::scalar(json!(["push more"])),
        executed_state::scalar(json!(["push more"])),
        executed_state::scalar(json!([])),
        executed_state::scalar(json!(["more"])),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn new_in_fold_with_ap() {
    let set_variable_peer_id = "set_variable_peer_id";
    let vm_peer_id = "vm_peer_id";

    let mut set_variable_vm = create_avm(set_variable_call_service(json!([1, 2, 3, 4, 5])), set_variable_peer_id);
    let mut vm = create_avm(echo_call_service(), vm_peer_id);

    let script = format!(
        r#"
        (seq
            (call "{0}" ("" "") [] iterable)
            (fold iterable x
                (seq
                    (new $s1
                        (seq
                            (ap "none" $s1)
                            (call "{1}" ("" "") [$s1] s-fix1) ;; should contains only "none" on each iteration
                        )
                    )
                    (next x)
                )
            )
        )

            "#,
        set_variable_peer_id, vm_peer_id
    );

    let result = checked_call_vm!(set_variable_vm, "", &script, "", "");
    let result = checked_call_vm!(vm, "", script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![
        executed_state::scalar(json!([1, 2, 3, 4, 5])),
        executed_state::ap(Some(0)),
        executed_state::scalar_string_array(vec!["none"]),
        executed_state::ap(Some(0)),
        executed_state::scalar_string_array(vec!["none"]),
        executed_state::ap(Some(0)),
        executed_state::scalar_string_array(vec!["none"]),
        executed_state::ap(Some(0)),
        executed_state::scalar_string_array(vec!["none"]),
        executed_state::ap(Some(0)),
        executed_state::scalar_string_array(vec!["none"]),
    ];
    assert_eq!(actual_trace, expected_trace);

    let data = data_from_result(&result);
    let actual_restricted_streams = data.restricted_streams;
    let expected_restricted_streams = maplit::hashmap! {
        "$s1".to_string() => maplit::hashmap! {
            146 => vec![1,1,1,1,1]
        }
    };
    assert_eq!(actual_restricted_streams, expected_restricted_streams);
}

#[test]
fn new_with_errors() {
    let faillible_peer_id = "failible_peer_id";
    let mut faillible_vm = create_avm(fallible_call_service("service_id_1"), faillible_peer_id);

    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_call_service(), local_peer_id);

    let script = format!(
        r#"
            (seq
                (call "{0}" ("" "") [1] $global_stream) ;; this stream should precense in a data
                (new $restricted_stream_1
                    (seq
                        (new $restricted_stream_2
                            (seq
                                (call "{0}" ("" "") [2] $restricted_stream_2) ;; should have generation 1 in a data
                                (call "{1}" ("service_id_1" "local_fn_name") [] result)
                            )
                        )
                        (call "{0}" ("" "") [2] restricted_stream_1) ;; should have generation 0 in a data
                    )
                )
            )"#,
        local_peer_id, faillible_peer_id
    );

    let result = checked_call_vm!(vm, "", &script, "", "");
    let result = call_vm!(faillible_vm, "", script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![
        executed_state::stream_number(1, 0),
        executed_state::stream_number(2, 0),
        executed_state::service_failed(1, "failed result from fallible_call_service"),
    ];
    assert_eq!(actual_trace, expected_trace);

    let data = data_from_result(&result);

    let actual_restricted_streams = data.restricted_streams;
    let expected_restricted_streams = maplit::hashmap! {
        "$restricted_stream_2".to_string() => maplit::hashmap! {
            216 => vec![1]
        },
        "$restricted_stream_1".to_string() => maplit::hashmap! {
            141 => vec![0]
        }
    };
    assert_eq!(actual_restricted_streams, expected_restricted_streams);

    let actual_global_streams = data.global_streams;
    let expected_global_streams = maplit::hashmap! {
        "$global_stream".to_string() => 1,
    };
    assert_eq!(actual_global_streams, expected_global_streams);
}
