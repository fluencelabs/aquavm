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
// https://github.com/fluencelabs/aquavm/issues/178
fn par_ap_behaviour() {
    let client_id = "client_id";
    let relay_id = "relay_id";
    let variable_setter_id = "variable_setter_id";

    // ap doesn't affect the subgraph_complete flag
    let script = f!(r#"
        (par
            (call "{variable_setter_id}" ("peer" "timeout") [] join_it) ; behaviour=unit
            (seq
                (par
                    (call "{relay_id}" ("peer" "timeout") [join_it] $result) ; behaviour=unit
                    (ap "fast_result" $result)
                )
                (seq
                    (canon "{client_id}" $result #result)
                    (call "{client_id}" ("op" "return") [#result.$[0]]) ; behaviour=unit
                )
            )
        )
        "#);

    let engine = air_test_framework::AirScriptExecutor::simple(TestRunParameters::new("client_id", 0, 1, ""), &script)
        .expect("invalid test executor config");

    let client_result_1 = engine.execute_one(client_id).unwrap();
    assert_next_pks!(&client_result_1.next_peer_pks, [relay_id, variable_setter_id]);

    let setter_result = engine.execute_one(variable_setter_id).unwrap();
    assert!(setter_result.next_peer_pks.is_empty());

    let relay_result = engine.execute_one(relay_id).unwrap();
    assert!(relay_result.next_peer_pks.is_empty());
}
