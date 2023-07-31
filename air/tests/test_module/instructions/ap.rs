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

use air::no_error_last_error_object;
use air::ExecutionCidState;
use air_test_framework::AirScriptExecutor;
use air_test_utils::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn ap_with_scalars() {
    let vm_1_peer_id = "vm_1_peer_id";
    let test_value = "scalar_2";
    let mut vm_1 = create_avm(set_variable_call_service(json!({ "field": test_value })), vm_1_peer_id);

    let vm_2_peer_id = "vm_2_peer_id";
    let mut vm_2 = create_avm(echo_call_service(), vm_2_peer_id);

    let script = format!(
        r#"
        (seq
            (seq
                (call "{vm_1_peer_id}" ("" "") ["scalar_1_result"] scalar_1)
                (ap scalar_1.$.field! scalar_2)
            )
            (call "{vm_2_peer_id}" ("" "") [scalar_2])
        )
        "#
    );

    let result = checked_call_vm!(vm_1, <_>::default(), &script, "", "");
    let result = checked_call_vm!(vm_2, <_>::default(), script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_state = vec![
        scalar!(
            json!({ "field": test_value }),
            peer = vm_1_peer_id,
            args = ["scalar_1_result"]
        ),
        unused!(test_value, peer = vm_2_peer_id, args = [test_value]),
    ];

    assert_eq!(actual_trace, expected_state);
    assert!(result.next_peer_pks.is_empty());
}

#[test]
fn ap_with_string_literal() {
    let vm_1_peer_id = "vm_1_peer_id";
    let mut vm_1 = create_avm(echo_call_service(), vm_1_peer_id);

    let some_string = "some_string";
    let script = format!(
        r#"
        (seq
            (ap "{some_string}" $stream)
            (seq
                (canon "{vm_1_peer_id}" $stream #canon_stream)
                (call "{vm_1_peer_id}" ("" "") [#canon_stream])))
        "#
    );

    let result = checked_call_vm!(vm_1, <_>::default(), script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_state = vec![
        executed_state::ap(0),
        executed_state::canon(json!(
            {
            "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "vm_1_peer_id", "service_id": ""},
            "values": [
                {
                    "result": "some_string",
                    "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "", "service_id": ""},
                    "trace_pos": 0
                }
            ]
        }
        )),
        unused!(json!([some_string]), peer = vm_1_peer_id, args = [json!([some_string])]),
    ];

    assert_eq!(actual_trace, expected_state);
    assert!(result.next_peer_pks.is_empty());
}

#[test]
fn ap_with_bool_literal() {
    let vm_1_peer_id = "vm_1_peer_id";
    let mut vm_1 = create_avm(echo_call_service(), vm_1_peer_id);

    let script = format!(
        r#"
        (seq
            (ap true $stream)
            (seq
                (canon "{vm_1_peer_id}" $stream #canon_stream)
                (call "{vm_1_peer_id}" ("" "") [#canon_stream])))
        "#
    );

    let result = checked_call_vm!(vm_1, <_>::default(), script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_state = vec![
        executed_state::ap(0),
        executed_state::canon(json!(   {
            "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "vm_1_peer_id", "service_id": ""},
            "values": [
                {
                    "result": true,
                    "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "", "service_id": ""},
                    "trace_pos": 0
                }
            ]
        })),
        unused!(json!([true]), peer = vm_1_peer_id, args = [json!([true])]),
    ];

    assert_eq!(actual_trace, expected_state);
    assert!(result.next_peer_pks.is_empty());
}

#[test]
fn ap_with_number_literal() {
    let vm_1_peer_id = "vm_1_peer_id";
    let mut vm_1 = create_avm(echo_call_service(), vm_1_peer_id);

    let script = format!(
        r#"
        (seq
            (ap 100 $stream)
            (seq
                (canon "{vm_1_peer_id}" $stream #canon_stream)
                (call "{vm_1_peer_id}" ("" "") [#canon_stream])))
        "#
    );

    let result = checked_call_vm!(vm_1, <_>::default(), script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_state = vec![
        executed_state::ap(0),
        executed_state::canon(json!({
            "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "vm_1_peer_id", "service_id": ""},
            "values": [
                {
                    "result": 100,
                    "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "", "service_id": ""},
                    "trace_pos": 0
                }
            ]
        })),
        unused!(json!([100]), peer = vm_1_peer_id, args = [json!([100])]),
    ];

    assert_eq!(actual_trace, expected_state);
    assert!(result.next_peer_pks.is_empty());
}

#[test]
fn ap_with_last_error() {
    let vm_1_peer_id = "vm_1_peer_id";
    let mut vm_1 = create_avm(echo_call_service(), vm_1_peer_id);

    let script = format!(
        r#"
        (seq
            (ap %last_error% $stream)
            (seq
                (canon "{vm_1_peer_id}" $stream #canon_stream)
                (call "{vm_1_peer_id}" ("" "") [#canon_stream])))
        "#
    );

    let result = checked_call_vm!(vm_1, <_>::default(), script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_state = vec![
        executed_state::ap(0),
        executed_state::canon(json!({
            "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "vm_1_peer_id", "service_id": ""},
            "values": [
                {
                    "result": no_error_last_error_object(),
                    "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "", "service_id": ""},
                    "trace_pos": 0
                }
            ]
        })),
        unused!(
            json!([no_error_last_error_object()]),
            peer = vm_1_peer_id,
            args = [no_error_last_error_object()]
        ),
    ];

    assert_eq!(actual_trace, expected_state);
    assert!(result.next_peer_pks.is_empty());
}

#[test]
fn ap_with_timestamp() {
    let vm_1_peer_id = "vm_1_peer_id";
    let mut vm_1 = create_avm(echo_call_service(), vm_1_peer_id);

    let script = format!(
        r#"
        (seq
            (ap %timestamp% scalar)
            (call "{vm_1_peer_id}" ("" "") [scalar])
        )
        "#
    );

    let test_params = TestRunParameters::from_timestamp(1337);
    let result = checked_call_vm!(vm_1, test_params.clone(), script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_state = vec![unused!(
        test_params.timestamp,
        peer = vm_1_peer_id,
        args = [test_params.timestamp]
    )];

    assert_eq!(actual_trace, expected_state);
}

#[test]
fn ap_with_ttl() {
    let vm_1_peer_id = "vm_1_peer_id";
    let mut vm_1 = create_avm(echo_call_service(), vm_1_peer_id);

    let script = format!(
        r#"
        (seq
            (ap %ttl% scalar)
            (call "{vm_1_peer_id}" ("" "") [scalar])
        )
        "#
    );

    let test_params = TestRunParameters::from_ttl(1337);
    let result = checked_call_vm!(vm_1, test_params.clone(), script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_state = vec![unused!(test_params.ttl, peer = vm_1_peer_id, args = [test_params.ttl])];

    assert_eq!(actual_trace, expected_state);
}

#[test]
fn ap_with_dst_stream() {
    let vm_1_peer_id = "vm_1_peer_id";
    let test_value = "scalar_2";
    let mut vm_1 = create_avm(set_variable_call_service(json!({ "field": test_value })), vm_1_peer_id);

    let vm_2_peer_id = "vm_2_peer_id";
    let mut vm_2 = create_avm(echo_call_service(), vm_2_peer_id);

    let script = format!(
        r#"
        (seq
            (seq
                (call "{vm_1_peer_id}" ("" "") ["scalar_1_result"] scalar_1)
                (ap scalar_1 $stream))
            (seq
                (canon "{vm_2_peer_id}" $stream #canon_stream)
                (call "{vm_2_peer_id}" ("" "") [#canon_stream])))
        "#
    );

    let result = checked_call_vm!(vm_1, <_>::default(), &script, "", "");
    let result = checked_call_vm!(vm_2, <_>::default(), script, "", result.data);

    let val_1 = scalar!(
        json!({ "field": test_value }),
        peer = vm_1_peer_id,
        args = ["scalar_1_result"]
    );
    let cid_1 = extract_service_result_cid(&val_1);

    let actual_trace = trace_from_result(&result);
    let expected_state = vec![
        val_1,
        executed_state::ap(0),
        executed_state::canon(json!({
            "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "vm_2_peer_id", "service_id": ""},
            "values": [{
                "result": {"field": "scalar_2"},
                "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "vm_1_peer_id", "service_id": ""},
                "provenance": Provenance::service_result(cid_1),
            }]
        })),
        unused!(
            json!([{ "field": test_value }]),
            peer = vm_2_peer_id,
            args = [json!([{ "field": test_value }])]
        ),
    ];

    assert_eq!(actual_trace, expected_state);
    assert!(result.next_peer_pks.is_empty());
}

#[test]
fn ap_canon_stream_with_lambda() {
    let vm_1_peer_id = "vm_1_peer_id";
    let (echo_call_service, tetraplet_checker) = tetraplet_host_function(echo_call_service());
    let mut vm_1 = create_avm(echo_call_service, vm_1_peer_id);

    let service_name = "some_service_name";
    let function_name = "some_function_name";
    let script = format!(
        r#"
        (seq
            (seq
                (call "{vm_1_peer_id}" ("" "") [0] $stream)
                (call "{vm_1_peer_id}" ("{service_name}" "{function_name}") [1] $stream))
            (seq
                (canon "{vm_1_peer_id}" $stream #canon_stream)
                (seq
                    (ap #canon_stream.$.[1] $stream_2)
                    (seq
                        (canon "{vm_1_peer_id}" $stream_2 #canon_stream_2)
                        (call "{vm_1_peer_id}" ("" "") [#canon_stream_2])))))
        "#
    );

    let result = checked_call_vm!(vm_1, <_>::default(), &script, "", "");

    let val_1 = stream!(0, 0, peer = vm_1_peer_id, args = [0]);
    let val_2 = stream!(
        1,
        1,
        peer = vm_1_peer_id,
        service = service_name,
        function = function_name,
        args = [1]
    );
    let cid_1 = extract_service_result_cid(&val_1);
    let cid_2 = extract_service_result_cid(&val_2);

    let canon_1 = executed_state::canon(json!({
    "tetraplet": {
        "function_name": "",
        "json_path": "",
        "peer_pk": "vm_1_peer_id",
        "service_id": "",
    },
    "values": [{
        "result": 0,
        "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "vm_1_peer_id", "service_id": ""},
        "provenance": Provenance::service_result(cid_1),
    }, {
        "result": 1,
        "tetraplet": {
            "function_name": "some_function_name",
            "json_path": "",
            "peer_pk": "vm_1_peer_id",
            "service_id": "some_service_name",
        },
        "provenance": Provenance::service_result(cid_2.clone()),
    }]}));

    let actual_trace = trace_from_result(&result);
    let expected_state = vec![
        val_1,
        val_2,
        canon_1,
        executed_state::ap(0),
        executed_state::canon(json!({
            "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "vm_1_peer_id", "service_id": ""},
            "values": [{
                "result": 1,
                "tetraplet": {"function_name": "some_function_name", "json_path": "", "peer_pk": "vm_1_peer_id", "service_id": "some_service_name"},
                "provenance": Provenance::service_result(cid_2),
            }]
        })),
        unused!(json!([1]), peer = vm_1_peer_id, args = [json!([1])]),
    ];
    assert_eq!(actual_trace, expected_state);

    let expected_tetraplet = RefCell::new(vec![vec![SecurityTetraplet::new(
        vm_1_peer_id,
        service_name,
        function_name,
        "",
    )]]);
    assert_eq!(tetraplet_checker.as_ref(), &expected_tetraplet);
}

#[test]
fn ap_canon_stream() {
    let vm_1_peer_id = "vm_1_peer_id";
    let arg_tetraplets = Rc::new(RefCell::new(vec![]));

    let echo_call_service: CallServiceClosure = Box::new(move |mut params| -> CallServiceResult {
        let arg_tetraplets_inner = arg_tetraplets.clone();
        arg_tetraplets_inner.borrow_mut().push(params.tetraplets.clone());
        CallServiceResult::ok(params.arguments.remove(0))
    });

    let (echo_call_service, tetraplet_checker) = tetraplet_host_function(echo_call_service);
    let mut vm_1 = create_avm(echo_call_service, vm_1_peer_id);

    let service_name = "some_service_name";
    let function_name = "some_function_name";
    let script = format!(
        r#"
        (seq
            (seq
                (call "{vm_1_peer_id}" ("" "") [0] $stream)
                (call "{vm_1_peer_id}" ("{service_name}" "{function_name}") [1] $stream))
            (seq
                (canon "{vm_1_peer_id}" $stream #canon_stream)
                (seq
                    (ap #canon_stream $stream_2)
                    (seq
                        (canon "{vm_1_peer_id}" $stream_2 #canon_stream_2)
                        (call "{vm_1_peer_id}" ("" "") [#canon_stream_2])))))
        "#
    );

    let result = checked_call_vm!(vm_1, <_>::default(), &script, "", "");
    print_trace(&result, "");

    let val_1 = stream!(0, 0, peer = vm_1_peer_id, args = [0]);
    let val_2 = stream!(
        1,
        1,
        peer = vm_1_peer_id,
        service = service_name,
        function = function_name,
        args = [1]
    );
    let cid_1 = extract_service_result_cid(&val_1);
    let cid_2 = extract_service_result_cid(&val_2);

    let canon_1 = executed_state::canon(json!({
    "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "vm_1_peer_id", "service_id": ""},
    "values": [{
        "result": 0,
        "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "vm_1_peer_id", "service_id": ""},
        "provenance": Provenance::service_result(cid_1),
    }, {
        "result": 1,
        "tetraplet": {
            "function_name": "some_function_name",
            "json_path": "",
            "peer_pk": "vm_1_peer_id",
            "service_id": "some_service_name"
        },
        "provenance": Provenance::service_result(cid_2),
    }]}));
    let canon_cid_1 = extract_canon_result_cid(&canon_1);

    let actual_trace = trace_from_result(&result);
    let expected_state = vec![
        val_1,
        val_2,
        canon_1,
        executed_state::ap(0),
        executed_state::canon(json!({
                "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "vm_1_peer_id", "service_id": ""},
                "values": [{
                    "result": [0, 1],
                    "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "vm_1_peer_id", "service_id": ""},
                    "provenance": Provenance::canon(canon_cid_1),
                }]}
        )),
        unused!(json!([[0, 1]]), peer = vm_1_peer_id, args = [json!([[0, 1]])]),
    ];
    assert_eq!(actual_trace, expected_state);

    let expected_tetraplet = RefCell::new(vec![vec![SecurityTetraplet::new(vm_1_peer_id, "", "", "")]]);
    assert_eq!(tetraplet_checker.as_ref(), &expected_tetraplet);
}

#[test]
fn ap_stream_map() {
    let vm_1_peer_id = "vm_1_peer_id";
    let mut vm_1 = create_avm(echo_call_service(), vm_1_peer_id);

    let service_name1 = "serv1";
    let service_name2 = "serv2";
    let script = format!(
        r#"
        (seq
            (seq
                (ap ("{vm_1_peer_id}" "{service_name1}") %map)
                (ap ("{vm_1_peer_id}" "{service_name2}") %map)
            )
            (fold %map i
                (seq
                    (call i.$.key (i.$.key i.$.value) [i] u)
                    (next i)
                )
            )
        )
        "#
    );

    let result = checked_call_vm!(vm_1, <_>::default(), &script, "", "");
    let actual_trace = trace_from_result(&result);
    let generation_idx = 0;
    let mut cid_tracker = ExecutionCidState::new();
    let service_result1 = json!({
      "key": vm_1_peer_id,
      "value": service_name1,
    });
    let service_result2 = json!({
      "key": vm_1_peer_id,
      "value": service_name2,
    });
    let service_args1 = vec![service_result1.clone()];
    let service_args2 = vec![service_result2.clone()];

    let expected_state = ExecutionTrace::from(vec![
        executed_state::ap(generation_idx),
        executed_state::ap(generation_idx),
        executed_state::fold(vec![
            subtrace_lore(0, SubTraceDesc::new(3.into(), 1), SubTraceDesc::new(5.into(), 0)),
            subtrace_lore(1, SubTraceDesc::new(4.into(), 1), SubTraceDesc::new(5.into(), 0)),
        ]),
        scalar_tracked!(
            service_result1,
            cid_tracker,
            peer = vm_1_peer_id,
            service = vm_1_peer_id,
            function = service_name1,
            args = service_args1
        ),
        scalar_tracked!(
            service_result2,
            cid_tracker,
            peer = vm_1_peer_id,
            service = vm_1_peer_id,
            function = service_name2,
            args = service_args2
        ),
    ]);
    assert_eq!(actual_trace, expected_state);
}

#[test]
fn ap_stream_map_with_undefined_last_error() {
    let vm_1_peer_id = "vm_1_peer_id";
    let script = format!(
        r#"
        (seq
            (ap ("key" %last_error%) %map)
            (fold %map i
                (seq
                    (call "{vm_1_peer_id}" ("m" "f") [i.$.value]) ; behaviour = echo
                    (next i)
                )
            )
        )
        "#
    );

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(vm_1_peer_id), &script)
        .expect("invalid test AIR script");
    let result = executor.execute_all(vm_1_peer_id).unwrap();
    let actual_trace = trace_from_result(&result.last().unwrap());

    let expected_state = vec![
        executed_state::ap(0),
        executed_state::fold(vec![subtrace_lore(
            0,
            SubTraceDesc::new(2.into(), 1),
            SubTraceDesc::new(3.into(), 0),
        )]),
        unused!(
            no_error_last_error_object(),
            peer = vm_1_peer_id,
            service = "m",
            function = "f",
            args = [no_error_last_error_object()]
        ),
    ];

    assert_eq!(actual_trace, expected_state,);
}

#[test]
fn ap_canon_stream_map_with_string_key_accessor_lambda() {
    let vm_1_peer_id = "vm_1_peer_id";
    let script = format!(
        r#"
        (seq
            (seq
                (ap ("key" "value1") %map)
                (canon "{vm_1_peer_id}" %map #%canon_map)
            )
            (seq
                (ap #%canon_map.$.key scalar)
                (call "{vm_1_peer_id}" ("m" "f") [scalar] scalar1) ; behaviour = echo
            )
        )
        "#
    );

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(vm_1_peer_id), &script)
        .expect("invalid test AIR script");
    let result = executor.execute_all(vm_1_peer_id).unwrap();
    let actual_trace = trace_from_result(&result.last().unwrap());

    let mut cid_tracker: ExecutionCidState = ExecutionCidState::new();
    let map_value = json!({"key": "key", "value": "value1"});
    let tetraplet = json!({"function_name": "", "json_path": "", "peer_pk": vm_1_peer_id, "service_id": ""});

    let expected_trace: Vec<ExecutedState> = vec![
        executed_state::ap(0),
        canon_tracked(
            json!({"tetraplet": tetraplet,
            "values": [
                {
                "result": map_value,
                "tetraplet": tetraplet,
                "provenance": Provenance::Literal,
            },
            ]}),
            &mut cid_tracker,
        ),
        scalar_tracked!(
            "value1",
            cid_tracker,
            peer = vm_1_peer_id,
            service = "m..0",
            function = "f",
            args = ["value1"]
        ),
    ];

    assert_eq!(actual_trace, expected_trace,);
}

// WIP try > unit32 key value
#[test]
fn ap_canon_stream_map_with_numeric_key_accessor_lambda() {
    let vm_1_peer_id = "vm_1_peer_id";
    let script = format!(
        r#"
        (seq
            (seq
                (ap (42 "value1") %map)
                (canon "{vm_1_peer_id}" %map #%canon_map)
            )
            (seq
                (ap #%canon_map.$.[42] scalar)
                (call "{vm_1_peer_id}" ("m" "f") [scalar] scalar1) ; behaviour = echo
            )
        )
        "#
    );

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(vm_1_peer_id), &script)
        .expect("invalid test AIR script");
    let result = executor.execute_all(vm_1_peer_id).unwrap();
    let actual_trace = trace_from_result(&result.last().unwrap());

    let mut cid_tracker: ExecutionCidState = ExecutionCidState::new();
    let map_value = json!({"key": 42, "value": "value1"});
    let tetraplet = json!({"function_name": "", "json_path": "", "peer_pk": vm_1_peer_id, "service_id": ""});

    let expected_trace: Vec<ExecutedState> = vec![
        executed_state::ap(0),
        canon_tracked(
            json!({"tetraplet": tetraplet,
            "values": [
                {
                "result": map_value,
                "tetraplet": tetraplet,
                "provenance": Provenance::Literal,
            },
            ]}),
            &mut cid_tracker,
        ),
        scalar_tracked!(
            "value1",
            cid_tracker,
            peer = vm_1_peer_id,
            service = "m..0",
            function = "f",
            args = ["value1"]
        ),
    ];

    assert_eq!(actual_trace, expected_trace,);
}
