/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use air_interpreter_data::ExecutionTrace;
use air_test_utils::prelude::*;

use futures::FutureExt;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn join_chat_1() {
    use std::collections::HashSet;

    let relay_1_peer_id = "relay_1_peer_id";
    let mut relay_1 = create_avm(unit_call_service(), relay_1_peer_id).await;

    let relay_2_peer_id = "relay_2_peer_id";
    let mut relay_2 = create_avm(unit_call_service(), relay_2_peer_id).await;

    let client_1_peer_id = "client_1_peer_id";
    let mut client_1 = create_avm(unit_call_service(), client_1_peer_id).await;

    let client_2_peer_id = "client_2_peer_id";
    let mut client_2 = create_avm(unit_call_service(), client_2_peer_id).await;

    let remote_peer_id = "remote_peer_id";
    let members = json!([[client_1_peer_id, relay_1_peer_id], [client_2_peer_id, relay_2_peer_id]]);
    let members_call_service: CallServiceClosure = Box::new(move |_| {
        let result = CallServiceResult::ok(members.clone());
        async move { result }.boxed_local()
    });
    let mut remote = create_avm(members_call_service, remote_peer_id).await;

    let script = format!(
        r#"
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
        "#
    );

    let client_1_result = checked_call_vm!(client_1, <_>::default(), &script, "", "");

    let client_1_actual_trace = trace_from_result(&client_1_result);
    let client_1_expected_trace = vec![executed_state::request_sent_by(client_1_peer_id)];

    assert_eq!(client_1_actual_trace, client_1_expected_trace);
    assert_eq!(client_1_result.next_peer_pks, vec![String::from(relay_1_peer_id)]);

    let relay_1_result = checked_call_vm!(relay_1, <_>::default(), &script, client_1_result.data, "");

    let relay_1_actual_trace = trace_from_result(&relay_1_result);
    let relay_1_expected_trace = vec![
        stream!(
            "result from unit_call_service",
            0,
            peer = relay_1_peer_id,
            service = "identity"
        ),
        executed_state::request_sent_by(relay_1_peer_id),
    ];

    assert_eq!(relay_1_actual_trace, relay_1_expected_trace);
    assert_eq!(relay_1_result.next_peer_pks, vec![String::from(remote_peer_id)]);

    let remote_result = checked_call_vm!(remote, <_>::default(), &script, relay_1_result.data, "");

    let remote_actual_trace = trace_from_result(&remote_result);
    let remote_expected_trace = vec![
        stream!(
            "result from unit_call_service",
            0,
            peer = relay_1_peer_id,
            service = "identity"
        ),
        stream!(
            json!([[client_1_peer_id, relay_1_peer_id], [client_2_peer_id, relay_2_peer_id]]),
            0,
            peer = remote_peer_id,
            service = "552196ea-b9b2-4761-98d4-8e7dba77fac4",
            function = "add"
        ),
        scalar!(
            json!([[client_1_peer_id, relay_1_peer_id], [client_2_peer_id, relay_2_peer_id]]),
            peer = remote_peer_id,
            service = "920e3ba3-cbdf-4ae3-8972-0fa2f31fffd9",
            function = "get_users"
        ),
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

    let relay_1_result = checked_call_vm!(relay_1, <_>::default(), &script, remote_result.data.clone(), "");

    let relay_1_actual_trace = trace_from_result(&relay_1_result);

    let relay_1_expected_trace = ExecutionTrace::from(vec![
        stream!(
            "result from unit_call_service",
            0,
            peer = relay_1_peer_id,
            service = "identity"
        ),
        stream!(
            json!([[client_1_peer_id, relay_1_peer_id], [client_2_peer_id, relay_2_peer_id]]),
            0,
            peer = remote_peer_id,
            service = "552196ea-b9b2-4761-98d4-8e7dba77fac4",
            function = "add"
        ),
        scalar!(
            json!([[client_1_peer_id, relay_1_peer_id], [client_2_peer_id, relay_2_peer_id]]),
            peer = remote_peer_id,
            service = "920e3ba3-cbdf-4ae3-8972-0fa2f31fffd9",
            function = "get_users"
        ),
        executed_state::par(2, 2),
        stream!(
            "result from unit_call_service",
            0,
            peer = relay_1_peer_id,
            service = "identity"
        ),
        executed_state::request_sent_by(relay_1_peer_id),
        executed_state::par(1, 0),
        executed_state::request_sent_by(remote_peer_id),
    ]);

    assert_eq!(relay_1_actual_trace, relay_1_expected_trace);
    assert_eq!(relay_1_result.next_peer_pks, vec![String::from(client_1_peer_id)]);

    let client_1_result = checked_call_vm!(client_1, <_>::default(), &script, relay_1_result.data, "");

    let client_1_actual_trace = trace_from_result(&client_1_result);

    let client_1_expected_trace = ExecutionTrace::from(vec![
        stream!(
            "result from unit_call_service",
            0,
            peer = relay_1_peer_id,
            service = "identity"
        ),
        stream!(
            json!([[client_1_peer_id, relay_1_peer_id], [client_2_peer_id, relay_2_peer_id]]),
            0,
            peer = remote_peer_id,
            service = "552196ea-b9b2-4761-98d4-8e7dba77fac4",
            function = "add"
        ),
        scalar!(
            json!([[client_1_peer_id, relay_1_peer_id], [client_2_peer_id, relay_2_peer_id]]),
            peer = remote_peer_id,
            service = "920e3ba3-cbdf-4ae3-8972-0fa2f31fffd9",
            function = "get_users"
        ),
        executed_state::par(2, 2),
        stream!(
            "result from unit_call_service",
            0,
            peer = relay_1_peer_id,
            service = "identity"
        ),
        stream!(
            "result from unit_call_service",
            0,
            peer = client_1_peer_id,
            service = "fgemb3",
            function = "add"
        ),
        executed_state::par(1, 0),
        executed_state::request_sent_by(remote_peer_id),
    ]);

    assert_eq!(client_1_actual_trace, client_1_expected_trace);
    assert!(client_1_result.next_peer_pks.is_empty());

    let relay_2_result = checked_call_vm!(relay_2, <_>::default(), &script, remote_result.data, "");

    let relay_2_actual_trace = trace_from_result(&relay_2_result);

    let relay_2_expected_trace = ExecutionTrace::from(vec![
        stream!(
            "result from unit_call_service",
            0,
            peer = relay_1_peer_id,
            service = "identity"
        ),
        stream!(
            json!([[client_1_peer_id, relay_1_peer_id], [client_2_peer_id, relay_2_peer_id]]),
            0,
            peer = remote_peer_id,
            service = "552196ea-b9b2-4761-98d4-8e7dba77fac4",
            function = "add"
        ),
        scalar!(
            json!([[client_1_peer_id, relay_1_peer_id], [client_2_peer_id, relay_2_peer_id]]),
            peer = remote_peer_id,
            service = "920e3ba3-cbdf-4ae3-8972-0fa2f31fffd9",
            function = "get_users"
        ),
        executed_state::par(1, 3),
        executed_state::request_sent_by(remote_peer_id),
        executed_state::par(2, 0),
        stream!(
            "result from unit_call_service",
            0,
            peer = relay_2_peer_id,
            service = "identity"
        ),
        executed_state::request_sent_by(relay_2_peer_id),
    ]);

    assert_eq!(relay_2_actual_trace, relay_2_expected_trace);
    assert_eq!(relay_2_result.next_peer_pks, vec![String::from(client_2_peer_id)]);

    let client_2_result = checked_call_vm!(client_2, <_>::default(), script, relay_2_result.data, "");

    let client_2_actual_trace = trace_from_result(&client_2_result);

    let client_2_expected_trace = ExecutionTrace::from(vec![
        stream!(
            "result from unit_call_service",
            0,
            peer = relay_1_peer_id,
            service = "identity"
        ),
        stream!(
            json!([[client_1_peer_id, relay_1_peer_id], [client_2_peer_id, relay_2_peer_id]]),
            0,
            peer = remote_peer_id,
            service = "552196ea-b9b2-4761-98d4-8e7dba77fac4",
            function = "add"
        ),
        scalar!(
            json!([[client_1_peer_id, relay_1_peer_id], [client_2_peer_id, relay_2_peer_id]]),
            peer = remote_peer_id,
            service = "920e3ba3-cbdf-4ae3-8972-0fa2f31fffd9",
            function = "get_users"
        ),
        executed_state::par(1, 3),
        executed_state::request_sent_by(remote_peer_id),
        executed_state::par(2, 0),
        stream!(
            "result from unit_call_service",
            0,
            peer = relay_2_peer_id,
            service = "identity"
        ),
        stream!(
            "result from unit_call_service",
            0,
            peer = client_2_peer_id,
            service = "fgemb3",
            function = "add"
        ),
    ]);

    assert_eq!(client_2_actual_trace, client_2_expected_trace);
    assert!(client_2_result.next_peer_pks.is_empty());
}

#[tokio::test]
async fn join_chat_2() {
    let members_call_service1: CallServiceClosure = Box::new(|_| {
        let result = CallServiceResult::ok(json!([["A"], ["B"]]));
        async move { result }.boxed_local()
    });

    let relay_1_peer_id = "relay_1_peer_id";
    let mut relay_1 = create_avm(unit_call_service(), relay_1_peer_id).await;

    let remote_peer_id = "remove_peer_id";
    let mut remote = create_avm(members_call_service1, remote_peer_id).await;

    let client_peer_id = "client_peer_id";
    let mut client_1 = create_avm(unit_call_service(), client_peer_id).await;

    let script = format!(
        r#"
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
        "#
    );

    let client_1_result = checked_call_vm!(client_1, <_>::default(), &script, "", "");
    let relay_1_result = checked_call_vm!(relay_1, <_>::default(), &script, client_1_result.data, "");
    let remote_result = checked_call_vm!(remote, <_>::default(), &script, relay_1_result.data, "");
    let relay_1_result = checked_call_vm!(relay_1, <_>::default(), &script, remote_result.data, "");
    let client_1_result = checked_call_vm!(client_1, <_>::default(), script, relay_1_result.data, "");

    let client_1_actual_trace = trace_from_result(&client_1_result);

    let client_1_expected_trace = ExecutionTrace::from(vec![
        stream!(
            "result from unit_call_service",
            0,
            peer = relay_1_peer_id,
            service = "identity"
        ),
        scalar!(
            json!([["A"], ["B"]]),
            peer = remote_peer_id,
            service = "920e3ba3-cbdf-4ae3-8972-0fa2f31fffd9",
            function = "get_users"
        ),
        executed_state::par(2, 3),
        stream!(
            "result from unit_call_service",
            0,
            peer = relay_1_peer_id,
            service = "identity"
        ),
        stream!(
            "result from unit_call_service",
            0,
            peer = client_peer_id,
            service = "fgemb3",
            function = "add",
            args = [json!(["A"])]
        ),
        executed_state::par(2, 0),
        stream!(
            "result from unit_call_service",
            0,
            peer = relay_1_peer_id,
            service = "identity"
        ),
        stream!(
            "result from unit_call_service",
            0,
            peer = client_peer_id,
            service = "fgemb3",
            function = "add",
            args = [json!(["B"])]
        ),
    ]);

    assert_eq!(client_1_actual_trace, client_1_expected_trace);
    assert!(client_1_result.next_peer_pks.is_empty());
}

#[tokio::test]
async fn init_peer_id() {
    let relay_1_peer_id = "relay_1_peer_id";
    let mut relay_1 = create_avm(unit_call_service(), relay_1_peer_id).await;

    let client_1_peer_id = "client_1_peer_id";
    let mut client_1 = create_avm(unit_call_service(), client_1_peer_id).await;

    let initiator_peer_id = "initiator_peer_id";
    let mut initiator = create_avm(unit_call_service(), initiator_peer_id).await;

    let remote_peer_id = "remote_peer_id";
    let members = json!([[client_1_peer_id], ["B"]]);
    let members_call_service: CallServiceClosure = Box::new(move |_| {
        let result = CallServiceResult::ok(members.clone());
        async move { result }.boxed_local()
    });
    let mut remote = create_avm(members_call_service, remote_peer_id).await;

    let script = format!(
        r#"(seq
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
        "#
    );

    let test_params = TestRunParameters::from_init_peer_id(initiator_peer_id);
    let initiator_1_result = checked_call_vm!(initiator, test_params.clone(), &script, "", "");
    let client_1_result = checked_call_vm!(client_1, test_params.clone(), &script, initiator_1_result.data, "");
    let relay_1_result = checked_call_vm!(relay_1, test_params.clone(), &script, client_1_result.data, "");
    let remote_result = checked_call_vm!(remote, test_params.clone(), &script, relay_1_result.data, "");
    let relay_1_result = checked_call_vm!(relay_1, test_params.clone(), &script, remote_result.data, "");
    let client_1_result = checked_call_vm!(client_1, test_params.clone(), &script, relay_1_result.data, "");

    let client_1_actual_trace = trace_from_result(&client_1_result);

    let client_1_expected_trace = ExecutionTrace::from(vec![
        unused!(
            "result from unit_call_service",
            peer = relay_1_peer_id,
            service = "identity"
        ),
        scalar!(
            json!([[client_1_peer_id], ["B"]]),
            peer = remote_peer_id,
            service = "920e3ba3-cbdf-4ae3-8972-0fa2f31fffd9",
            function = "get_users"
        ),
        executed_state::par(2, 3),
        unused!(
            "result from unit_call_service",
            peer = relay_1_peer_id,
            service = "identity"
        ),
        unused!(
            "result from unit_call_service",
            peer = client_1_peer_id,
            service = "fgemb3",
            function = "add",
            args = [json!([client_1_peer_id])]
        ),
        executed_state::par(2, 0),
        unused!(
            "result from unit_call_service",
            peer = relay_1_peer_id,
            service = "identity"
        ),
        unused!(
            "result from unit_call_service",
            peer = client_1_peer_id,
            service = "fgemb3",
            function = "add",
            args = [json!(["B"])]
        ),
        executed_state::request_sent_by(client_1_peer_id),
    ]);

    assert_eq!(client_1_actual_trace, client_1_expected_trace);
    assert_eq!(client_1_result.next_peer_pks, vec![initiator_peer_id.to_string()]);

    let initiator_1_result = checked_call_vm!(initiator, test_params, script, client_1_result.data, "");

    let initiator_1_actual_trace = trace_from_result(&initiator_1_result);

    let initiator_1_expected_trace = ExecutionTrace::from(vec![
        unused!(
            "result from unit_call_service",
            peer = relay_1_peer_id,
            service = "identity"
        ),
        scalar!(
            json!([[client_1_peer_id], ["B"]]),
            peer = remote_peer_id,
            service = "920e3ba3-cbdf-4ae3-8972-0fa2f31fffd9",
            function = "get_users"
        ),
        executed_state::par(2, 3),
        unused!(
            "result from unit_call_service",
            peer = relay_1_peer_id,
            service = "identity"
        ),
        unused!(
            "result from unit_call_service",
            peer = client_1_peer_id,
            service = "fgemb3",
            function = "add",
            args = [json!([client_1_peer_id])]
        ),
        executed_state::par(2, 0),
        unused!(
            "result from unit_call_service",
            peer = relay_1_peer_id,
            service = "identity"
        ),
        unused!(
            "result from unit_call_service",
            peer = client_1_peer_id,
            service = "fgemb3",
            function = "add",
            args = [json!(["B"])]
        ),
        unused!(
            "result from unit_call_service",
            peer = initiator_peer_id,
            service = "identity"
        ),
    ]);

    assert_eq!(initiator_1_actual_trace, initiator_1_expected_trace);
    assert!(initiator_1_result.next_peer_pks.is_empty());
}
