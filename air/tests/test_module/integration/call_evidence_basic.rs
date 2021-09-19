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
fn executed_trace_seq_par_call() {
    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(unit_call_service(), local_peer_id);

    let script = format!(
        r#"
        (seq
            (par
                (call "{0}" ("local_service_id" "local_fn_name") [] result_1)
                (call "remote_peer_id" ("service_id" "fn_name") [] g)
            )
            (call "{0}" ("local_service_id" "local_fn_name") [] result_2)
        )"#,
        local_peer_id
    );

    let initial_trace = vec![par(1, 1), scalar_string("test"), scalar_string("test")];
    let initial_data = raw_data_from_trace(initial_trace);

    let result = checked_call_vm!(vm, "asd", script, "", initial_data);
    let actual_trace = trace_from_result(&result);

    let test_string = "test";

    let expected_trace = vec![
        par(1, 1),
        scalar_string(test_string),
        scalar_string(test_string),
        scalar_string(test_string),
    ];

    assert_eq!(actual_trace, expected_trace);
    assert!(result.next_peer_pks.is_empty());
}

#[test]
fn executed_trace_par_par_call() {
    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(unit_call_service(), local_peer_id);

    let script = format!(
        r#"
        (par
            (par
                (call "{0}" ("local_service_id" "local_fn_name") [] result_1)
                (call "remote_peer_id" ("service_id" "fn_name") [] g)
            )
            (call "{0}" ("local_service_id" "local_fn_name") [] result_2)
        )"#,
        local_peer_id,
    );

    let initial_state = vec![
        par(2, 1),
        par(1, 0),
        request_sent_by("peer_id_1"),
        scalar_string("test"),
    ];

    let initial_data = raw_data_from_trace(initial_state);

    let result = checked_call_vm!(vm, "asd", &script, "", initial_data);
    let actual_trace = trace_from_result(&result);

    let test_string = "test";
    let expected_trace = vec![
        par(3, 1),
        par(1, 1),
        scalar_string(test_string),
        request_sent_by(local_peer_id),
        scalar_string(test_string),
    ];

    assert_eq!(actual_trace, expected_trace);
    assert_eq!(result.next_peer_pks, vec![String::from("remote_peer_id")]);

    let initial_state = vec![
        par(3, 0),
        par(1, 1),
        request_sent_by("peer_id_1"),
        request_sent_by(local_peer_id),
    ];

    let initial_data = raw_data_from_trace(initial_state);

    let result = checked_call_vm!(vm, "asd", script, "", initial_data);
    let actual_trace = trace_from_result(&result);

    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn executed_trace_seq_seq() {
    let peer_id_1 = String::from("12D3KooWHk9BjDQBUqnavciRPhAYFvqKBe4ZiPPvde7vDaqgn5er");
    let peer_id_2 = String::from("12D3KooWAzJcYitiZrerycVB4Wryrx22CFKdDGx7c4u31PFdfTbR");
    let mut vm1 = create_avm(unit_call_service(), peer_id_1.clone());
    let mut vm2 = create_avm(unit_call_service(), peer_id_2.clone());

    let script = format!(
        r#"
        (seq
            (call "{}" ("identity" "") [] void0)
            (seq
                (call "{}" ("add_blueprint" "") [] blueprint_id)
                (call "{}" ("addBlueprint-14d8488e-d10d-474d-96b2-878f6a7d74c8" "") [] void1)
            )
        )
        "#,
        peer_id_1, peer_id_1, peer_id_2
    );

    let result = checked_call_vm!(vm2, "asd", script.clone(), "", "");
    assert_eq!(result.next_peer_pks, vec![peer_id_1.clone()]);

    let result = checked_call_vm!(vm1, "asd", script.clone(), "", result.data);
    assert_eq!(result.next_peer_pks, vec![peer_id_2.clone()]);

    let result = checked_call_vm!(vm2, "asd", script, "", result.data);

    let actual_trace = trace_from_result(&result);

    let test_string = "test";
    let expected_trace = vec![
        scalar_string(test_string),
        scalar_string(test_string),
        scalar_string(test_string),
    ];

    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn executed_trace_create_service() {
    let module = "greeting";
    let module_config = json!(
        {
            "name": module,
            "mem_pages_count": 100,
            "logger_enabled": true,
            "wasi": {
                "envs": json!({}),
                "preopened_files": vec!["/tmp"],
                "mapped_dirs": json!({}),
            }
        }
    );

    let module_bytes = json!([1, 2]);
    let blueprint = json!({ "name": "blueprint", "dependencies": [module]});

    let add_module_response = "add_module response";
    let add_blueprint_response = "add_blueprint response";
    let create_response = "create response";

    let call_service: CallServiceClosure = Box::new(move |params| -> CallServiceResult {
        let response = match params.service_id.as_str() {
            "add_module" => add_module_response,
            "add_blueprint" => add_blueprint_response,
            "create" => create_response,
            _ => "unknown response",
        };
        CallServiceResult::ok(&json!(response))
    });

    let mut vm = create_avm(call_service, "A");

    let script = include_str!("./scripts/create_service.clj");

    let add_module_response = String::from("add_module response");
    let add_blueprint_response = String::from("add_blueprint response");
    let create_response = String::from("create response");
    let expected_trace = vec![
        scalar(module_bytes),
        scalar(module_config),
        scalar(blueprint),
        scalar_string(add_module_response),
        scalar_string(add_blueprint_response),
        scalar_string(create_response),
        scalar_string("test"),
    ];
    let initial_data = raw_data_from_trace(expected_trace.clone());

    let result = checked_call_vm!(vm, "init_peer_id", script, "", initial_data);

    let actual_trace = trace_from_result(&result);

    assert_eq!(actual_trace, expected_trace);
    assert!(result.next_peer_pks.is_empty());
}

#[test]
fn executed_trace_par_seq_fold_call() {
    let return_numbers_call_service: CallServiceClosure = Box::new(|_| -> CallServiceResult {
        CallServiceResult::ok(&json!(["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]))
    });

    let mut vm1 = create_avm(return_numbers_call_service, "some_peer_id_1");
    let mut vm2 = create_avm(echo_call_service(), "some_peer_id_2");
    let mut vm3 = create_avm(unit_call_service(), "some_peer_id_3");

    let script = String::from(
        r#"
        (par
            (seq
                (call "some_peer_id_1" ("local_service_id" "local_fn_name") [] IterableResultPeer1)
                (fold IterableResultPeer1 i
                    (par
                        (call "some_peer_id_2" ("local_service_id" "local_fn_name") [i] $acc)
                        (next i)
                    )
                )
            )
            (call "some_peer_id_3" ("local_service_id" "local_fn_name") [] result_2)
        )"#,
    );

    let result = checked_call_vm!(vm2, "asd", script.clone(), "", "");
    let result = checked_call_vm!(vm1, "asd", script.clone(), "", result.data);
    let mut data = result.data;

    for _ in 0..100 {
        let result = checked_call_vm!(vm2, "asd", script.clone(), "", data);
        data = result.data;
    }

    let result = checked_call_vm!(vm3, "asd", script, "", data);
    let actual_trace = trace_from_result(&result);

    let generation = 0;
    let expected_trace = vec![
        par(21, 1),
        scalar_string_array(vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]),
        par(1, 18),
        stream_string(1.to_string(), generation),
        par(1, 16),
        stream_string(2.to_string(), generation),
        par(1, 14),
        stream_string(3.to_string(), generation),
        par(1, 12),
        stream_string(4.to_string(), generation),
        par(1, 10),
        stream_string(5.to_string(), generation),
        par(1, 8),
        stream_string(6.to_string(), generation),
        par(1, 6),
        stream_string(7.to_string(), generation),
        par(1, 4),
        stream_string(8.to_string(), generation),
        par(1, 2),
        stream_string(9.to_string(), generation),
        par(1, 0),
        stream_string(10.to_string(), generation),
        scalar_string("test"),
    ];

    assert_eq!(actual_trace, expected_trace);
    assert!(result.next_peer_pks.is_empty());
}

#[test]
fn executed_trace_par_seq_fold_in_cycle_call() {
    let return_numbers_call_service: CallServiceClosure = Box::new(|_| -> CallServiceResult {
        CallServiceResult::ok(&json!(["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]))
    });

    let mut vm1 = create_avm(return_numbers_call_service, "some_peer_id_1");
    let mut vm2 = create_avm(echo_call_service(), "some_peer_id_2");
    let mut vm3 = create_avm(unit_call_service(), "some_peer_id_3");

    let script = r#"
        (par 
            (seq 
                (call "some_peer_id_1" ("local_service_id" "local_fn_name") [] IterableResultPeer1)
                (fold IterableResultPeer1 i
                    (par 
                        (call "some_peer_id_2" ("local_service_id" "local_fn_name") [i] $acc)
                        (next i)
                    )
                )
            )
            (call "some_peer_id_3" ("local_service_id" "local_fn_name") [] result_2)
        )"#;

    let mut data = vec![];

    for _ in 0..100 {
        let result = checked_call_vm!(vm1, "asd", script, "", data);
        let result = checked_call_vm!(vm2, "asd", script, "", result.data);
        let result = checked_call_vm!(vm3, "asd", script, "", result.data);

        let actual_trace = trace_from_result(&result);

        let generation = 0;
        let expected_trace = vec![
            par(21, 1),
            scalar_string_array(vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]),
            par(1, 18),
            stream_string(1.to_string(), generation),
            par(1, 16),
            stream_string(2.to_string(), generation),
            par(1, 14),
            stream_string(3.to_string(), generation),
            par(1, 12),
            stream_string(4.to_string(), generation),
            par(1, 10),
            stream_string(5.to_string(), generation),
            par(1, 8),
            stream_string(6.to_string(), generation),
            par(1, 6),
            stream_string(7.to_string(), generation),
            par(1, 4),
            stream_string(8.to_string(), generation),
            par(1, 2),
            stream_string(9.to_string(), generation),
            par(1, 0),
            stream_string(10.to_string(), generation),
            scalar_string("test"),
        ];

        assert_eq!(actual_trace, expected_trace);

        data = result.data;
    }
}

#[test]
fn executed_trace_seq_par_seq_seq() {
    let peer_id_1 = "12D3KooWHk9BjDQBUqnavciRPhAYFvqKBe4ZiPPvde7vDaqgn5er";
    let peer_id_2 = "12D3KooWAzJcYitiZrerycVB4Wryrx22CFKdDGx7c4u31PFdfTbR";
    let mut vm1 = create_avm(unit_call_service(), peer_id_1);
    let mut vm2 = create_avm(unit_call_service(), peer_id_2);
    let script = format!(
        r#"
        (seq 
            (par 
                (seq 
                    (call "{}" ("" "") [] result_1)
                    (call "{}" ("" "") [] result_2)
                )
                (seq 
                    (call "{}" ("" "") [] result_3)
                    (call "{}" ("" "") [] result_4)
                )
            )
            (call "{}" ("" "") [] result_5)
        )
        "#,
        peer_id_1, peer_id_2, peer_id_2, peer_id_1, peer_id_2
    );

    let result = checked_call_vm!(vm2, "asd", &script, "", "");
    assert_eq!(result.next_peer_pks, vec![peer_id_1.to_string()]);

    let result = checked_call_vm!(vm1, "asd", &script, "", result.data);
    assert_eq!(result.next_peer_pks, vec![peer_id_2.to_string()]);

    let result = checked_call_vm!(vm2, "asd", script, "", result.data);

    let actual_trace = trace_from_result(&result);

    let service_result_string = "test";
    let executed_trace = vec![
        par(2, 2),
        scalar_string(service_result_string),
        scalar_string(service_result_string),
        scalar_string(service_result_string),
        scalar_string(service_result_string),
        scalar_string(service_result_string),
    ];

    assert_eq!(actual_trace, executed_trace);
    assert!(result.next_peer_pks.is_empty());
}
