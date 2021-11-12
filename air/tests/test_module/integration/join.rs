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
fn join_chat() {
    use std::collections::HashSet;

    let members_call_service1: CallServiceClosure =
        Box::new(|_| -> CallServiceResult { CallServiceResult::ok(json!([["A", "Relay1"], ["B", "Relay2"]])) });

    let mut relay_1 = create_avm(unit_call_service(), "Relay1");
    let mut relay_2 = create_avm(unit_call_service(), "Relay2");
    let mut remote = create_avm(members_call_service1, "Remote");
    let mut client_1 = create_avm(unit_call_service(), "A");
    let mut client_2 = create_avm(unit_call_service(), "B");

    let script = r#"
            (seq
                (call "Relay1" ("identity" "") [] $void1)
                (seq
                    (call "Remote" ("552196ea-b9b2-4761-98d4-8e7dba77fac4" "add") [] $void2)
                    (seq
                        (call "Remote" ("920e3ba3-cbdf-4ae3-8972-0fa2f31fffd9" "get_users") [] members)
                        (fold members m
                            (par
                                (seq
                                    (call m.$.[1]! ("identity" "") [] $void)
                                    (call m.$.[0]! ("fgemb3" "add") [] $void3)
                                )
                                (next m)
                            )
                        )
                    )
                )
            )
        "#;

    let client_1_result = checked_call_vm!(client_1, "asd", script, "", "");

    let client_1_actual_trace = trace_from_result(&client_1_result);
    let client_1_expected_trace = vec![executed_state::request_sent_by("A")];

    assert_eq!(client_1_actual_trace, client_1_expected_trace);
    assert_eq!(client_1_result.next_peer_pks, vec![String::from("Relay1")]);

    let relay_1_result = checked_call_vm!(relay_1, "asd", script.clone(), client_1_result.data, "");

    let relay_1_actual_trace = trace_from_result(&relay_1_result);
    let relay_1_expected_trace = vec![
        executed_state::stream_string("test", 0),
        executed_state::request_sent_by("Relay1"),
    ];

    assert_eq!(relay_1_actual_trace, relay_1_expected_trace);
    assert_eq!(relay_1_result.next_peer_pks, vec![String::from("Remote")]);

    let remote_result = checked_call_vm!(remote, "asd", script.clone(), relay_1_result.data, "");

    let remote_actual_trace = trace_from_result(&remote_result);
    let remote_expected_trace = vec![
        executed_state::stream_string("test", 0),
        executed_state::stream(json!([["A", "Relay1"], ["B", "Relay2"]]), 0),
        executed_state::scalar(json!([["A", "Relay1"], ["B", "Relay2"]])),
        executed_state::par(1, 2),
        executed_state::request_sent_by("Remote"),
        executed_state::par(1, 0),
        executed_state::request_sent_by("Remote"),
    ];

    let remote_result_next_peer_pks: HashSet<_> = remote_result.next_peer_pks.iter().map(|s| s.as_str()).collect();
    let next_peer_pks_right = maplit::hashset! {
        "Relay1",
        "Relay2",
    };

    assert_eq!(remote_actual_trace, remote_expected_trace);
    assert_eq!(remote_result_next_peer_pks, next_peer_pks_right);

    let relay_1_result = checked_call_vm!(relay_1, "asd", script.clone(), remote_result.data.clone(), "");

    let relay_1_actual_trace = trace_from_result(&relay_1_result);

    let relay_1_expected_trace = vec![
        executed_state::stream_string("test", 0),
        executed_state::stream(json!([["A", "Relay1"], ["B", "Relay2"]]), 0),
        executed_state::scalar(json!([["A", "Relay1"], ["B", "Relay2"]])),
        executed_state::par(2, 2),
        executed_state::stream_string("test", 0),
        executed_state::request_sent_by("Relay1"),
        executed_state::par(1, 0),
        executed_state::request_sent_by("Remote"),
    ];

    assert_eq!(relay_1_actual_trace, relay_1_expected_trace);
    assert_eq!(relay_1_result.next_peer_pks, vec![String::from("A")]);

    let client_1_result = checked_call_vm!(client_1, "asd", script.clone(), relay_1_result.data, "");

    let client_1_actual_trace = trace_from_result(&client_1_result);

    let client_1_expected_trace = vec![
        executed_state::stream_string("test", 0),
        executed_state::stream(json!([["A", "Relay1"], ["B", "Relay2"]]), 0),
        executed_state::scalar(json!([["A", "Relay1"], ["B", "Relay2"]])),
        executed_state::par(2, 2),
        executed_state::stream_string("test", 0),
        executed_state::stream_string("test", 0),
        executed_state::par(1, 0),
        executed_state::request_sent_by("Remote"),
    ];

    assert_eq!(client_1_actual_trace, client_1_expected_trace);
    assert_eq!(client_1_result.next_peer_pks, Vec::<String>::new());

    let relay_2_result = checked_call_vm!(relay_2, "asd", script.clone(), remote_result.data, "");

    let relay_2_actual_trace = trace_from_result(&relay_2_result);

    let relay_2_expected_trace = vec![
        executed_state::stream_string("test", 0),
        executed_state::stream(json!([["A", "Relay1"], ["B", "Relay2"]]), 0),
        executed_state::scalar(json!([["A", "Relay1"], ["B", "Relay2"]])),
        executed_state::par(1, 3),
        executed_state::request_sent_by("Remote"),
        executed_state::par(2, 0),
        executed_state::stream_string("test", 0),
        executed_state::request_sent_by("Relay2"),
    ];

    assert_eq!(relay_2_actual_trace, relay_2_expected_trace);
    assert_eq!(relay_2_result.next_peer_pks, vec![String::from("B")]);

    let client_2_result = checked_call_vm!(client_2, "asd", script, relay_2_result.data, "");

    let client_2_actual_trace = trace_from_result(&client_2_result);

    let client_2_expected_trace = vec![
        executed_state::stream_string("test", 0),
        executed_state::stream(json!([["A", "Relay1"], ["B", "Relay2"]]), 0),
        executed_state::scalar(json!([["A", "Relay1"], ["B", "Relay2"]])),
        executed_state::par(1, 3),
        executed_state::request_sent_by("Remote"),
        executed_state::par(2, 0),
        executed_state::stream_string("test", 0),
        executed_state::stream_string("test", 0),
    ];

    assert_eq!(client_2_actual_trace, client_2_expected_trace);
    assert_eq!(client_2_result.next_peer_pks, Vec::<String>::new());
}

