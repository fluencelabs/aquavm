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

use pretty_assertions::assert_eq;

#[tokio::test]
async fn check_that_scalar_is_visible_only_inside_fold_block() {
    let variable_setter_peer_id = "variable_setter_peer_id";
    let mut variable_setter_vm = create_avm(set_variable_call_service(json!([1, 2, 3])), variable_setter_peer_id).await;

    let fallible_peer_id = "fallible_peer_id";
    let mut fallible_peer_vm = create_avm(fallible_call_service("fail"), fallible_peer_id).await;

    let variable_receiver_peer_id = "variable_receiver_peer_id";
    let mut variable_receiver_peer_vm = create_avm(echo_call_service(), variable_receiver_peer_id).await;

    let script = format!(
        r#"
        (seq
            (call "{variable_setter_peer_id}" ("" "") ["iterable_1"] iterable_1)
            (xor
                (fold iterable_1 iterator_1
                     (seq
                         (call "{variable_setter_peer_id}" ("" "") [] scalar)
                         (seq
                             (next iterator_1)
                             (call "{fallible_peer_id}" ("fail" "") [] scalar)
                         )
                     )
                )
                (call "{variable_receiver_peer_id}" ("" "") [scalar])
            )
        )
    "#
    );

    let result = checked_call_vm!(variable_setter_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(fallible_peer_vm, <_>::default(), &script, "", result.data);
    let result = checked_call_vm!(variable_receiver_peer_vm, <_>::default(), &script, "", result.data);
    let actual_trace = trace_from_result(&result);

    let expected_trace = ExecutionTrace::from(vec![
        scalar!(
            json!([1, 2, 3]),
            peer = variable_setter_peer_id,
            args = vec!["iterable_1"]
        ),
        scalar!(json!([1, 2, 3]), peer = variable_setter_peer_id),
        scalar!(json!([1, 2, 3]), peer = variable_setter_peer_id),
        scalar!(json!([1, 2, 3]), peer = variable_setter_peer_id),
        failed!(
            1,
            "failed result from fallible_call_service",
            peer = fallible_peer_id,
            service = "fail"
        ),
        executed_state::request_sent_by(fallible_peer_id),
    ]);

    assert_eq!(actual_trace, expected_trace);
}

#[tokio::test]
async fn scopes_check_that_scalar_not_overwritten_by_fold_end() {
    let variable_setter_peer_id = "variable_setter_peer_id";
    let mut variable_setter_vm = create_avm(set_variable_call_service(json!([1, 2, 3])), variable_setter_peer_id).await;

    let fallible_peer_id = "fallible_peer_id";
    let mut fallible_peer_vm = create_avm(fallible_call_service("fail"), fallible_peer_id).await;

    let variable_receiver_peer_id = "variable_receiver_peer_id";
    let mut variable_receiver_peer_vm = create_avm(echo_call_service(), variable_receiver_peer_id).await;

    let script = format!(
        r#"
        (seq
            (seq
                (call "{variable_setter_peer_id}" ("" "") ["iterable_1"] iterable_1)
                (call "{variable_setter_peer_id}" ("" "") ["scalar"] scalar)
            )
            (xor
                (fold iterable_1 iterator_1
                     (seq
                         (call "{variable_setter_peer_id}" ("" "") [] scalar)
                         (seq
                             (next iterator_1)
                             (call "{fallible_peer_id}" ("fail" "") [] scalar)
                         )
                     )
                )
                (call "{variable_receiver_peer_id}" ("" "") [scalar])
            )
        )
    "#
    );

    let result = checked_call_vm!(variable_setter_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(fallible_peer_vm, <_>::default(), &script, "", result.data);
    let result = checked_call_vm!(variable_receiver_peer_vm, <_>::default(), &script, "", result.data);
    let actual_trace = trace_from_result(&result);

    let expected_trace = ExecutionTrace::from(vec![
        scalar!(
            json!([1, 2, 3]),
            peer = variable_setter_peer_id,
            args = vec!["iterable_1"]
        ),
        scalar!(json!([1, 2, 3]), peer = variable_setter_peer_id, args = vec!["scalar"]),
        scalar!(json!([1, 2, 3]), peer = variable_setter_peer_id),
        scalar!(json!([1, 2, 3]), peer = variable_setter_peer_id),
        scalar!(json!([1, 2, 3]), peer = variable_setter_peer_id),
        failed!(
            1,
            "failed result from fallible_call_service",
            peer = fallible_peer_id,
            service = "fail"
        ),
        unused!(
            json!([1, 2, 3]),
            peer = variable_receiver_peer_id,
            args = vec![json!([1, 2, 3])]
        ),
    ]);

    assert_eq!(actual_trace, expected_trace);
}
