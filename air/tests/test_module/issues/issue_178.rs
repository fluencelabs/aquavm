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
// https://github.com/fluencelabs/aquavm/issues/178
async fn par_ap_behaviour() {
    let client_name = "client_id";
    let relay_name = "relay_id";
    let variable_setter_name = "variable_setter_id";

    // ap doesn't affect the subgraph_complete flag
    let script = format!(
        r#"
        (par
            (call "{variable_setter_name}" ("peer" "timeout") [] join_it) ; behaviour=unit
            (seq
                (par
                    (call "{relay_name}" ("peer" "timeout") [join_it] $result) ; behaviour=unit
                    (ap "fast_result" $result)
                )
                (seq
                    (canon "{client_name}" $result #result)
                    (call "{client_name}" ("op" "return") [#result.$[0]]) ; behaviour=unit
                )
            )
        )
        "#
    );

    let engine =
        air_test_framework::AirScriptExecutor::from_annotated(TestRunParameters::new(client_name, 0, 1, ""), &script)
            .await
            .expect("invalid test executor config");

    let relay_id = engine.resolve_name(relay_name).to_string();
    let variable_setter_id = engine.resolve_name(variable_setter_name).to_string();

    let client_result_1 = engine.execute_one(client_name).await.unwrap();
    assert_next_pks!(
        &client_result_1.next_peer_pks,
        &[relay_id.as_str(), variable_setter_id.as_str()]
    );

    let setter_result = engine.execute_one(variable_setter_name).await.unwrap();
    assert!(setter_result.next_peer_pks.is_empty());

    let relay_result = engine.execute_one(relay_name).await.unwrap();
    assert!(relay_result.next_peer_pks.is_empty());
}