#[test]
fn join() {
    let members_call_service1: CallServiceClosure =
        Box::new(|_| -> CallServiceResult { CallServiceResult::ok(json!([["A"], ["B"]])) });

    let mut relay_1 = create_avm(unit_call_service(), "Relay1");
    let mut remote = create_avm(members_call_service1, "Remote");
    let mut client_1 = create_avm(unit_call_service(), "A");

    let script = r#"
            (seq
                (call "Relay1" ("identity" "") [] $void1)
                (seq
                    (call "Remote" ("920e3ba3-cbdf-4ae3-8972-0fa2f31fffd9" "get_users") [] members)
                    (fold members m
                        (par
                            (seq
                                (call "Relay1" ("identity" "") [] $void)
                                (call "A" ("fgemb3" "add") [m] $void3)
                            )
                            (next m)
                        )
                    )
                )
            )
        "#;

    let client_1_result = checked_call_vm!(client_1, "asd", script, "", "");
    let relay_1_result = checked_call_vm!(relay_1, "asd", script, client_1_result.data, "");
    let remote_result = checked_call_vm!(remote, "asd", script, relay_1_result.data, "");
    let relay_1_result = checked_call_vm!(relay_1, "asd", script, remote_result.data, "");
    let client_1_result = checked_call_vm!(client_1, "asd", script, relay_1_result.data, "");

    let client_1_actual_trace = trace_from_result(&client_1_result);

    let client_1_expected_trace = vec![
        executed_state::stream_string("test", 0),
        executed_state::scalar(json!([["A"], ["B"]])),
        executed_state::par(2, 3),
        executed_state::stream_string("test", 0),
        executed_state::stream_string("test", 0),
        executed_state::par(2, 0),
        executed_state::stream_string("test", 0),
        executed_state::stream_string("test", 0),
    ];

    assert_eq!(client_1_actual_trace, client_1_expected_trace);
    assert_eq!(client_1_result.next_peer_pks, Vec::<String>::new());
}

