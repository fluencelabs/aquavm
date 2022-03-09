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
fn join_chat_1() {
    use std::collections::HashSet;

    let relay_1_peer_id = "relay_1_peer_id";
    let mut relay_1 = create_avm(unit_call_service(), relay_1_peer_id);

    let relay_2_peer_id = "relay_2_peer_id";
    let mut relay_2 = create_avm(unit_call_service(), relay_2_peer_id);

    let client_1_peer_id = "client_1_peer_id";
    let mut client_1 = create_avm(unit_call_service(), client_1_peer_id);

    let client_2_peer_id = "client_2_peer_id";
    let mut client_2 = create_avm(unit_call_service(), client_2_peer_id);

    let remote_peer_id = "remote_peer_id";
    let members = json!([[client_1_peer_id, relay_1_peer_id], [client_2_peer_id, relay_2_peer_id]]);
    let members_call_service: CallServiceClosure =
        Box::new(move |_| -> CallServiceResult { CallServiceResult::ok(members.clone()) });
    let mut remote = create_avm(members_call_service, remote_peer_id);

    let script = f!(r#"
            (seq
                (call "{relay_1_peer_id}" ("identity" "") [] $void1)
                (seq
                    (call "{remote_peer_id}" ("552196ea-b9b2-4761-98d4-8e7dba77fac4" "add") [] $void2)
                    (seq
                        (call "{remote_peer_id}" ("920e3ba3-cbdf-4ae3-8972-0fa2f31fffd9" "get_users") [] members)
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
        "#);

    let client_1_result = checked_call_vm!(client_1, "asd", &script, "", "");

    let client_1_actual_trace = trace_from_result(&client_1_result);
    let client_1_expected_trace = vec![executed_state::request_sent_by(client_1_peer_id)];

    assert_eq!(client_1_actual_trace, client_1_expected_trace);
    assert_eq!(client_1_result.next_peer_pks, vec![String::from(relay_1_peer_id)]);

    let relay_1_result = checked_call_vm!(relay_1, "asd", &script, client_1_result.data, "");

    let relay_1_actual_trace = trace_from_result(&relay_1_result);
    let relay_1_expected_trace = vec![
        executed_state::stream_string("result from unit_call_service", 0),
        executed_state::request_sent_by(relay_1_peer_id),
    ];

    assert_eq!(relay_1_actual_trace, relay_1_expected_trace);
    assert_eq!(relay_1_result.next_peer_pks, vec![String::from(remote_peer_id)]);

    let remote_result = checked_call_vm!(remote, "asd", &script, relay_1_result.data, "");

    let remote_actual_trace = trace_from_result(&remote_result);
    let remote_expected_trace = vec![
        executed_state::stream_string("result from unit_call_service", 0),
        executed_state::stream(
            json!([[client_1_peer_id, relay_1_peer_id], [client_2_peer_id, relay_2_peer_id]]),
            0,
        ),
        executed_state::scalar(json!([
            [client_1_peer_id, relay_1_peer_id],
            [client_2_peer_id, relay_2_peer_id]
        ])),
        executed_state::par(1, 2),
        executed_state::request_sent_by(remote_peer_id),
        executed_state::par(1, 0),
        executed_state::request_sent_by(remote_peer_id),
    ];

    let actual_remote_next_peer_pks: HashSet<_> = remote_result.next_peer_pks.iter().map(|s| s.as_str()).collect();
    let expected_next_peer_pks = maplit::hashset! {
        relay_1_peer_id,
        relay_2_peer_id,
    };

    assert_eq!(remote_actual_trace, remote_expected_trace);
    assert_eq!(actual_remote_next_peer_pks, expected_next_peer_pks);

    let relay_1_result = checked_call_vm!(relay_1, "asd", &script, remote_result.data.clone(), "");

    let relay_1_actual_trace = trace_from_result(&relay_1_result);

    let relay_1_expected_trace = vec![
        executed_state::stream_string("result from unit_call_service", 0),
        executed_state::stream(
            json!([[client_1_peer_id, relay_1_peer_id], [client_2_peer_id, relay_2_peer_id]]),
            0,
        ),
        executed_state::scalar(json!([
            [client_1_peer_id, relay_1_peer_id],
            [client_2_peer_id, relay_2_peer_id]
        ])),
        executed_state::par(2, 2),
        executed_state::stream_string("result from unit_call_service", 0),
        executed_state::request_sent_by(relay_1_peer_id),
        executed_state::par(1, 0),
        executed_state::request_sent_by(remote_peer_id),
    ];

    assert_eq!(relay_1_actual_trace, relay_1_expected_trace);
    assert_eq!(relay_1_result.next_peer_pks, vec![String::from(client_1_peer_id)]);

    let client_1_result = checked_call_vm!(client_1, "asd", &script, relay_1_result.data, "");

    let client_1_actual_trace = trace_from_result(&client_1_result);

    let client_1_expected_trace = vec![
        executed_state::stream_string("result from unit_call_service", 0),
        executed_state::stream(
            json!([[client_1_peer_id, relay_1_peer_id], [client_2_peer_id, relay_2_peer_id]]),
            0,
        ),
        executed_state::scalar(json!([
            [client_1_peer_id, relay_1_peer_id],
            [client_2_peer_id, relay_2_peer_id]
        ])),
        executed_state::par(2, 2),
        executed_state::stream_string("result from unit_call_service", 0),
        executed_state::stream_string("result from unit_call_service", 0),
        executed_state::par(1, 0),
        executed_state::request_sent_by(remote_peer_id),
    ];

    assert_eq!(client_1_actual_trace, client_1_expected_trace);
    assert!(client_1_result.next_peer_pks.is_empty());

    let relay_2_result = checked_call_vm!(relay_2, "asd", &script, remote_result.data, "");

    let relay_2_actual_trace = trace_from_result(&relay_2_result);

    let relay_2_expected_trace = vec![
        executed_state::stream_string("result from unit_call_service", 0),
        executed_state::stream(
            json!([[client_1_peer_id, relay_1_peer_id], [client_2_peer_id, relay_2_peer_id]]),
            0,
        ),
        executed_state::scalar(json!([
            [client_1_peer_id, relay_1_peer_id],
            [client_2_peer_id, relay_2_peer_id]
        ])),
        executed_state::par(1, 3),
        executed_state::request_sent_by(remote_peer_id),
        executed_state::par(2, 0),
        executed_state::stream_string("result from unit_call_service", 0),
        executed_state::request_sent_by(relay_2_peer_id),
    ];

    assert_eq!(relay_2_actual_trace, relay_2_expected_trace);
    assert_eq!(relay_2_result.next_peer_pks, vec![String::from(client_2_peer_id)]);

    let client_2_result = checked_call_vm!(client_2, "asd", script, relay_2_result.data, "");

    let client_2_actual_trace = trace_from_result(&client_2_result);

    let client_2_expected_trace = vec![
        executed_state::stream_string("result from unit_call_service", 0),
        executed_state::stream(
            json!([[client_1_peer_id, relay_1_peer_id], [client_2_peer_id, relay_2_peer_id]]),
            0,
        ),
        executed_state::scalar(json!([
            [client_1_peer_id, relay_1_peer_id],
            [client_2_peer_id, relay_2_peer_id]
        ])),
        executed_state::par(1, 3),
        executed_state::request_sent_by(remote_peer_id),
        executed_state::par(2, 0),
        executed_state::stream_string("result from unit_call_service", 0),
        executed_state::stream_string("result from unit_call_service", 0),
    ];

    assert_eq!(client_2_actual_trace, client_2_expected_trace);
    assert!(client_2_result.next_peer_pks.is_empty());
}

#[test]
fn join_chat_2() {
    let members_call_service1: CallServiceClosure =
        Box::new(|_| -> CallServiceResult { CallServiceResult::ok(json!([["A"], ["B"]])) });

    let relay_1_peer_id = "relay_1_peer_id";
    let mut relay_1 = create_avm(unit_call_service(), relay_1_peer_id);

    let remote_peer_id = "remove_peer_id";
    let mut remote = create_avm(members_call_service1, remote_peer_id);

    let client_peer_id = "client_peer_id";
    let mut client_1 = create_avm(unit_call_service(), client_peer_id);

    let script = f!(r#"
            (seq
                (call "{relay_1_peer_id}" ("identity" "") [] $void1)
                (seq
                    (call "{remote_peer_id}" ("920e3ba3-cbdf-4ae3-8972-0fa2f31fffd9" "get_users") [] members)
                    (fold members m
                        (par
                            (seq
                                (call "{relay_1_peer_id}" ("identity" "") [] $void)
                                (call "{client_peer_id}" ("fgemb3" "add") [m] $void3)
                            )
                            (next m)
                        )
                    )
                )
            )
        "#);

    let client_1_result = checked_call_vm!(client_1, "asd", &script, "", "");
    let relay_1_result = checked_call_vm!(relay_1, "asd", &script, client_1_result.data, "");
    let remote_result = checked_call_vm!(remote, "asd", &script, relay_1_result.data, "");
    let relay_1_result = checked_call_vm!(relay_1, "asd", &script, remote_result.data, "");
    let client_1_result = checked_call_vm!(client_1, "asd", script, relay_1_result.data, "");

    let client_1_actual_trace = trace_from_result(&client_1_result);

    let client_1_expected_trace = vec![
        executed_state::stream_string("result from unit_call_service", 0),
        executed_state::scalar(json!([["A"], ["B"]])),
        executed_state::par(2, 3),
        executed_state::stream_string("result from unit_call_service", 0),
        executed_state::stream_string("result from unit_call_service", 0),
        executed_state::par(2, 0),
        executed_state::stream_string("result from unit_call_service", 0),
        executed_state::stream_string("result from unit_call_service", 0),
    ];

    assert_eq!(client_1_actual_trace, client_1_expected_trace);
    assert!(client_1_result.next_peer_pks.is_empty());
}

#[test]
fn init_peer_id() {
    let relay_1_peer_id = "relay_1_peer_id";
    let mut relay_1 = create_avm(unit_call_service(), relay_1_peer_id);

    let client_1_peer_id = "client_1_peer_id";
    let mut client_1 = create_avm(unit_call_service(), client_1_peer_id);

    let initiator_peer_id = "initiator_peer_id";
    let mut initiator = create_avm(unit_call_service(), initiator_peer_id);

    let remote_peer_id = "remote_peer_id";
    let members = json!([[client_1_peer_id], ["B"]]);
    let members_call_service: CallServiceClosure =
        Box::new(move |_| -> CallServiceResult { CallServiceResult::ok(members.clone()) });
    let mut remote = create_avm(members_call_service, remote_peer_id);

    let script = f!(r#"(seq
                (seq
                    (call "{relay_1_peer_id}" ("identity" "") [])
                    (seq
                        (call "{remote_peer_id}" ("920e3ba3-cbdf-4ae3-8972-0fa2f31fffd9" "get_users") [] members)
                        (fold members m
                            (par
                                (seq
                                    (call "{relay_1_peer_id}" ("identity" "") [])
                                    (call "{client_1_peer_id}" ("fgemb3" "add") [m])
                                )
                                (next m)
                            )
                        )
                    )
                )
                (call %init_peer_id% ("identity" "") [])
            )
        "#);

    let initiator_1_result = checked_call_vm!(initiator, initiator_peer_id, &script, "", "");
    let client_1_result = checked_call_vm!(client_1, initiator_peer_id, &script, initiator_1_result.data, "");
    let relay_1_result = checked_call_vm!(relay_1, initiator_peer_id, &script, client_1_result.data, "");
    let remote_result = checked_call_vm!(remote, initiator_peer_id, &script, relay_1_result.data, "");
    let relay_1_result = checked_call_vm!(relay_1, initiator_peer_id, &script, remote_result.data, "");
    let client_1_result = checked_call_vm!(client_1, initiator_peer_id, &script, relay_1_result.data, "");

    let client_1_actual_trace = trace_from_result(&client_1_result);

    let client_1_expected_trace = vec![
        executed_state::scalar_string("result from unit_call_service"),
        executed_state::scalar(json!([[client_1_peer_id], ["B"]])),
        executed_state::par(2, 3),
        executed_state::scalar_string("result from unit_call_service"),
        executed_state::scalar_string("result from unit_call_service"),
        executed_state::par(2, 0),
        executed_state::scalar_string("result from unit_call_service"),
        executed_state::scalar_string("result from unit_call_service"),
        executed_state::request_sent_by(client_1_peer_id),
    ];

    assert_eq!(client_1_actual_trace, client_1_expected_trace);
    assert_eq!(client_1_result.next_peer_pks, vec![initiator_peer_id.to_string()]);

    let initiator_1_result = checked_call_vm!(initiator, initiator_peer_id, script, client_1_result.data, "");

    let initiator_1_actual_trace = trace_from_result(&initiator_1_result);

    let initiator_1_expected_trace = vec![
        executed_state::scalar_string("result from unit_call_service"),
        executed_state::scalar(json!([[client_1_peer_id], ["B"]])),
        executed_state::par(2, 3),
        executed_state::scalar_string("result from unit_call_service"),
        executed_state::scalar_string("result from unit_call_service"),
        executed_state::par(2, 0),
        executed_state::scalar_string("result from unit_call_service"),
        executed_state::scalar_string("result from unit_call_service"),
        executed_state::scalar_string("result from unit_call_service"),
    ];

    assert_eq!(initiator_1_actual_trace, initiator_1_expected_trace);
    assert!(initiator_1_result.next_peer_pks.is_empty());
}
