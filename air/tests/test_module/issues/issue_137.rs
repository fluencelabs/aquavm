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
// https://github.com/fluencelabs/aquavm/issues/137
async fn issue_137() {
    let initiator_id = "initiator_id";
    let mut initiator = create_avm(unit_call_service(), initiator_id).await;
    let node_1_id = "node_1_id";
    let mut node_1 = create_avm(unit_call_service(), node_1_id).await;
    let node_2_id = "node_2_id";
    let mut node_2 = create_avm(unit_call_service(), node_2_id).await;
    let node_3_id = "node_3_id";
    let node_4_id = "node_4_id";
    let mut node_4 = create_avm(unit_call_service(), node_4_id).await;

    let script = format!(
        r#"
        (seq
            (call "{initiator_id}" ("" "") []) ;; initiator
            (par
                (seq
                    (par
                        (call "{node_1_id}" ("" "") []) ;; node 1
                        (call "{node_2_id}" ("" "") []) ;; node 2
                    )
                    (call "{node_3_id}" ("" "") []) ;; node 3
                )
                (par
                    (seq
                        (call "{node_1_id}" ("" "") []) ;; node 1
                        (call "{node_4_id}" ("" "") []) ;; node 4
                    )
                    (seq
                        (call "{node_2_id}" ("" "") []) ;; node 2
                        (call "{node_4_id}" ("" "") []) ;; node 4
                    )
                )
            )
        )
        "#
    );

    let initiator_result = checked_call_vm!(initiator, <_>::default(), &script, "", "");
    let node_1_result = checked_call_vm!(node_1, <_>::default(), &script, "", initiator_result.data.clone());
    let node_2_result = checked_call_vm!(node_2, <_>::default(), &script, "", initiator_result.data);
    let node_4_result_1 = checked_call_vm!(node_4, <_>::default(), &script, "", node_1_result.data);
    let result = call_vm!(node_4, <_>::default(), script, node_4_result_1.data, node_2_result.data);

    assert!(is_interpreter_succeded(&result));
}
