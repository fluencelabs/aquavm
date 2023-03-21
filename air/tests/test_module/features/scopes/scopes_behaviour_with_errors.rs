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

use air_interpreter_data::ExecutionTrace;
use air_test_utils::prelude::*;

use pretty_assertions::assert_eq;

#[test]
fn check_that_scalar_is_visible_only_inside_fold_block() {
    let variable_setter_peer_id = "variable_setter_peer_id";
    let mut variable_setter_vm = create_avm(set_variable_call_service(json!([1, 2, 3])), variable_setter_peer_id);

    let fallible_peer_id = "fallible_peer_id";
    let mut fallible_peer_vm = create_avm(fallible_call_service("fail"), fallible_peer_id);

    let variable_receiver_peer_id = "variable_receiver_peer_id";
    let mut variable_receiver_peer_vm = create_avm(echo_call_service(), variable_receiver_peer_id);

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

#[test]
fn scopes_check_that_scalar_not_overwritten_by_fold_end() {
    let variable_setter_peer_id = "variable_setter_peer_id";
    let mut variable_setter_vm = create_avm(set_variable_call_service(json!([1, 2, 3])), variable_setter_peer_id);

    let fallible_peer_id = "fallible_peer_id";
    let mut fallible_peer_vm = create_avm(fallible_call_service("fail"), fallible_peer_id);

    let variable_receiver_peer_id = "variable_receiver_peer_id";
    let mut variable_receiver_peer_vm = create_avm(echo_call_service(), variable_receiver_peer_id);

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
