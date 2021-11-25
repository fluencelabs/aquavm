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

#[test]
// https://github.com/fluencelabs/aquavm/issues/137
fn issue_137() {
    let initiator_id = "initiator_id";
    let mut initiator = create_avm(unit_call_service(), initiator_id);
    let node_1_id = "node_1_id";
    let mut node_1 = create_avm(unit_call_service(), node_1_id);
    let node_2_id = "node_2_id";
    let mut node_2 = create_avm(unit_call_service(), node_2_id);
    let node_3_id = "node_3_id";
    let node_4_id = "node_4_id";
    let mut node_4 = create_avm(unit_call_service(), node_4_id);

    let script = format!(
        r#"
        (seq
            (call "{0}" ("" "") []) ;; initiator
            (par
                (seq
                    (par
                        (call "{1}" ("" "") []) ;; node 1
                        (call "{2}" ("" "") []) ;; node 2
                    )
                    (call "{3}" ("" "") []) ;; node 3
                )
                (par
                    (seq
                        (call "{1}" ("" "") []) ;; node 1
                        (call "{4}" ("" "") []) ;; node 4
                    )
                    (seq
                        (call "{2}" ("" "") []) ;; node 2
                        (call "{4}" ("" "") []) ;; node 4
                    )
                )
            )
        )
        "#,
        initiator_id, node_1_id, node_2_id, node_3_id, node_4_id
    );

    let initiator_result = checked_call_vm!(initiator, "", &script, "", "");
    let node_1_result = checked_call_vm!(node_1, "", &script, "", initiator_result.data.clone());
    let node_2_result = checked_call_vm!(node_2, "", &script, "", initiator_result.data);
    let node_4_result_1 = checked_call_vm!(node_4, "", &script, "", node_1_result.data);
    let result = call_vm!(node_4, "", script, node_4_result_1.data, node_2_result.data);
    assert_eq!(result.ret_code, 0);
}
