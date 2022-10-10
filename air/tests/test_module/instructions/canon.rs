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

use std::ops::Deref;

#[test]
fn canon_moves_execution_flow() {
    let mut vm = create_avm(echo_call_service(), "A");
    let peer_id_1 = "peer_id_1";
    let peer_id_2 = "peer_id_2";

    let script = f!(r#"
            (par
                (call "{peer_id_1}" ("" "") [] $stream)
                (canon "{peer_id_2}" $stream #canon_stream)
            )"#);

    let result = checked_call_vm!(vm, <_>::default(), script, "", "");

    assert_next_pks!(&result.next_peer_pks, &[peer_id_1, peer_id_2]);
}

#[test]
fn basic_canon() {
    let mut vm = create_avm(echo_call_service(), "A");
    let mut set_variable_vm = create_avm(
        set_variable_call_service(json!(["1", "2", "3", "4", "5"])),
        "set_variable",
    );

    let script = r#"
            (seq
                (call "set_variable" ("" "") [] Iterable)
                (seq
                    (fold Iterable i
                        (seq
                            (call "A" ("" "") [i] $stream)
                            (next i)))
                    (canon "A" $stream #canon_stream)))
                    "#;

    let result = checked_call_vm!(set_variable_vm, <_>::default(), script, "", "");
    let result = checked_call_vm!(vm, <_>::default(), script, "", result.data);
    let actual_state = &trace_from_result(&result)[6.into()];

    let expected_state = executed_state::canon_new(
        json!({"tetraplet": {"function_name": "", "json_path": "", "peer_pk": "A", "service_id": ""},
        "values": [{"result": "1", "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "A", "service_id": ""}, "trace_pos": 1},
            {"result": "2", "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "A", "service_id": ""}, "trace_pos": 2},
            {"result": "3", "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "A", "service_id": ""}, "trace_pos": 3},
            {"result": "4", "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "A", "service_id": ""}, "trace_pos": 4},
            {"result": "5", "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "A", "service_id": ""}, "trace_pos": 5}]}),
    );
    assert_eq!(actual_state, &expected_state);
}

#[test]
fn canon_fixes_stream_correct() {
    let peer_id_1 = "peer_id_1";
    let mut vm_1 = create_avm(echo_call_service(), peer_id_1);
    let peer_id_2 = "peer_id_2";
    let mut vm_2 = create_avm(echo_call_service(), peer_id_2);
    let peer_id_3 = "peer_id_3";
    let mut vm_3 = create_avm(echo_call_service(), peer_id_3);
    let peer_id_4 = "peer_id_4";
    let mut vm_4 = create_avm(echo_call_service(), peer_id_4);

    let script = f!(r#"
        (seq
            (par
                (call "{peer_id_1}" ("" "") [1] $stream)
                (par
                     (call "{peer_id_2}" ("" "") [2] $stream)
                     (call "{peer_id_3}" ("" "") [3] $stream)))
            (seq
                (call "{peer_id_4}" ("" "") [4])
                (seq
                     (canon "{peer_id_3}" $stream #canon_stream)
                     (par
                         (call "{peer_id_3}" ("" "") [#canon_stream])
                         (call "{peer_id_1}" ("" "") [#canon_stream])))))
            "#);

    let vm_1_result_1 = checked_call_vm!(vm_1, <_>::default(), &script, "", "");
    let vm_2_result = checked_call_vm!(vm_2, <_>::default(), &script, "", "");
    let vm_3_result_1 = checked_call_vm!(vm_3, <_>::default(), &script, "", vm_2_result.data);
    let vm_4_result = checked_call_vm!(vm_4, <_>::default(), &script, "", vm_3_result_1.data.clone());
    let vm_3_result_2 = checked_call_vm!(vm_3, <_>::default(), &script, vm_3_result_1.data, vm_4_result.data);
    let actual_vm_3_result_2_trace = trace_from_result(&vm_3_result_2);
    let expected_vm_3_result_2_trace = vec![
        executed_state::par(1, 3),
        executed_state::request_sent_by(peer_id_2),
        executed_state::par(1, 1),
        executed_state::stream_number(2, 0),
        executed_state::stream_number(3, 1),
        executed_state::scalar_number(4),
        executed_state::canon_new(
            json!({"tetraplet": {"function_name": "", "json_path": "", "peer_pk": "peer_id_3", "service_id": ""},
            "values": [{"result": 2, "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "peer_id_2", "service_id": ""}, "trace_pos": 3},
                {"result": 3, "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "peer_id_3", "service_id": ""}, "trace_pos": 4}]}),
        ),
        executed_state::par(1, 1),
        executed_state::scalar(json!([2, 3])),
        executed_state::request_sent_by(peer_id_3),
    ];
    assert_eq!(actual_vm_3_result_2_trace, expected_vm_3_result_2_trace);

    let vm_1_result_2 = checked_call_vm!(vm_1, <_>::default(), script, vm_1_result_1.data, vm_3_result_2.data);
    let vm_1_result_2_trace = trace_from_result(&vm_1_result_2);
    let expected_vm_1_result_2_trace = vec![
        executed_state::par(1, 3),
        executed_state::stream_number(1, 0),
        executed_state::par(1, 1),
        executed_state::stream_number(2, 1),
        executed_state::stream_number(3, 2),
        executed_state::scalar_number(4),
        executed_state::canon_new(
            json!({"tetraplet": {"function_name": "", "json_path": "", "peer_pk": "peer_id_3", "service_id": ""},
            "values": [{"result": 2, "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "peer_id_2", "service_id": ""}, "trace_pos": 3},
                {"result": 3, "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "peer_id_3", "service_id": ""}, "trace_pos": 4}]}),
        ),
        executed_state::par(1, 1),
        executed_state::scalar(json!([2, 3])),
        executed_state::scalar(json!([2, 3])),
    ];
    assert_eq!(vm_1_result_2_trace.deref(), expected_vm_1_result_2_trace);
}

#[test]
fn canon_stream_can_be_created_from_aps() {
    let vm_1_peer_id = "vm_1_peer_id";
    let mut vm_1 = create_avm(echo_call_service(), vm_1_peer_id);

    let vm_2_peer_id = "vm_2_peer_id";
    let mut vm_2 = create_avm(echo_call_service(), vm_2_peer_id);

    let script = f!(r#"
        (seq
            (seq
                (ap 0 $stream)
                (ap 1 $stream))
            (seq
                (canon "{vm_1_peer_id}" $stream #canon_stream)
                (seq
                    (ap #canon_stream $stream_2)
                    (call "{vm_2_peer_id}" ("" "") [$stream_2]))))
        "#);

    let result_1 = checked_call_vm!(vm_1, <_>::default(), &script, "", "");
    let result_2 = checked_call_vm!(vm_2, <_>::default(), &script, "", result_1.data.clone());
    // it fails on this call if canon merger can't handle ap results
    let _ = checked_call_vm!(vm_2, <_>::default(), &script, result_1.data, result_2.data);
}

#[test]
fn canon_gates() {
    let peer_id_1 = "peer_id_1";
    let mut vm_1 = create_avm(set_variable_call_service(json!([1, 2, 3, 4, 5])), peer_id_1);

    let peer_id_2 = "peer_id_2";
    let mut vm_2 = create_avm(echo_call_service(), peer_id_2);

    let peer_id_3 = "peer_id_3";
    let stop_len_count = 2;
    let vm_3_call_service: CallServiceClosure = Box::new(move |params: CallRequestParams| -> CallServiceResult {
        let value = params.arguments[0].as_array().unwrap().len();
        if value >= stop_len_count {
            CallServiceResult::ok(json!(true))
        } else {
            CallServiceResult::ok(json!(false))
        }
    });
    let mut vm_3 = create_avm(vm_3_call_service, peer_id_3);

    let script = f!(r#"
        (seq
          (seq
            (call "{peer_id_1}" ("" "") [] iterable)
            (fold iterable iterator
              (par
                (call "{peer_id_2}" ("" "") [iterator] $stream)
                (next iterator))))
          (new $tmp
            (fold $stream s
              (xor
                (seq
                  (ap s $tmp)
                  (seq
                    (seq
                      (canon "{peer_id_3}" $tmp #t)
                      (call "{peer_id_3}" ("" "") [#t] x))
                    (match x true
                      (call "{peer_id_3}" ("" "") [#t]))))
                (next s)))))
            "#);

    let vm_1_result = checked_call_vm!(vm_1, <_>::default(), &script, "", "");
    let vm_2_result = checked_call_vm!(vm_2, <_>::default(), &script, "", vm_1_result.data);
    let vm_3_result = checked_call_vm!(vm_3, <_>::default(), &script, "", vm_2_result.data);

    let actual_trace = trace_from_result(&vm_3_result);
    let fold = match &actual_trace[11.into()] {
        ExecutedState::Fold(fold_result) => fold_result,
        _ => unreachable!(),
    };

    // fold should stop at the correspond len
    assert_eq!(fold.lore.len(), stop_len_count);
}

#[test]
fn canon_empty_stream() {
    let peer_id_1 = "peer_id_1";
    let mut vm_1 = create_avm(echo_call_service(), peer_id_1);
    let peer_id_2 = "peer_id_2";
    let mut vm_2 = create_avm(echo_call_service(), peer_id_2);

    let script = f!(r#"
            (new $stream
                (seq
                    (canon "{peer_id_1}" $stream #canon_stream)
                    (call "{peer_id_1}" ("" "") [#canon_stream])))
                    "#);

    let result = checked_call_vm!(vm_1, <_>::default(), &script, "", "");
    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![
        executed_state::canon_new(
            json!({"tetraplet": {"function_name": "", "json_path": "", "peer_pk": "peer_id_1", "service_id": ""}, "values": []}),
        ),
        executed_state::scalar(json!([])),
    ];
    assert_eq!(actual_trace, expected_trace);

    let result = checked_call_vm!(vm_2, <_>::default(), script, "", result.data);
    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![
        executed_state::canon_new(
            json!({"tetraplet": {"function_name": "", "json_path": "", "peer_pk": "peer_id_1", "service_id": ""}, "values": []} ),
        ),
        executed_state::scalar(json!([])),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn canon_over_later_defined_stream() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let mut peer_vm_1 = create_avm(echo_call_service(), vm_peer_id_1);

    let vm_peer_id_2 = "vm_peer_id_2";
    let mut peer_vm_2 = create_avm(echo_call_service(), vm_peer_id_2);

    let vm_peer_id_3 = "vm_peer_id_3";
    let mut peer_vm_3 = create_avm(echo_call_service(), vm_peer_id_3);

    let script = f!(r#"
        (par
            (call "{vm_peer_id_2}" ("" "") [1] $stream)
            (seq
                (canon "{vm_peer_id_1}" $stream #canon_stream) ; it returns a catchable error
                (call "{vm_peer_id_3}" ("" "") [#canon_stream])
            )
        )
    "#);

    let result = checked_call_vm!(peer_vm_1, <_>::default(), &script, "", "");
    let result = checked_call_vm!(peer_vm_2, <_>::default(), &script, "", result.data);
    let result = checked_call_vm!(peer_vm_3, <_>::default(), &script, "", result.data);
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        executed_state::par(1, 2),
        executed_state::stream_number(1, 0),
        executed_state::canon_new(
            json!({"tetraplet": {"function_name": "", "json_path": "", "peer_pk": "vm_peer_id_1", "service_id": ""},"values": []}),
        ),
        executed_state::scalar(json!([])),
    ];
    assert_eq!(actual_trace, expected_trace);
}
