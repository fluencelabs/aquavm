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
// test for github.com/fluencelabs/aquavm/issues/180
async fn issue_180() {
    let peer_1_id = "peer_1_id";
    let peer_2_id = "peer_2_id";
    let mut peer_1 = create_avm(unit_call_service(), peer_1_id).await;

    let script = format!(
        r#"
        (par
            (call "{peer_2_id}" ("" "") [] join_var)
            (seq
                (par
                    (call "{peer_1_id}" ("" "") [join_var]) ;; sets subgraph_complete to false
                    (fold join_var iterator ;; (on < 0.17.3) triggers ValueNotFound exception and doesn't touch subgraph_complete flag
                        (null)
                    )
                )
                (call "{peer_1_id}" ("" "") []) ;; this should be called only when join_var is set
            )
        )
        "#
    );

    let peer_1_result = checked_call_vm!(peer_1, <_>::default(), &script, "", "");
    let trace = trace_from_result(&peer_1_result);
    assert_eq!(trace.len(), 3);
}
