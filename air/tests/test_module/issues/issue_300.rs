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

use air_test_utils::prelude::*;

#[tokio::test]
// test for github.com/fluencelabs/aquavm/issues/300
async fn issue_300() {
    let peer_id_1 = "peer_id_1";
    let mut peer_vm_1 = create_avm(echo_call_service(), peer_id_1).await;
    let peer_id_2 = "peer_id_2";
    let mut peer_vm_2 = create_avm(echo_call_service(), peer_id_2).await;

    let script = format!(
        r#"
        (new $stream
            (par
                (call "{peer_id_1}" ("" "") [2] $stream)
                (call "{peer_id_2}" ("" "") [1] $stream)
            )
        )
    "#
    );

    let result_1 = checked_call_vm!(peer_vm_2, <_>::default(), &script, "", "");
    let result_2 = checked_call_vm!(peer_vm_1, <_>::default(), &script, "", result_1.data);
    let actual_trace = trace_from_result(&result_2);

    let expected_trace = vec![
        executed_state::par(1, 1),
        stream!(2, 1, peer = peer_id_1, args = vec![2]),
        stream!(1, 0, peer = peer_id_2, args = vec![1]),
    ];
    assert_eq!(actual_trace, expected_trace);
}
