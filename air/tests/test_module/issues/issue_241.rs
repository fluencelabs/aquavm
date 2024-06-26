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
// test for github.com/fluencelabs/aquavm/issues/241
async fn issue_241() {
    let peer_1_id = "peer_1_id";
    let array_1_content = json!(["1", "2"]);
    let mut peer_1_vm = create_avm(set_variable_call_service(array_1_content.clone()), peer_1_id).await;

    let some_peer_id = "some_peer_id";
    let mut some_peer_vm = create_avm(unit_call_service(), some_peer_id).await;

    let set_array_0_peer_id = "set_array_0_peer_id";
    let peer_2_id = "peer_2_id";
    let peers = json!([peer_1_id, peer_2_id]);
    let mut set_array_0_vm = create_avm(set_variable_call_service(peers.clone()), set_array_0_peer_id).await;

    let script = format!(
        r#"
        (seq
            (call "{set_array_0_peer_id}" ("" "") [] array-0)
            (fold array-0 array-0-iterator
                (par
                    (call array-0-iterator ("" "") [] array-1)
                    (seq
                        (fold array-1 array-1-iterator
                            (seq
                                (call "{some_peer_id}" ("" "") [])
                                (next array-1-iterator)
                            )
                        )
                        (next array-0-iterator)
                    )
                )
            )
        )
    "#
    );

    let result = checked_call_vm!(set_array_0_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(peer_1_vm, <_>::default(), &script, "", result.data);
    let result = checked_call_vm!(some_peer_vm, <_>::default(), &script, "", result.data);
    let actual_trace = trace_from_result(&result);

    let expected_trace = ExecutionTrace::from(vec![
        scalar!(peers, peer = set_array_0_peer_id),
        executed_state::par(1, 4),
        scalar!(array_1_content, peer = peer_1_id),
        unused!("result from unit_call_service", peer = some_peer_id),
        unused!("result from unit_call_service", peer = some_peer_id),
        executed_state::par(1, 0),
        // before 0.22.0 scalar!s wasn't clear after end of a fold block and here was more states
        // from the second iteration of fold over array-1
        executed_state::request_sent_by(some_peer_id),
    ]);
    assert_eq!(actual_trace, expected_trace);
}