#[test]
fn init_peer_id() {
    let members_call_service1: CallServiceClosure =
        Box::new(|_| -> CallServiceResult { CallServiceResult::ok(json!([["A"], ["B"]])) });

    let initiator_peer_id = String::from("initiator");

    let mut relay_1 = create_avm(unit_call_service(), "Relay1");
    let mut remote = create_avm(members_call_service1, "Remote");
    let mut client_1 = create_avm(unit_call_service(), "A");
    let mut initiator = create_avm(unit_call_service(), initiator_peer_id.clone());

    let script = r#"(seq
                (seq
                    (call "Relay1" ("identity" "") [])
                    (seq
                        (call "Remote" ("920e3ba3-cbdf-4ae3-8972-0fa2f31fffd9" "get_users") [] members)
                        (fold members m
                            (par
                                (seq
                                    (call "Relay1" ("identity" "") [])
                                    (call "A" ("fgemb3" "add") [m])
                                )
                                (next m)
                            )
                        )
                    )
                )
                (call %init_peer_id% ("identity" "") [])
            )
        "#;

    let initiator_1_result = checked_call_vm!(initiator, &initiator_peer_id, script, "", "");
    let client_1_result = checked_call_vm!(client_1, &initiator_peer_id, script, initiator_1_result.data, "");
    let relay_1_result = checked_call_vm!(relay_1, &initiator_peer_id, script, client_1_result.data, "");
    let remote_result = checked_call_vm!(remote, &initiator_peer_id, script, relay_1_result.data, "");
    let relay_1_result = checked_call_vm!(relay_1, &initiator_peer_id, script, remote_result.data, "");
    let client_1_result = checked_call_vm!(client_1, &initiator_peer_id, script, relay_1_result.data, "");

    let client_1_actual_trace = trace_from_result(&client_1_result);

    let client_1_expected_trace = vec![
        executed_state::scalar_string("test"),
        executed_state::scalar(json!([["A"], ["B"]])),
        executed_state::par(2, 3),
        executed_state::scalar_string("test"),
        executed_state::scalar_string("test"),
        executed_state::par(2, 0),
        executed_state::scalar_string("test"),
        executed_state::scalar_string("test"),
        executed_state::request_sent_by("A"),
    ];

    assert_eq!(client_1_actual_trace, client_1_expected_trace);
    assert_eq!(client_1_result.next_peer_pks, vec![initiator_peer_id.clone()]);

    let initiator_1_result = checked_call_vm!(initiator, initiator_peer_id, script, client_1_result.data, "");

    let initiator_1_actual_trace = trace_from_result(&initiator_1_result);

    let initiator_1_expected_trace = vec![
        executed_state::scalar_string("test"),
        executed_state::scalar(json!([["A"], ["B"]])),
        executed_state::par(2, 3),
        executed_state::scalar_string("test"),
        executed_state::scalar_string("test"),
        executed_state::par(2, 0),
        executed_state::scalar_string("test"),
        executed_state::scalar_string("test"),
        executed_state::scalar_string("test"),
    ];

    assert_eq!(initiator_1_actual_trace, initiator_1_expected_trace);
    assert_eq!(initiator_1_result.next_peer_pks, Vec::<String>::new());
}
