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

use air_interpreter_data::ExecutionTrace;
use air_test_utils::prelude::*;

use pretty_assertions::assert_eq;

#[test]
#[ignore]
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
                    (call "{set_variable_peer_id}" ("" "") ["1"] $stream)
                    (call "{set_variable_peer_id}" ("" "") ["2"] $stream)
                )
                (fold $stream i
                    (seq
                        (new $stream
                            (seq
                                (par
                                    (call "{local_vm_peer_id_1}" ("" "") [i] $stream)
                                    (next i)
                                )
                                (call "{local_vm_peer_id_1}" ("" "") [$stream])
                            )
                        )
                        (call "{local_vm_peer_id_2}" ("" "") [$stream])
                    )
                )
            )"#
    );

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let vm_1_result = checked_call_vm!(local_vm_1, <_>::default(), &script, "", result.data);
    let vm_2_result = checked_call_vm!(local_vm_2, <_>::default(), &script, "", vm_1_result.data.clone());

    let vm_1_result = checked_call_vm!(
        local_vm_1,
        <_>::default(),
        &script,
        vm_1_result.data,
        vm_2_result.data.clone()
    );
    let vm_2_result = checked_call_vm!(local_vm_2, <_>::default(), script, vm_2_result.data, vm_1_result.data);

    let actual_trace = trace_from_result(&vm_2_result);
    let expected_trace = vec![
        stream!(1, 0, peer = set_variable_peer_id, args = ["1"]),
        stream!(2, 0, peer = set_variable_peer_id, args = ["2"]),
        executed_state::fold(vec![
            executed_state::subtrace_lore(0, SubTraceDesc::new(3.into(), 1), SubTraceDesc::new(7.into(), 2)),
            executed_state::subtrace_lore(1, SubTraceDesc::new(4.into(), 1), SubTraceDesc::new(5.into(), 2)),
        ]),
        stream!(1, 0, peer = local_vm_peer_id_1, args = [1]),
        stream!(2, 0, peer = local_vm_peer_id_1, args = [1]),
        scalar!(json!([2]), peer = local_vm_peer_id_1, args = [json!([2])]),
        scalar!(json!([1, 2]), peer = local_vm_peer_id_1, args = [json!([1, 2])]),
        scalar!(json!([1]), peer = local_vm_peer_id_1, args = [json!([1])]),
        scalar!(json!([1, 2]), peer = local_vm_peer_id_1, args = [json!([1, 2])]),
    ];
    assert_eq!(actual_trace, expected_trace);
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
                        (call "{vm_peer_id}" ("" "") ["test"] $stream)
                    )
                    (seq
                        (canon "{vm_peer_id}" $stream #canon_stream)
                        (call "{vm_peer_id}" ("" "") [#canon_stream])
                    )
                )
            )"#
    );

    let result = checked_call_vm!(vm, <_>::default(), script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_trace = ExecutionTrace::from(vec![
        stream!("test", 0, peer = vm_peer_id, args = ["test"]),
        executed_state::canon(json!({
            "tetraplet": {"function_name": "", "lens": "", "peer_pk": "vm_peer_id", "service_id": ""},
            "values": [

            ]
        })),
        unused!(json!([]), peer = vm_peer_id, args = [json!([])]),
    ]);
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
                (seq
                    (canon "{vm_peer_id}" $a #a)
                    (call "{vm_peer_id}" ("op" "identity") [#a] a-fix)
                )
            )
        )
        (seq
            (seq
                (seq
                    (canon "{vm_peer_id}" $a0 #a0)
                    (call "{vm_peer_id}" ("callbackSrv" "response") [#a0]) ;; should be non-empty
                )
                (seq
                    (canon "{vm_peer_id}" $a1 #a1)
                    (call "{vm_peer_id}" ("callbackSrv" "response") [#a1]) ;; should be non-empty
                )
            )
            (seq
                (seq
                    (canon "{vm_peer_id}" $a #aa)
                    (call "{vm_peer_id}" ("callbackSrv" "response") [#aa])  ;; should be empty
                )
                (call "{vm_peer_id}" ("callbackSrv" "response") [a-fix])  ;; should be empty
            )
        )
    )
    "#
    );

    let result = checked_call_vm!(vm, <_>::default(), script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_trace = ExecutionTrace::from(vec![
        executed_state::ap(0),
        executed_state::ap(0),
        executed_state::ap(0),
        executed_state::canon(json!(
            {
                "tetraplet": {"function_name": "", "lens": "", "peer_pk": "vm_peer_id", "service_id": ""},
                "values": [
                    {
                        "result": "more",
                        "tetraplet": {"function_name": "", "lens": "", "peer_pk": "", "service_id": ""},
                        "trace_pos": 2
                    }
                ]
            }
        )),
        scalar!(
            json!(["more"]),
            peer = vm_peer_id,
            service = "op",
            function = "identity",
            args = [json!(["more"])]
        ),
        executed_state::canon(json!(
            {
                "tetraplet": {"function_name": "", "lens": "", "peer_pk": "vm_peer_id", "service_id": ""},
                "values": [
                    {
                        "result": "push more",
                        "tetraplet": {"function_name": "", "lens": "", "peer_pk": "", "service_id": ""},
                        "trace_pos": 0
                    }
                ]
            }
        )),
        unused!(
            json!(["push more"]),
            peer = vm_peer_id,
            service = "callbackSrv",
            function = "response",
            args = [json!(["push more"])]
        ),
        executed_state::canon(json!({
            "tetraplet": {"function_name": "", "lens": "", "peer_pk": "vm_peer_id", "service_id": ""},
            "values": [
                {
                    "result": "push more",
                    "tetraplet": {"function_name": "", "lens": "", "peer_pk": "", "service_id": ""},
                    "trace_pos": 1
                }
            ]
        })),
        unused!(
            json!(["push more"]),
            peer = vm_peer_id,
            service = "callbackSrv",
            function = "response",
            args = [json!(["push more"])]
        ),
        executed_state::canon(json!({
            "tetraplet": {"function_name": "", "lens": "", "peer_pk": "vm_peer_id", "service_id": ""},
            "values": [
            ]
        })),
        unused!(
            json!([]),
            peer = vm_peer_id,
            service = "callbackSrv",
            function = "response",
            args = [json!([])]
        ),
        unused!(
            json!(["more"]),
            peer = vm_peer_id,
            service = "callbackSrv",
            function = "response",
            args = [json!(["more"])]
        ),
    ]);
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
            (call "{set_variable_peer_id}" ("" "") [] iterable)
            (fold iterable x
                (seq
                    (new $s1
                        (seq
                            (ap "none" $s1)
                            (seq
                                (canon "{vm_peer_id}" $s1 #canon_s1)
                                (call "{vm_peer_id}" ("" "") [#canon_s1] s-fix1) ;; should contains only "none" on each iteration
                            )
                        )
                    )
                    (next x)
                )
            )
        )
            "#
    );

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(vm, <_>::default(), script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![
        scalar!(json!([1, 2, 3, 4, 5]), peer = set_variable_peer_id),
        executed_state::ap(0),
        executed_state::canon(json!({
            "tetraplet": {"function_name": "", "lens": "", "peer_pk": "vm_peer_id", "service_id": ""},
            "values": [
                {
                    "result": "none",
                    "tetraplet": {"function_name": "", "lens": "", "peer_pk": "", "service_id": ""},
                    "trace_pos": 1
                }
            ]
        })),
        scalar!(json!(["none"]), peer = vm_peer_id, args = [json!(["none"])]),
        executed_state::ap(0),
        executed_state::canon(json!({
            "tetraplet": {"function_name": "", "lens": "", "peer_pk": "vm_peer_id", "service_id": ""},
            "values": [
                {
                    "result": "none",
                    "tetraplet": {"function_name": "", "lens": "", "peer_pk": "", "service_id": ""},
                    "trace_pos": 4
                }
            ]
        })),
        scalar!(json!(["none"]), peer = vm_peer_id, args = [json!(["none"])]),
        executed_state::ap(0),
        executed_state::canon(json!({
            "tetraplet": {"function_name": "", "lens": "", "peer_pk": "vm_peer_id", "service_id": ""},
            "values": [
                {
                    "result": "none",
                    "tetraplet": {"function_name": "", "lens": "", "peer_pk": "", "service_id": ""},
                    "trace_pos": 7
                }
            ]
        })),
        scalar!(json!(["none"]), peer = vm_peer_id, args = [json!(["none"])]),
        executed_state::ap(0),
        executed_state::canon(json!({
            "tetraplet": {"function_name": "", "lens": "", "peer_pk": "vm_peer_id", "service_id": ""},
            "values": [
                {
                    "result": "none",
                    "tetraplet": {"function_name": "", "lens": "", "peer_pk": "", "service_id": ""},
                    "trace_pos": 10
                }
            ]
        })),
        scalar!(json!(["none"]), peer = vm_peer_id, args = [json!(["none"])]),
        executed_state::ap(0),
        executed_state::canon(json!({
            "tetraplet": {"function_name": "", "lens": "", "peer_pk": "vm_peer_id", "service_id": ""},
            "values": [
                {
                    "result": "none",
                    "tetraplet": {"function_name": "", "lens": "", "peer_pk": "", "service_id": ""},
                    "trace_pos": 13
                }
            ]
        })),
        scalar!(json!(["none"]), peer = vm_peer_id, args = [json!(["none"])]),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn new_with_streams_with_errors() {
    let fallible_peer_id = "fallible_peer_id";
    let mut fallible_vm = create_avm(fallible_call_service("service_id_1"), fallible_peer_id);

    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_call_service(), local_peer_id);

    let script = format!(
        r#"
            (seq
                (call "{local_peer_id}" ("" "") [1] $global_stream) ;; this stream should precense in a data
                (new $restricted_stream_1
                    (seq
                        (new $restricted_stream_2
                            (seq
                                (call "{local_peer_id}" ("" "") [2] $restricted_stream_2) ;; should have generation 1 in a data
                                (call "{fallible_peer_id}" ("service_id_1" "local_fn_name") [] result)
                            )
                        )
                        (call "{local_peer_id}" ("" "") [2] restricted_stream_1) ;; should have generation 0 in a data
                    )
                )
            )"#
    );

    let result = checked_call_vm!(vm, <_>::default(), &script, "", "");
    let result = call_vm!(fallible_vm, <_>::default(), script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![
        stream!(1, 0, peer = local_peer_id, args = [1]),
        stream!(2, 0, peer = local_peer_id, args = [2]),
        failed!(
            1,
            "failed result from fallible_call_service",
            peer = fallible_peer_id,
            service = "service_id_1",
            function = "local_fn_name"
        ),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn new_with_scalars_with_errors() {
    let set_variable_peer_id = "set_variable_peer_id";
    let variables_mapping = maplit::hashmap! {
        "global".to_string() => json!(1),
        "scoped".to_string() => json!(2),
    };
    let mut set_variable_vm = create_avm(
        set_variables_call_service(variables_mapping, VariableOptionSource::Argument(0)),
        set_variable_peer_id,
    );

    let variable_receiver_peer_id = "variable_receiver_peer_id";
    let mut variable_receiver_vm = create_avm(echo_call_service(), variable_receiver_peer_id);

    let fallible_peer_id = "fallible_peer_id";
    let fallible_service_id = "fallible_service_id";
    let mut fallible_peer_vm = create_avm(fallible_call_service(fallible_service_id), fallible_peer_id);

    let script = format!(
        r#"
            (seq
                (seq
                    (call "{set_variable_peer_id}" ("" "") ["global"] scalar)
                    (xor
                        (new scalar
                            (seq
                                (call "{set_variable_peer_id}" ("" "") ["scoped"] scalar)
                                (seq
                                    (call "{variable_receiver_peer_id}" ("" "") [scalar])
                                    (call "{fallible_peer_id}" ("{fallible_service_id}" "") [])
                                )
                            )
                        )
                        (call "{variable_receiver_peer_id}" ("" "") [scalar])
                    )
                )
                (call "{variable_receiver_peer_id}" ("" "") [scalar])
            )"#
    );

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(variable_receiver_vm, <_>::default(), &script, "", result.data);
    let result = checked_call_vm!(fallible_peer_vm, <_>::default(), &script, "", result.data);
    let result = checked_call_vm!(variable_receiver_vm, <_>::default(), &script, "", result.data);
    let actual_trace = trace_from_result(&result);

    let msg = r#"failed result from fallible_call_service"#;
    let expected_trace = ExecutionTrace::from(vec![
        scalar!(1, peer = set_variable_peer_id, args = ["global"]),
        scalar!(2, peer = set_variable_peer_id, args = ["scoped"]),
        unused!(2, peer = variable_receiver_peer_id, args = [2]),
        failed!(1, msg, peer = fallible_peer_id, service = fallible_service_id),
        unused!(1, peer = variable_receiver_peer_id, args = [1]),
        unused!(1, peer = variable_receiver_peer_id, args = [1]),
    ]);
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn new_with_global_scalars() {
    let set_variable_peer_id = "set_variable_peer_id";
    let variables_mapping = maplit::hashmap! {
        "global".to_string() => json!(1),
        "scoped".to_string() => json!(2),
    };
    let mut set_variable_vm = create_avm(
        set_variables_call_service(variables_mapping, VariableOptionSource::Argument(0)),
        set_variable_peer_id,
    );

    let variable_receiver_peer_id = "variable_receiver_peer_id";
    let mut variable_receiver = create_avm(echo_call_service(), variable_receiver_peer_id);

    let script = format!(
        r#"
            (seq
                (seq
                    (call "{set_variable_peer_id}" ("" "") ["global"] scalar)
                    (new scalar
                        (seq
                            (call "{set_variable_peer_id}" ("" "") ["scoped"] scalar)
                            (call "{variable_receiver_peer_id}" ("" "") [scalar])
                        )
                    )
                )
                (call "{variable_receiver_peer_id}" ("" "") [scalar])
            )"#
    );

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(variable_receiver, <_>::default(), &script, "", result.data);
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        scalar!(1, peer = set_variable_peer_id, args = ["global"]),
        scalar!(2, peer = set_variable_peer_id, args = ["scoped"]),
        unused!(2, peer = variable_receiver_peer_id, args = [2]),
        unused!(1, peer = variable_receiver_peer_id, args = [1]),
    ];
    assert_eq!(actual_trace, expected_trace);
}

const GET_ITERABLE_ACTION_NAME: &str = "get_iterable_action_name";
const OUTSIDE_ACTION_NAME: &str = "outside_new";
const INSIDE_ACTION_NAME: &str = "inside_new";
const OUTPUT_ACTION_NAME: &str = "output";

fn prepare_new_test_call_service() -> CallServiceClosure {
    let outside_new_id = std::cell::Cell::new(0u32);
    let inside_new_id = std::cell::Cell::new(10u32);

    Box::new(move |mut params| {
        let action = params.arguments.remove(0);
        let action = action.as_str().unwrap();
        match action {
            GET_ITERABLE_ACTION_NAME => CallServiceResult::ok(json!([1, 2, 3])),
            OUTSIDE_ACTION_NAME => {
                let outside_result = outside_new_id.get();
                outside_new_id.set(outside_result + 1);
                CallServiceResult::ok(json!(outside_result))
            }
            INSIDE_ACTION_NAME => {
                let inside_result = inside_new_id.get();
                inside_new_id.set(inside_result + 1);
                CallServiceResult::ok(json!(inside_result))
            }
            OUTPUT_ACTION_NAME => {
                let second_argument = params.arguments.remove(0);
                CallServiceResult::ok(second_argument)
            }
            action_name => {
                println!("unknown action: {action_name:?}");
                CallServiceResult::err(1, json!("no such action"))
            }
        }
    })
}

#[test]
fn new_with_scalars_in_lfold_with_outside_next() {
    let test_peer_id = "test_peer_id";

    let test_call_service = prepare_new_test_call_service();
    let mut test_vm = create_avm(test_call_service, test_peer_id);

    let script = format!(
        r#"
    (seq
        (call "{test_peer_id}" ("" "") ["{GET_ITERABLE_ACTION_NAME}"] iterable)
        (fold iterable iterator
            (seq
                (seq
                    (seq
                        (call "{test_peer_id}" ("" "") ["{OUTSIDE_ACTION_NAME}"] scalar)
                        (call "{test_peer_id}" ("" "") ["{OUTPUT_ACTION_NAME}" scalar])
                    )
                    (new scalar
                        (seq
                            (call "{test_peer_id}" ("" "") ["{INSIDE_ACTION_NAME}"] scalar)
                            (call "{test_peer_id}" ("" "") ["{OUTPUT_ACTION_NAME}" scalar])
                        )
                    )
                )
                (seq
                    (next iterator)
                    (call "{test_peer_id}" ("" "") ["{OUTPUT_ACTION_NAME}" scalar])
                )
            )
        )
    )
    "#
    );

    let result = checked_call_vm!(test_vm, <_>::default(), &script, "", "");
    let actual_trace = trace_from_result(&result);

    let expected_trace = ExecutionTrace::from(vec![
        scalar!(json!([1, 2, 3]), peer = test_peer_id, args = [GET_ITERABLE_ACTION_NAME]),
        scalar!(0, peer = test_peer_id, args = [json!(OUTSIDE_ACTION_NAME)]),
        unused!(0, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(0)]),
        scalar!(10, peer = test_peer_id, args = [INSIDE_ACTION_NAME]),
        unused!(10, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(10)]),
        scalar!(1, peer = test_peer_id, args = [json!(OUTSIDE_ACTION_NAME)]),
        unused!(1, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(1)]),
        scalar!(11, peer = test_peer_id, args = [INSIDE_ACTION_NAME]),
        unused!(11, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(11)]),
        scalar!(2, peer = test_peer_id, args = [json!(OUTSIDE_ACTION_NAME)]),
        unused!(2, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(2)]),
        scalar!(12, peer = test_peer_id, args = [INSIDE_ACTION_NAME]),
        unused!(12, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(12)]),
        unused!(2, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(2)]),
        unused!(1, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(1)]),
        unused!(0, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(0)]),
    ]);
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn new_with_scalars_in_rfold_with_outside_next() {
    let test_peer_id = "test_peer_id";

    let test_call_service = prepare_new_test_call_service();
    let mut test_vm = create_avm(test_call_service, test_peer_id);

    let script = format!(
        r#"
    (seq
        (call "{test_peer_id}" ("" "") ["{GET_ITERABLE_ACTION_NAME}"] iterable)
        (fold iterable iterator
            (seq
                (next iterator)
                (seq
                    (seq
                        (call "{test_peer_id}" ("" "") ["{OUTSIDE_ACTION_NAME}"] scalar)
                        (call "{test_peer_id}" ("" "") ["{OUTPUT_ACTION_NAME}" scalar])
                    )
                    (seq
                        (new scalar
                            (seq
                                (call "{test_peer_id}" ("" "") ["{INSIDE_ACTION_NAME}"] scalar)
                                (call "{test_peer_id}" ("" "") ["{OUTPUT_ACTION_NAME}" scalar])
                            )
                        )
                        (call "{test_peer_id}" ("" "") ["{OUTPUT_ACTION_NAME}" scalar])
                    )
                )
            )
        )
    )
    "#
    );

    let result = checked_call_vm!(test_vm, <_>::default(), &script, "", "");
    let actual_trace = trace_from_result(&result);

    let expected_trace = ExecutionTrace::from(vec![
        scalar!(json!([1, 2, 3]), peer = test_peer_id, args = [GET_ITERABLE_ACTION_NAME]),
        scalar!(0, peer = test_peer_id, args = [OUTSIDE_ACTION_NAME]),
        unused!(0, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(0)]),
        scalar!(10, peer = test_peer_id, args = [json!(INSIDE_ACTION_NAME)]),
        unused!(10, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(10)]),
        unused!(0, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(0)]),
        scalar!(1, peer = test_peer_id, args = [OUTSIDE_ACTION_NAME]),
        unused!(1, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(1)]),
        scalar!(11, peer = test_peer_id, args = [json!(INSIDE_ACTION_NAME)]),
        unused!(11, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(11)]),
        unused!(1, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(1)]),
        scalar!(2, peer = test_peer_id, args = [OUTSIDE_ACTION_NAME]),
        unused!(2, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(2)]),
        scalar!(12, peer = test_peer_id, args = [json!(INSIDE_ACTION_NAME)]),
        unused!(12, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(12)]),
        unused!(2, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(2)]),
    ]);
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn new_with_scalars_in_fold_with_inside_next() {
    let test_peer_id = "test_peer_id";

    let test_call_service = prepare_new_test_call_service();
    let mut test_vm = create_avm(test_call_service, test_peer_id);

    let script = format!(
        r#"
    (seq
        (call "{test_peer_id}" ("" "") ["{GET_ITERABLE_ACTION_NAME}"] iterable)
        (fold iterable iterator
            (seq
                (seq
                    (seq
                        (call "{test_peer_id}" ("" "") ["{OUTSIDE_ACTION_NAME}"] scalar)
                        (call "{test_peer_id}" ("" "") ["{OUTPUT_ACTION_NAME}" scalar])
                    )
                    (new scalar
                        (seq
                            (call "{test_peer_id}" ("" "") ["{INSIDE_ACTION_NAME}"] scalar)
                            (seq
                                (call "{test_peer_id}" ("" "") ["{OUTPUT_ACTION_NAME}" scalar])
                                (seq
                                    (next iterator)
                                    (call "{test_peer_id}" ("" "") ["{OUTPUT_ACTION_NAME}" scalar])
                                )
                            )
                        )
                    )
                )
                (call "{test_peer_id}" ("" "") ["{OUTPUT_ACTION_NAME}" scalar])
            )
        )
    )
    "#
    );

    let result = checked_call_vm!(test_vm, <_>::default(), &script, "", "");
    let actual_trace = trace_from_result(&result);

    let expected_trace = ExecutionTrace::from(vec![
        scalar!(json!([1, 2, 3]), peer = test_peer_id, args = [GET_ITERABLE_ACTION_NAME]),
        scalar!(0, peer = test_peer_id, args = [OUTSIDE_ACTION_NAME]),
        unused!(0, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(0)]),
        scalar!(10, peer = test_peer_id, args = [json!(INSIDE_ACTION_NAME)]),
        unused!(10, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(10)]),
        scalar!(1, peer = test_peer_id, args = [OUTSIDE_ACTION_NAME]),
        unused!(1, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(1)]),
        scalar!(11, peer = test_peer_id, args = [json!(INSIDE_ACTION_NAME)]),
        unused!(11, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(11)]),
        scalar!(2, peer = test_peer_id, args = [OUTSIDE_ACTION_NAME]),
        unused!(2, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(2)]),
        scalar!(12, peer = test_peer_id, args = [json!(INSIDE_ACTION_NAME)]),
        unused!(12, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(12)]),
        unused!(12, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(12)]),
        unused!(2, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(2)]),
        unused!(11, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(11)]),
        unused!(1, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(1)]),
        unused!(10, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(10)]),
        unused!(0, peer = test_peer_id, args = [json!(OUTPUT_ACTION_NAME), json!(0)]),
    ]);
    assert_eq!(actual_trace, expected_trace);
}
