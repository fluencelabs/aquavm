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

use std::cell::RefCell;

#[test]
fn ap_with_scalars() {
    let vm_1_peer_id = "vm_1_peer_id";
    let test_value = "scalar_2";
    let mut vm_1 = create_avm(set_variable_call_service(json!({ "field": test_value })), vm_1_peer_id);

    let vm_2_peer_id = "vm_2_peer_id";
    let mut vm_2 = create_avm(echo_call_service(), vm_2_peer_id);

    let script = f!(r#"
        (seq
            (seq
                (call "{vm_1_peer_id}" ("" "") ["scalar_1_result"] scalar_1)
                (ap scalar_1.$.field! scalar_2)
            )
            (call "{vm_2_peer_id}" ("" "") [scalar_2])
        )
        "#);

    let result = checked_call_vm!(vm_1, <_>::default(), &script, "", "");
    let result = checked_call_vm!(vm_2, <_>::default(), script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_state = vec![
        executed_state::scalar(json!({ "field": test_value })),
        executed_state::scalar_string(test_value),
    ];

    assert_eq!(actual_trace, expected_state);
    assert!(result.next_peer_pks.is_empty());
}

#[test]
fn ap_with_string_literal() {
    let vm_1_peer_id = "vm_1_peer_id";
    let mut vm_1 = create_avm(echo_call_service(), vm_1_peer_id);

    let some_string = "some_string";
    let script = f!(r#"
        (seq
            (ap "{some_string}" $stream)
            (call "{vm_1_peer_id}" ("" "") [$stream])
        )
        "#);

    let result = checked_call_vm!(vm_1, <_>::default(), script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_state = vec![executed_state::ap(0), executed_state::scalar(json!([some_string]))];

    assert_eq!(actual_trace, expected_state);
    assert!(result.next_peer_pks.is_empty());
}

#[test]
fn ap_with_bool_literal() {
    let vm_1_peer_id = "vm_1_peer_id";
    let mut vm_1 = create_avm(echo_call_service(), vm_1_peer_id);

    let script = f!(r#"
        (seq
            (ap true $stream)
            (call "{vm_1_peer_id}" ("" "") [$stream])
        )
        "#);

    let result = checked_call_vm!(vm_1, <_>::default(), script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_state = vec![executed_state::ap(0), executed_state::scalar(json!([true]))];

    assert_eq!(actual_trace, expected_state);
    assert!(result.next_peer_pks.is_empty());
}

#[test]
fn ap_with_number_literal() {
    let vm_1_peer_id = "vm_1_peer_id";
    let mut vm_1 = create_avm(echo_call_service(), vm_1_peer_id);

    let script = f!(r#"
        (seq
            (ap 100 $stream)
            (call "{vm_1_peer_id}" ("" "") [$stream])
        )
        "#);

    let result = checked_call_vm!(vm_1, <_>::default(), script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_state = vec![executed_state::ap(0), executed_state::scalar(json!([100]))];

    assert_eq!(actual_trace, expected_state);
    assert!(result.next_peer_pks.is_empty());
}

#[test]
fn ap_with_last_error() {
    let vm_1_peer_id = "vm_1_peer_id";
    let mut vm_1 = create_avm(echo_call_service(), vm_1_peer_id);

    let script = f!(r#"
        (seq
            (ap %last_error% $stream)
            (call "{vm_1_peer_id}" ("" "") [$stream])
        )
        "#);

    let result = checked_call_vm!(vm_1, <_>::default(), script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_state = vec![executed_state::ap(0), executed_state::scalar(json!([null]))];

    assert_eq!(actual_trace, expected_state);
    assert!(result.next_peer_pks.is_empty());
}

#[test]
fn ap_with_timestamp() {
    let vm_1_peer_id = "vm_1_peer_id";
    let mut vm_1 = create_avm(echo_call_service(), vm_1_peer_id);

    let script = f!(r#"
        (seq
            (ap %timestamp% scalar)
            (call "{vm_1_peer_id}" ("" "") [scalar])
        )
        "#);

    let test_params = TestRunParameters::from_timestamp(1337);
    let result = checked_call_vm!(vm_1, test_params.clone(), script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_state = vec![executed_state::scalar_number(test_params.timestamp)];

    assert_eq!(actual_trace, expected_state);
}

#[test]
fn ap_with_ttl() {
    let vm_1_peer_id = "vm_1_peer_id";
    let mut vm_1 = create_avm(echo_call_service(), vm_1_peer_id);

    let script = f!(r#"
        (seq
            (ap %ttl% scalar)
            (call "{vm_1_peer_id}" ("" "") [scalar])
        )
        "#);

    let test_params = TestRunParameters::from_ttl(1337);
    let result = checked_call_vm!(vm_1, test_params.clone(), script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_state = vec![executed_state::scalar_number(test_params.ttl)];

    assert_eq!(actual_trace, expected_state);
}

#[test]
fn ap_with_dst_stream() {
    let vm_1_peer_id = "vm_1_peer_id";
    let test_value = "scalar_2";
    let mut vm_1 = create_avm(set_variable_call_service(json!({ "field": test_value })), vm_1_peer_id);

    let vm_2_peer_id = "vm_2_peer_id";
    let mut vm_2 = create_avm(echo_call_service(), vm_2_peer_id);

    let script = f!(r#"
        (seq
            (seq
                (call "{vm_1_peer_id}" ("" "") ["scalar_1_result"] scalar_1)
                (ap scalar_1 $stream)
            )
            (call "{vm_2_peer_id}" ("" "") [$stream])
        )
        "#);

    let result = checked_call_vm!(vm_1, <_>::default(), &script, "", "");
    let result = checked_call_vm!(vm_2, <_>::default(), script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_state = vec![
        executed_state::scalar(json!({ "field": test_value })),
        executed_state::ap(0),
        executed_state::scalar(json!([{ "field": test_value }])),
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
    let script = f!(r#"
        (seq
            (seq
                (call "{vm_1_peer_id}" ("" "") [0] $stream)
                (call "{vm_1_peer_id}" ("{service_name}" "{function_name}") [1] $stream))
            (seq
                (canon "{vm_1_peer_id}" $stream #canon_stream)
                (seq
                    (ap #canon_stream.$.[1] $stream_2)
                    (call "{vm_1_peer_id}" ("" "") [$stream_2]))))
        "#);

    let result = checked_call_vm!(vm_1, <_>::default(), &script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_state = vec![
        executed_state::stream_number(0, 0),
        executed_state::stream_number(1, 1),
        executed_state::canon(vec![
            123, 34, 118, 97, 108, 117, 101, 115, 34, 58, 91, 123, 34, 114, 101, 115, 117, 108, 116, 34, 58, 48, 44,
            34, 116, 101, 116, 114, 97, 112, 108, 101, 116, 34, 58, 123, 34, 112, 101, 101, 114, 95, 112, 107, 34, 58,
            34, 118, 109, 95, 49, 95, 112, 101, 101, 114, 95, 105, 100, 34, 44, 34, 115, 101, 114, 118, 105, 99, 101,
            95, 105, 100, 34, 58, 34, 34, 44, 34, 102, 117, 110, 99, 116, 105, 111, 110, 95, 110, 97, 109, 101, 34, 58,
            34, 34, 44, 34, 106, 115, 111, 110, 95, 112, 97, 116, 104, 34, 58, 34, 34, 125, 44, 34, 116, 114, 97, 99,
            101, 95, 112, 111, 115, 34, 58, 48, 125, 44, 123, 34, 114, 101, 115, 117, 108, 116, 34, 58, 49, 44, 34,
            116, 101, 116, 114, 97, 112, 108, 101, 116, 34, 58, 123, 34, 112, 101, 101, 114, 95, 112, 107, 34, 58, 34,
            118, 109, 95, 49, 95, 112, 101, 101, 114, 95, 105, 100, 34, 44, 34, 115, 101, 114, 118, 105, 99, 101, 95,
            105, 100, 34, 58, 34, 115, 111, 109, 101, 95, 115, 101, 114, 118, 105, 99, 101, 95, 110, 97, 109, 101, 34,
            44, 34, 102, 117, 110, 99, 116, 105, 111, 110, 95, 110, 97, 109, 101, 34, 58, 34, 115, 111, 109, 101, 95,
            102, 117, 110, 99, 116, 105, 111, 110, 95, 110, 97, 109, 101, 34, 44, 34, 106, 115, 111, 110, 95, 112, 97,
            116, 104, 34, 58, 34, 34, 125, 44, 34, 116, 114, 97, 99, 101, 95, 112, 111, 115, 34, 58, 49, 125, 93, 44,
            34, 116, 101, 116, 114, 97, 112, 108, 101, 116, 34, 58, 123, 34, 112, 101, 101, 114, 95, 112, 107, 34, 58,
            34, 118, 109, 95, 49, 95, 112, 101, 101, 114, 95, 105, 100, 34, 44, 34, 115, 101, 114, 118, 105, 99, 101,
            95, 105, 100, 34, 58, 34, 34, 44, 34, 102, 117, 110, 99, 116, 105, 111, 110, 95, 110, 97, 109, 101, 34, 58,
            34, 34, 44, 34, 106, 115, 111, 110, 95, 112, 97, 116, 104, 34, 58, 34, 34, 125, 125,
        ]),
        executed_state::ap(0),
        executed_state::scalar(json!([1])),
    ];
    assert_eq!(actual_trace, expected_state);

    let expected_tetraplet = RefCell::new(vec![vec![SecurityTetraplet::new(
        vm_1_peer_id,
        service_name,
        function_name,
        ".$.[1]",
    )]]);
    assert_eq!(tetraplet_checker.as_ref(), &expected_tetraplet);
}

#[test]
fn ap_canon_stream() {
    let vm_1_peer_id = "vm_1_peer_id";
    let (echo_call_service, tetraplet_checker) = tetraplet_host_function(echo_call_service());
    let mut vm_1 = create_avm(echo_call_service, vm_1_peer_id);

    let service_name = "some_service_name";
    let function_name = "some_function_name";
    let script = f!(r#"
        (seq
            (seq
                (call "{vm_1_peer_id}" ("" "") [0] $stream)
                (call "{vm_1_peer_id}" ("{service_name}" "{function_name}") [1] $stream))
            (seq
                (canon "{vm_1_peer_id}" $stream #canon_stream)
                (seq
                    (ap #canon_stream $stream_2)
                    (call "{vm_1_peer_id}" ("" "") [$stream_2]))))
        "#);

    let result = checked_call_vm!(vm_1, <_>::default(), &script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_state = vec![
        executed_state::stream_number(0, 0),
        executed_state::stream_number(1, 1),
        executed_state::canon(vec![
            123, 34, 118, 97, 108, 117, 101, 115, 34, 58, 91, 123, 34, 114, 101, 115, 117, 108, 116, 34, 58, 48, 44,
            34, 116, 101, 116, 114, 97, 112, 108, 101, 116, 34, 58, 123, 34, 112, 101, 101, 114, 95, 112, 107, 34, 58,
            34, 118, 109, 95, 49, 95, 112, 101, 101, 114, 95, 105, 100, 34, 44, 34, 115, 101, 114, 118, 105, 99, 101,
            95, 105, 100, 34, 58, 34, 34, 44, 34, 102, 117, 110, 99, 116, 105, 111, 110, 95, 110, 97, 109, 101, 34, 58,
            34, 34, 44, 34, 106, 115, 111, 110, 95, 112, 97, 116, 104, 34, 58, 34, 34, 125, 44, 34, 116, 114, 97, 99,
            101, 95, 112, 111, 115, 34, 58, 48, 125, 44, 123, 34, 114, 101, 115, 117, 108, 116, 34, 58, 49, 44, 34,
            116, 101, 116, 114, 97, 112, 108, 101, 116, 34, 58, 123, 34, 112, 101, 101, 114, 95, 112, 107, 34, 58, 34,
            118, 109, 95, 49, 95, 112, 101, 101, 114, 95, 105, 100, 34, 44, 34, 115, 101, 114, 118, 105, 99, 101, 95,
            105, 100, 34, 58, 34, 115, 111, 109, 101, 95, 115, 101, 114, 118, 105, 99, 101, 95, 110, 97, 109, 101, 34,
            44, 34, 102, 117, 110, 99, 116, 105, 111, 110, 95, 110, 97, 109, 101, 34, 58, 34, 115, 111, 109, 101, 95,
            102, 117, 110, 99, 116, 105, 111, 110, 95, 110, 97, 109, 101, 34, 44, 34, 106, 115, 111, 110, 95, 112, 97,
            116, 104, 34, 58, 34, 34, 125, 44, 34, 116, 114, 97, 99, 101, 95, 112, 111, 115, 34, 58, 49, 125, 93, 44,
            34, 116, 101, 116, 114, 97, 112, 108, 101, 116, 34, 58, 123, 34, 112, 101, 101, 114, 95, 112, 107, 34, 58,
            34, 118, 109, 95, 49, 95, 112, 101, 101, 114, 95, 105, 100, 34, 44, 34, 115, 101, 114, 118, 105, 99, 101,
            95, 105, 100, 34, 58, 34, 34, 44, 34, 102, 117, 110, 99, 116, 105, 111, 110, 95, 110, 97, 109, 101, 34, 58,
            34, 34, 44, 34, 106, 115, 111, 110, 95, 112, 97, 116, 104, 34, 58, 34, 34, 125, 125,
        ]),
        executed_state::ap(0),
        executed_state::scalar(json!([[0, 1]])),
    ];
    assert_eq!(actual_trace, expected_state);

    let expected_tetraplet = RefCell::new(vec![vec![SecurityTetraplet::new(vm_1_peer_id, "", "", "")]]);
    assert_eq!(tetraplet_checker.as_ref(), &expected_tetraplet);
}
