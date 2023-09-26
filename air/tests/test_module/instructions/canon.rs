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

use air::ExecutionCidState;
use air_test_framework::AirScriptExecutor;
use air_test_utils::key_utils::at;
use air_test_utils::prelude::*;
use pretty_assertions::assert_eq;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

#[test]
fn canon_moves_execution_flow() {
    let peer_id_1 = "peer_id_1";
    let peer_id_2 = "peer_id_2";
    let init_peer_id = "A";
    let mut vm = create_avm(echo_call_service(), init_peer_id);

    let script = format!(
        r#"
            (par
                (call "{peer_id_1}" ("" "") [] $stream)
                (canon "{peer_id_2}" $stream #canon_stream)
            )"#
    );

    let result = checked_call_vm!(vm, <_>::default(), script, "", "");

    assert_next_pks!(&result.next_peer_pks, &[peer_id_1, peer_id_2]);
    let trace = trace_from_result(&result);
    assert_eq!(
        &*trace,
        vec![
            par(1, 1),
            executed_state::request_sent_by(init_peer_id),
            canon_request(init_peer_id),
        ],
    )
}

#[test]
fn basic_canon() {
    let mut vm = create_avm(echo_call_service(), "A");
    let data = json!(["1", "2", "3", "4", "5"]);
    let mut set_variable_vm = create_avm(set_variable_call_service(data.clone()), "set_variable");

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

    let cids: Vec<_> = (1..=5)
        .map(|i| {
            let val = format!("{}", i);
            extract_service_result_cid(&scalar!(val.clone(), peer = "A", args = [val]))
        })
        .collect();

    let expected_state = executed_state::canon(
        json!({"tetraplet": {"function_name": "", "json_path": "", "peer_pk": "A", "service_id": ""},
        "values": [{
            "result": "1",
            "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "A", "service_id": ""},
            "provenance": Provenance::service_result(cids[0].clone()),
        }, {
            "result": "2",
            "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "A", "service_id": ""},
            "provenance": Provenance::service_result(cids[1].clone()),
        }, {
            "result": "3",
            "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "A", "service_id": ""},
            "provenance": Provenance::service_result(cids[2].clone()),
        }, {
            "result": "4",
            "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "A", "service_id": ""},
            "provenance": Provenance::service_result(cids[3].clone()),
        }, {
            "result": "5",
            "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "A", "service_id": ""},
            "provenance": Provenance::service_result(cids[4].clone()),
        }]}),
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

    let script = format!(
        r#"
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
            "#
    );

    let vm_1_result_1 = checked_call_vm!(vm_1, <_>::default(), &script, "", "");
    let vm_2_result = checked_call_vm!(vm_2, <_>::default(), &script, "", "");
    let vm_3_result_1 = checked_call_vm!(vm_3, <_>::default(), &script, "", vm_2_result.data);
    let vm_4_result = checked_call_vm!(vm_4, <_>::default(), &script, "", vm_3_result_1.data.clone());
    let vm_3_result_2 = checked_call_vm!(vm_3, <_>::default(), &script, vm_3_result_1.data, vm_4_result.data);
    let actual_vm_3_result_2_trace = trace_from_result(&vm_3_result_2);

    let val_2 = stream!(2, 0, peer = peer_id_2, args = [2]);
    let val_3 = stream!(3, 1, peer = peer_id_3, args = [3]);
    let cid_2 = extract_service_result_cid(&val_2);
    let cid_3 = extract_service_result_cid(&val_3);

    let expected_vm_3_result_2_trace = vec![
        executed_state::par(1, 3),
        executed_state::request_sent_by(peer_id_2),
        executed_state::par(1, 1),
        val_2,
        val_3,
        unused!(4, peer = peer_id_4, args = [4]),
        executed_state::canon(json!({
        "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "peer_id_3", "service_id": ""},
        "values": [{
            "result": 2,
            "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "peer_id_2", "service_id": ""},
            "provenance": Provenance::service_result(cid_2.clone()),
        }, {
            "result": 3,
            "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "peer_id_3", "service_id": ""},
            "provenance": Provenance::service_result(cid_3.clone()),
        }]})),
        executed_state::par(1, 1),
        unused!(json!([2, 3]), peer = peer_id_3, args = [json!([2, 3])]),
        executed_state::request_sent_by(peer_id_3),
    ];
    assert_eq!(actual_vm_3_result_2_trace, expected_vm_3_result_2_trace);

    let vm_1_result_2 = checked_call_vm!(vm_1, <_>::default(), script, vm_1_result_1.data, vm_3_result_2.data);
    let vm_1_result_2_trace = trace_from_result(&vm_1_result_2);
    let expected_vm_1_result_2_trace = vec![
        executed_state::par(1, 3),
        stream!(1, 0, peer = peer_id_1, args = [1]),
        executed_state::par(1, 1),
        stream!(2, 1, peer = peer_id_2, args = [2]),
        stream!(3, 2, peer = peer_id_3, args = [3]),
        unused!(4, peer = peer_id_4, args = [4]),
        executed_state::canon(json!({
            "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "peer_id_3", "service_id": ""},
            "values": [{
                "result": 2,
                "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "peer_id_2", "service_id": ""},
                "provenance": Provenance::service_result(cid_2),
            }, {
                "result": 3,
                "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "peer_id_3", "service_id": ""},
                "provenance": Provenance::service_result(cid_3),
            }]
        })),
        executed_state::par(1, 1),
        unused!(json!([2, 3]), peer = peer_id_3, args = [json!([2, 3])]),
        unused!(json!([2, 3]), peer = peer_id_1, args = [json!([2, 3])]),
    ];
    assert_eq!(vm_1_result_2_trace.deref(), expected_vm_1_result_2_trace);
}

#[test]
fn canon_stream_can_be_created_from_aps() {
    let vm_1_peer_id = "vm_1_peer_id";
    let mut vm_1 = create_avm(echo_call_service(), vm_1_peer_id);

    let vm_2_peer_id = "vm_2_peer_id";
    let mut vm_2 = create_avm(echo_call_service(), vm_2_peer_id);

    let script = format!(
        r#"
        (seq
            (seq
                (ap 0 $stream)
                (ap 1 $stream))
            (seq
                (canon "{vm_1_peer_id}" $stream #canon_stream)
                (seq
                    (ap #canon_stream $stream_2)
                    (seq
                        (canon "{vm_2_peer_id}" $stream_2 #canon_stream_2)
                        (call "{vm_2_peer_id}" ("" "") [#canon_stream_2])))))
        "#
    );

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

    let script = format!(
        r#"
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
            "#
    );

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

    let script = format!(
        r#"
            (new $stream
                (seq
                    (canon "{peer_id_1}" $stream #canon_stream)
                    (call "{peer_id_1}" ("" "") [#canon_stream])))
                    "#
    );

    let result = checked_call_vm!(vm_1, <_>::default(), &script, "", "");
    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![
        executed_state::canon(
            json!({"tetraplet": {"function_name": "", "json_path": "", "peer_pk": "peer_id_1", "service_id": ""}, "values": []}),
        ),
        unused!(json!([]), peer = peer_id_1, args = [json!([])]),
    ];
    assert_eq!(actual_trace, expected_trace);

    let result = checked_call_vm!(vm_2, <_>::default(), script, "", result.data);
    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![
        executed_state::canon(
            json!({"tetraplet": {"function_name": "", "json_path": "", "peer_pk": "peer_id_1", "service_id": ""}, "values": []} ),
        ),
        unused!(json!([]), peer = peer_id_1, args = [json!([])]),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn canon_empty_not_writable_stream() {
    let peer_id = "peer_id";
    let mut vm = create_avm(echo_call_service(), peer_id);

    let script = format!(
        r#"
        (par
            (call "unwkown_peer_id" ("" "") [] $stream)
            (canon "{peer_id}" $stream #canon_stream)
        )
    "#
    );

    let result = checked_call_vm!(vm, <_>::default(), &script, "", "");
    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![
        executed_state::par(1, 1),
        executed_state::request_sent_by(peer_id),
        executed_state::canon(
            json!({"tetraplet": {"function_name": "", "json_path": "", "peer_pk": "peer_id", "service_id": ""}, "values": []} ),
        ),
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

    let script = format!(
        r#"
        (par
            (call "{vm_peer_id_2}" ("" "") [1] $stream)
            (seq
                (canon "{vm_peer_id_1}" $stream #canon_stream) ; it returns a catchable error
                (call "{vm_peer_id_3}" ("" "") [#canon_stream])
            )
        )
    "#
    );

    let result = checked_call_vm!(peer_vm_1, <_>::default(), &script, "", "");
    let result = checked_call_vm!(peer_vm_2, <_>::default(), &script, "", result.data);
    let result = checked_call_vm!(peer_vm_3, <_>::default(), &script, "", result.data);
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        executed_state::par(1, 2),
        stream!(1, 0, peer = vm_peer_id_2, args = [1]),
        executed_state::canon(
            json!({"tetraplet": {"function_name": "", "json_path": "", "peer_pk": "vm_peer_id_1", "service_id": ""},"values": []}),
        ),
        unused!(json!([]), peer = vm_peer_id_3, args = [json!([])]),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn canon_map_scalar() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let mut peer_vm_1 = create_avm(echo_call_service(), vm_peer_id_1);

    let script = format!(
        r#"
        (seq
            (seq
                (seq
                    (seq
                        (ap ("k" "v1") %map)
                        (ap ("k" "v2") %map)
                    )
                    (seq
                        (ap (42 "v3") %map)
                        (ap (42 "v4") %map)
                    )
                )
                (seq
                    (ap (-42 "v5") %map)
                    (ap (-42 "v6") %map)
                )
            )
            (seq
                (canon "{vm_peer_id_1}" %map scalar)
                (call "{vm_peer_id_1}" ("m1" "f1") [scalar] output)
            )
        )
    "#
    );

    let result = checked_call_vm!(peer_vm_1, <_>::default(), &script, "", "");

    let actual_trace = trace_from_result(&result);

    let mut cid_state: ExecutionCidState = ExecutionCidState::new();
    let value1 = json!({"k": "v1", "42": "v3", "-42": "v5"});
    let tetraplet = json!({"function_name": "", "json_path": "", "peer_pk": "vm_peer_id_1", "service_id": ""});

    let expected_trace = ExecutionTrace::from(vec![
        executed_state::ap(0),
        executed_state::ap(0),
        executed_state::ap(0),
        executed_state::ap(0),
        executed_state::ap(0),
        executed_state::ap(0),
        canon_tracked(
            json!({"tetraplet": {"function_name": "", "json_path": "", "peer_pk": "vm_peer_id_1", "service_id": ""},
            "values": [
                {
                    "result": value1,
                    "tetraplet": tetraplet,
                    "provenance": Provenance::Literal,
                },
            ]}),
            &mut cid_state,
        ),
        scalar_tracked!(
            value1.clone(),
            cid_state,
            peer = vm_peer_id_1,
            service = "m1",
            function = "f1",
            args = vec![value1]
        ),
    ]);
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn canon_map_scalar_with_par() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let vm_peer_id_2 = "vm_peer_id_2";

    let mut peer_vm_1 = create_avm(echo_call_service(), vm_peer_id_1);
    let mut peer_vm_2 = create_avm(echo_call_service(), vm_peer_id_2);

    let script = format!(
        r#"
        (par
            (seq
                (seq
                    (ap ("k" "v1") %map)
                    (ap (-42 "v2") %map)
                )
                (seq
                    (canon "{vm_peer_id_1}" %map scalar)
                    (call "{vm_peer_id_1}" ("m1" "f1") [scalar] output)
                )
            )
            (seq
                (seq
                    (ap (42 "v3") %map)
                    (ap ("42" "v4") %map)
                )
                (seq
                    (canon "{vm_peer_id_2}" %map scalar1)
                    (call "{vm_peer_id_2}" ("m2" "f2") [scalar1] output1)
                )
            )
        )
    "#
    );

    let result = checked_call_vm!(peer_vm_1, <_>::default(), &script, "", "");
    let actual_trace = trace_from_result(&result);

    let mut cid_state: ExecutionCidState = ExecutionCidState::new();
    let value_1 = json!({"k": "v1", "-42": "v2"});
    let tetraplet = json!({"function_name": "", "json_path": "", "peer_pk": "vm_peer_id_1", "service_id": ""});
    let mut states_vec = vec![
        executed_state::par(4, 3),
        executed_state::ap(0),
        executed_state::ap(0),
        canon_tracked(
            json!({"tetraplet": tetraplet,
            "values": [
                {
                "result": value_1,
                "tetraplet": tetraplet,
                "provenance": Provenance::Literal,
                },
            ]}),
            &mut cid_state,
        ),
        scalar_tracked!(
            value_1.clone(),
            cid_state,
            peer = vm_peer_id_1,
            service = "m1",
            function = "f1",
            args = vec![value_1.clone()]
        ),
        executed_state::ap(0),
        executed_state::ap(0),
        canon_request(vm_peer_id_1),
    ];

    let expected_trace = ExecutionTrace::from(states_vec.clone());
    assert_eq!(actual_trace, expected_trace);

    let result = checked_call_vm!(peer_vm_2, <_>::default(), &script, "", result.data);
    let actual_trace = trace_from_result(&result);

    let value_2 = json!({"-42": "v2", "42": "v3", "k": "v1"});

    states_vec[0] = executed_state::par(4, 4);
    // remove last state to be replaced
    let can_req = states_vec.pop();
    assert_eq!(can_req, Some(canon_request(vm_peer_id_1)), "test invalid");
    let tetraplet = json!({"function_name": "", "json_path": "", "peer_pk": "vm_peer_id_2", "service_id": ""});

    states_vec.extend(vec![
        canon_tracked(
            json!({"tetraplet": tetraplet,
                    "values": [
                        {
                        "result": value_2,
                        "tetraplet": tetraplet,
                        "provenance": Provenance::Literal,
                        },
            ]}),
            &mut cid_state,
        ),
        scalar_tracked!(
            value_2.clone(),
            cid_state,
            peer = vm_peer_id_2,
            service = "m2",
            function = "f2",
            args = vec![value_2.clone()]
        ),
    ]);
    let expected_trace = ExecutionTrace::from(states_vec.clone());

    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn test_extend_by_request_sent_by() {
    let peer_id_1 = "peer_1";
    let peer_id_2 = "peer_2";
    let other_peer_id = "A";

    let mut peer_vm_1 = create_avm(echo_call_service(), peer_id_1);
    let mut peer_vm_2 = create_avm(echo_call_service(), peer_id_2);

    let script = format!(
        r#"
        (seq
           (par
              (call "{peer_id_1}" ("" "") [1] $stream)
              (call "{peer_id_2}" ("" "") [1] $stream))
           (canon "{other_peer_id}" $stream #canon))
        "#
    );

    let result_1_1 = checked_call_vm!(peer_vm_1, <_>::default(), &script, "", "");
    let result_2_1 = checked_call_vm!(peer_vm_2, <_>::default(), &script, "", result_1_1.data);

    let trace_2_1 = trace_from_result(&result_2_1);
    assert_eq!(
        &*trace_2_1,
        vec![
            par(1, 1),
            stream!(1, 0, peer = peer_id_1, args = [1]),
            stream!(1, 1, peer = peer_id_2, args = [1]),
            canon_request(peer_id_1),
        ],
    )
}

#[test]
fn test_merge_request_sent_by() {
    let peer_id_1 = "peer_1";
    let peer_id_2 = "peer_2";
    let other_peer_id = "A";

    let mut peer_vm_1 = create_avm(echo_call_service(), peer_id_1);
    let mut peer_vm_2 = create_avm(echo_call_service(), peer_id_2);

    let script = format!(
        r#"
        (seq
           (par
              (call "{peer_id_1}" ("" "") [1] $stream)
              (call "{peer_id_2}" ("" "") [1] $stream))
           (canon "{other_peer_id}" $stream #canon))
        "#
    );

    let result_1_1 = checked_call_vm!(peer_vm_1, <_>::default(), &script, "", "");
    let result_2_1 = checked_call_vm!(peer_vm_2, <_>::default(), &script, "", "");
    let result_1_2 = checked_call_vm!(peer_vm_1, <_>::default(), &script, result_1_1.data, result_2_1.data);

    let trace_1_2 = trace_from_result(&result_1_2);
    assert_eq!(
        &*trace_1_2,
        vec![
            par(1, 1),
            stream!(1, 0, peer = peer_id_1, args = [1]),
            stream!(1, 1, peer = peer_id_2, args = [1]),
            canon_request(peer_id_1),
        ],
    )
}

#[test]
fn test_merge_executed() {
    let peer_id_1 = "peer_1";
    let peer_id_2 = "peer_2";
    let other_peer_id = "A";

    let mut peer_vm_1 = create_avm(echo_call_service(), peer_id_1);
    let mut peer_vm_2 = create_avm(echo_call_service(), peer_id_2);
    let mut peer_other_id = create_avm(echo_call_service(), other_peer_id);

    let script = format!(
        r#"
        (seq
           (par
              (call "{peer_id_1}" ("" "") [1] $stream)
              (call "{peer_id_2}" ("" "") [1] $stream))
           (canon "{other_peer_id}" $stream #canon))
        "#
    );

    let result_1_1 = checked_call_vm!(peer_vm_1, <_>::default(), &script, "", "");
    let result_other_1 = checked_call_vm!(peer_other_id, <_>::default(), &script, "", result_1_1.data.clone());
    let result_2_1 = checked_call_vm!(peer_vm_2, <_>::default(), &script, "", result_1_1.data);
    let result_2_3 = checked_call_vm!(
        peer_other_id,
        <_>::default(),
        &script,
        result_other_1.data,
        result_2_1.data
    );

    let trace_2_3 = trace_from_result(&result_2_3);
    let s1 = stream!(1, 0, peer = peer_id_1, args = [1]);
    let cid1 = extract_service_result_cid(&s1);

    assert_eq!(
        &*trace_2_3,
        vec![
            par(1, 1),
            s1,
            stream!(1, 1, peer = peer_id_2, args = [1]),
            executed_state::canon(
                json!({"tetraplet": {"function_name": "", "json_path": "", "peer_pk": other_peer_id, "service_id": ""},
                "values": [{
                    "result": 1,
                    "tetraplet": {"function_name": "", "json_path": "", "peer_pk": peer_id_1, "service_id": ""},
                    "provenance": Provenance::service_result(cid1),
                }]}),
            ),
        ],
        "{:#?}",
        data_from_result(&result_2_3),
    );
}

#[test]
fn canon_stream_map() {
    let vm_peer_id_1_name = "vm_peer_id_1";
    let vm_peer_id_1_id = at(vm_peer_id_1_name);

    let script = format!(
        r#"
    (seq
        (ap (42 "value2") %map)
        (seq
            (ap ("key" "value1") %map)
            (canon "{vm_peer_id_1_name}" %map #%canon_map)
        )
    )
        "#
    );

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(vm_peer_id_1_name), &script)
        .expect("invalid test AIR script");
    let result = executor.execute_all(vm_peer_id_1_name).unwrap();

    let actual_data = data_from_result(&result.last().unwrap());

    let mut cid_state: ExecutionCidState = ExecutionCidState::new();
    let map_value1 = json!({"key": 42, "value": "value2"});
    let map_value2 = json!({"key": "key", "value": "value1"});
    let tetraplet = json!({"function_name": "", "json_path": "", "peer_pk": vm_peer_id_1_id, "service_id": ""});

    let states_vec = vec![
        executed_state::ap(0),
        executed_state::ap(0),
        canon_tracked(
            json!({"tetraplet": tetraplet,
            "values": [
                {
                "result": map_value1,
                "tetraplet": tetraplet,
                "provenance": Provenance::Literal,
            },
            {
                "result": map_value2,
                "tetraplet": tetraplet,
                "provenance": Provenance::Literal,
                },]}),
            &mut cid_state,
        ),
    ];

    let expected_trace = ExecutionTrace::from(states_vec.clone());

    assert_eq!(actual_data.trace, expected_trace, "{:#?}", actual_data.cid_info,);
}

#[test]
fn canon_map_non_existing_index_tetraplet_check() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let arg_tetraplets = Rc::new(RefCell::new(vec![]));

    let echo_call_service: CallServiceClosure = Box::new(move |mut params| -> CallServiceResult {
        let arg_tetraplets_inner = arg_tetraplets.clone();
        arg_tetraplets_inner.borrow_mut().push(params.tetraplets.clone());
        CallServiceResult::ok(params.arguments.remove(0))
    });

    let (echo_call_service, tetraplet_checker) = tetraplet_host_function(echo_call_service);
    let mut vm_1 = create_avm(echo_call_service, vm_peer_id_1);

    let script = format!(
        r#"
        (seq
            (ap (42 "value2") %map)
            (seq
                (canon "{vm_peer_id_1}" %map #%canon_map)
                (call "{vm_peer_id_1}" ("" "") [#%canon_map.$.key] output)
            )
        )
    "#
    );

    let result = checked_call_vm!(vm_1, <_>::default(), &script, "", "");

    let actual_trace = trace_from_result(&result);

    let mut cid_state: ExecutionCidState = ExecutionCidState::new();
    let map_value1 = json!({"key": 42, "value": "value2"});
    let call_result = json!([]);

    let tetraplet = json!({"function_name": "", "json_path": "", "peer_pk": "vm_peer_id_1", "service_id": ""});
    let empty_tetraplet = json!({"function_name": "", "json_path": "", "peer_pk": "", "service_id": ""});

    let states_vec = vec![
        executed_state::ap(0),
        canon_tracked(
            json!({"tetraplet": tetraplet,
            "values": [
                {
                "result": map_value1,
                "tetraplet": empty_tetraplet,
                "provenance": Provenance::Literal,
            },
            ]}),
            &mut cid_state,
        ),
        scalar_tracked!(
            call_result.clone(),
            cid_state,
            peer = vm_peer_id_1,
            service = "",
            function = "",
            args = vec![call_result]
        ),
    ];

    let expected_trace = ExecutionTrace::from(states_vec.clone());
    let expected_tetraplet = RefCell::new(vec![vec![SecurityTetraplet::new(vm_peer_id_1, "", "", ".$.key")]]);

    assert_eq!(actual_trace, expected_trace,);

    assert_eq!(tetraplet_checker.as_ref(), &expected_tetraplet);
}

#[test]
fn canon_map_non_existing_2_indices_tetraplet_check() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let arg_tetraplets = Rc::new(RefCell::new(vec![]));

    let echo_call_service: CallServiceClosure = Box::new(move |mut params| -> CallServiceResult {
        let arg_tetraplets_inner = arg_tetraplets.clone();
        arg_tetraplets_inner.borrow_mut().push(params.tetraplets.clone());
        CallServiceResult::ok(params.arguments.remove(0))
    });

    let (echo_call_service, tetraplet_checker) = tetraplet_host_function(echo_call_service);
    let mut vm_1 = create_avm(echo_call_service, vm_peer_id_1);

    let script = format!(
        r#"
        (seq
            (seq
                (ap (42 "value2") %map)
                (ap (42 "value1") %map)
            )
            (seq
                (canon "{vm_peer_id_1}" %map #%canon_map)
                (call "{vm_peer_id_1}" ("" "") [#%canon_map.$.[42].[0]] output)
            )
        )
    "#
    );

    let result = checked_call_vm!(vm_1, <_>::default(), &script, "", "");

    let actual_trace = trace_from_result(&result);

    let mut cid_state: ExecutionCidState = ExecutionCidState::new();
    let map_value1 = json!({"key": 42, "value": "value2"});
    let call_result = json!("value2");

    let tetraplet = json!({"function_name": "", "json_path": "", "peer_pk": "vm_peer_id_1", "service_id": ""});
    let empty_tetraplet = json!({"function_name": "", "json_path": "", "peer_pk": "", "service_id": ""});

    let states_vec = vec![
        executed_state::ap(0),
        executed_state::ap(0),
        canon_tracked(
            json!({"tetraplet": tetraplet,
            "values": [
                {
                "result": map_value1,
                "tetraplet": empty_tetraplet,
                "provenance": Provenance::Literal,
            },
            ]}),
            &mut cid_state,
        ),
        scalar_tracked!(
            call_result.clone(),
            cid_state,
            peer = vm_peer_id_1,
            service = "",
            function = "",
            args = vec![call_result]
        ),
    ];

    let expected_trace = ExecutionTrace::from(states_vec.clone());
    let expected_tetraplet = RefCell::new(vec![vec![SecurityTetraplet::new(vm_peer_id_1, "", "", ".$.[42].[0]")]]);

    assert_eq!(actual_trace, expected_trace,);

    assert_eq!(tetraplet_checker.as_ref(), &expected_tetraplet);
}

#[test]
fn canon_map_2_scalar_tetraplet_check() {
    let vm_peer_id_1 = "vm_peer_id_1";

    let arg_tetraplets = Rc::new(RefCell::new(vec![]));

    let call_service: CallServiceClosure = Box::new(move |mut params| -> CallServiceResult {
        let arg_tetraplets_inner = arg_tetraplets.clone();
        arg_tetraplets_inner.borrow_mut().push(params.tetraplets.clone());
        CallServiceResult::ok(params.arguments.remove(0))
    });

    let (call_service, tetraplet_checker) = tetraplet_host_function(call_service);
    let mut vm_1 = create_avm(call_service, vm_peer_id_1);

    let script = format!(
        r#"
        (seq
            (seq
                (seq
                    (ap (42 "value1") %map)
                    (ap ("key" "value1") %map)
                )
                (ap (42 "value2") %map)
            )
            (seq
                (canon "{vm_peer_id_1}" %map scalar)
                (call "{vm_peer_id_1}" ("" "") [scalar] output)
            )
        )
    "#
    );

    let result = checked_call_vm!(vm_1, <_>::default(), &script, "", "");

    let actual_data = data_from_result(&result);

    let mut cid_state: ExecutionCidState = ExecutionCidState::new();
    let map_value1 = json!({"42": "value1", "key": "value1"});
    let call_result = map_value1.clone();

    let tetraplet = json!({"function_name": "", "json_path": "", "peer_pk": vm_peer_id_1, "service_id": ""});

    let states_vec = vec![
        executed_state::ap(0),
        executed_state::ap(0),
        executed_state::ap(0),
        canon_tracked(
            json!({"tetraplet": tetraplet,
            "values": [
                {
                "result": map_value1,
                "tetraplet": tetraplet,
                "provenance": Provenance::Literal,
            },
            ]}),
            &mut cid_state,
        ),
        scalar_tracked!(
            call_result.clone(),
            cid_state,
            peer = vm_peer_id_1,
            service = "",
            function = "",
            args = vec![call_result]
        ),
    ];

    let expected_trace = ExecutionTrace::from(states_vec.clone());
    let expected_tetraplet = RefCell::new(vec![vec![SecurityTetraplet::new(vm_peer_id_1, "", "", "")]]);

    assert_eq!(tetraplet_checker.as_ref(), &expected_tetraplet);

    assert_eq!(actual_data.trace, expected_trace, "{:#?}", actual_data.cid_info,);
}

#[test]
fn canon_map_2_scalar_with_lens_tetraplet_check() {
    let vm_peer_id_1 = "vm_peer_id_1";

    let arg_tetraplets = Rc::new(RefCell::new(vec![]));

    let call_service: CallServiceClosure = Box::new(move |mut params| -> CallServiceResult {
        let arg_tetraplets_inner = arg_tetraplets.clone();
        arg_tetraplets_inner.borrow_mut().push(params.tetraplets.clone());
        CallServiceResult::ok(params.arguments.remove(0))
    });

    let (call_service, tetraplet_checker) = tetraplet_host_function(call_service);
    let mut vm_1 = create_avm(call_service, vm_peer_id_1);

    let script = format!(
        r#"
        (seq
            (seq
                (seq
                    (ap (42 "value1") %map)
                    (ap ("key" "value1") %map)
                )
                (ap (42 "value2") %map)
            )
            (seq
                (canon "{vm_peer_id_1}" %map scalar)
                (call "{vm_peer_id_1}" ("" "") [scalar.$.key] output)
            )
        )
    "#
    );

    let result = checked_call_vm!(vm_1, <_>::default(), &script, "", "");
    let actual_data = data_from_result(&result);

    let mut cid_state: ExecutionCidState = ExecutionCidState::new();
    let map_value1 = json!({"42": "value1", "key": "value1"});
    let call_result = json!("value1");

    let tetraplet = json!({"function_name": "", "json_path": "", "peer_pk": vm_peer_id_1, "service_id": ""});

    let states_vec = vec![
        executed_state::ap(0),
        executed_state::ap(0),
        executed_state::ap(0),
        canon_tracked(
            json!({"tetraplet": tetraplet,
            "values": [
                {
                "result": map_value1,
                "tetraplet": tetraplet,
                "provenance": Provenance::Literal,
            },
            ]}),
            &mut cid_state,
        ),
        scalar_tracked!(
            call_result.clone(),
            cid_state,
            peer = vm_peer_id_1,
            service = "",
            function = "",
            args = vec![call_result]
        ),
    ];

    let expected_trace = ExecutionTrace::from(states_vec.clone());
    let expected_tetraplet = RefCell::new(vec![vec![SecurityTetraplet::new(vm_peer_id_1, "", "", ".$.key")]]);

    assert_eq!(tetraplet_checker.as_ref(), &expected_tetraplet);

    assert_eq!(actual_data.trace, expected_trace, "{:#?}", actual_data.cid_info,);
}

#[test]
fn canon_join_behavoir() {
    let init_peer_name = "init_peer_id";

    let script = r#"
    (seq
       (par
          (null)
          (seq
             (never)
             (ap %init_peer_id% var)))
       (seq
          (ap 42 $stream)
          (canon var $stream #canon)))
    "#;

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(init_peer_name), &script)
        .expect("invalid test AIR script");
    let result = executor.execute_one(init_peer_name).unwrap();

    assert_eq!(result.ret_code, 0, "{:?}", result.error_message);
}

#[test]
fn canon_map_join_behavoir() {
    let init_peer_name = "init_peer_id";

    let script = r#"
    (seq
       (par
          (null)
          (seq
             (never)
             (ap %init_peer_id% var)))
       (seq
          (ap ("answer" 42) %map)
          (canon var %map #%canon)))
    "#;

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(init_peer_name), &script)
        .expect("invalid test AIR script");
    let result = executor.execute_one(init_peer_name).unwrap();

    assert_eq!(result.ret_code, 0, "{:?}", result.error_message);
}

#[test]
fn canon_map_var_join_behavoir() {
    let init_peer_name = "init_peer_id";

    let script = r#"
    (seq
       (par
          (null)
          (seq
             (never)
             (ap %init_peer_id% var)))
       (seq
          (ap ("answer" 42) %map)
          (canon var %map value)))
    "#;

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(init_peer_name), &script)
        .expect("invalid test AIR script");
    let result = executor.execute_one(init_peer_name).unwrap();

    assert_eq!(result.ret_code, 0, "{:?}", result.error_message);
}
