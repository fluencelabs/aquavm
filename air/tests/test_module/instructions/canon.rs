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
use maplit::hashmap;

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

    let expected_state = executed_state::canon(vec![1.into(), 2.into(), 3.into(), 4.into(), 5.into()]);
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
        executed_state::canon(vec![3.into(), 4.into()]),
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
        executed_state::stream_number(3, 1),
        executed_state::scalar_number(4),
        executed_state::canon(vec![3.into(), 4.into()]),
        executed_state::par(1, 1),
        executed_state::scalar(json!([2, 3])),
        executed_state::scalar(json!([2, 3])),
    ];
    assert_eq!(vm_1_result_2_trace, expected_vm_1_result_2_trace);
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
    let expected_trace = vec![executed_state::canon(vec![]), executed_state::scalar(json!([]))];
    assert_eq!(actual_trace, expected_trace);

    let result = checked_call_vm!(vm_2, <_>::default(), script, "", result.data);
    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![executed_state::canon(vec![]), executed_state::scalar(json!([]))];
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn issue_smth() {
    env_logger::init();
    let mut vm = create_avm(echo_call_service(), "12D3KooWQ6S6pzRCupyqoK5G4mfPojs4ob47u4aconRJUf61eChG");
    // {"trace":[{"call":{"executed":{"scalar":"12D3KooWHCJbJKGDfCgHSoCuK9q4STyRnVveqLoXAPBbXHTZx9Cv"}}},{"call":{"sent_by":"12D3KooWQ6S6pzRCupyqoK5G4mfPojs4ob47u4aconRJUf61eChG: 2"}}],"streams":{},"version":"0.2.2","lcid":2,"r_streams":{}}
    let prev_data: Vec<u8> = vec![123,34,116,114,97,99,101,34,58,91,123,34,99,97,108,108,34,58,123,34,101,120,101,99,117,116,101,100,34,58,123,34,115,99,97,108,97,114,34,58,34,49,50,68,51,75,111,111,87,72,67,74,98,74,75,71,68,102,67,103,72,83,111,67,117,75,57,113,52,83,84,121,82,110,86,118,101,113,76,111,88,65,80,66,98,88,72,84,90,120,57,67,118,34,125,125,125,44,123,34,99,97,108,108,34,58,123,34,115,101,110,116,95,98,121,34,58,34,49,50,68,51,75,111,111,87,81,54,83,54,112,122,82,67,117,112,121,113,111,75,53,71,52,109,102,80,111,106,115,52,111,98,52,55,117,52,97,99,111,110,82,74,85,102,54,49,101,67,104,71,58,32,50,34,125,125,93,44,34,115,116,114,101,97,109,115,34,58,123,125,44,34,118,101,114,115,105,111,110,34,58,34,48,46,50,46,50,34,44,34,108,99,105,100,34,58,50,44,34,114,95,115,116,114,101,97,109,115,34,58,123,125,125];
    let script = r#"
                    (seq
                     (seq
                      (seq
                       (call %init_peer_id% ("getDataSrv" "-relay-") [] -relay-)
                       (call %init_peer_id% ("getDataSrv" "strNone") [] strNone)
                      )
                      (new $str
                       (seq
                        (seq
                         (new $option-inline
                          (seq
                           (xor
                            (ap strNone.$.[0]! $option-inline)
                            (null)
                           )
                           (canon %init_peer_id% $option-inline  #option-inline-0)
                          )
                         )
                         (fold #option-inline-0 i-0
                          (seq
                           (ap i-0 $str)
                           (next i-0)
                          )
                         )
                        )
                        (canon %init_peer_id% $str  #str-fix)
                       )
                      )
                     )
                     (call %init_peer_id% ("callbackSrv" "response") [[] #str-fix []])
                    )"#;

    //{"2":{"ret_code":0,"result":"[]"}}
    let call_results = hashmap! {
        2u32 => CallServiceResult {
            ret_code: 0,
            result: JValue::Array(vec![]),
        }
    };
    vm.runner.call(script.clone(), prev_data.clone(), "", "12D3KooWQ6S6pzRCupyqoK5G4mfPojs4ob47u4aconRJUf61eChG", 0, 700, call_results).unwrap();
}
