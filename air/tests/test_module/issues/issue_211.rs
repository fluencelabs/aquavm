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

use air_interpreter_data::ExecutionTrace;
use air_test_utils::prelude::*;

use pretty_assertions::assert_eq;

#[tokio::test]
// test for github.com/fluencelabs/aquavm/issues/211
// On the versions < 0.20.1 it just crashes
async fn issue_211() {
    let peer_1_name = "peer_1_id";

    let script = format!(
        r#"
     (xor
        (seq
          (seq
             (seq
                (seq
                   (null)
                   (call %init_peer_id% ("getdatasrv" "idx") [] idx) ; ok=2
                )
                (call %init_peer_id% ("getdatasrv" "nodes") [] nodes) ; ok = [1,2,3]
             )
             (new $nodes2
                (seq
                   (seq
                      (par
                         (fold nodes node
                            (par
                               (ap node $nodes2)
                               (next node)
                            )
                         )
                         (null)
                      )
                      (seq
                         (canon %init_peer_id% $nodes2 #nodes2_0)
                         (call %init_peer_id% ("op" "noop") [#nodes2_0.$.[idx]! nodes]) ; ok="expected result"
                      )
                   )
                   (seq
                      (canon %init_peer_id% $nodes2 #nodes2_1)
                      (call %init_peer_id% ("op" "identity") [#nodes2_1] nodes2-fix) ; ok="expected result"
                   )
                )
             )
          )
          (null)
        )
        (call %init_peer_id% ("errorhandlingsrv" "error") [%last_error% 2]) ; ok="error"
     )
    "#
    );

    let run_params = TestRunParameters::from_init_peer_id(peer_1_name);

    let engine = air_test_framework::AirScriptExecutor::from_annotated(run_params, &script)
        .await
        .expect("invalid test executor config");
    let peer_1_id = engine.resolve_name(peer_1_name).to_string();
    let peer_1_id = peer_1_id.as_str();

    let result = engine.execute_one(peer_1_name).await.unwrap();

    let scalar_2 = scalar!(
        json!([1, 2, 3]),
        peer_name = peer_1_name,
        service = "getdatasrv..1",
        function = "nodes"
    );
    let cid_2 = extract_service_result_cid(&scalar_2);

    let expected_trace = ExecutionTrace::from(vec![
        scalar!(2, peer_name = peer_1_name, service = "getdatasrv..0", function = "idx"),
        scalar_2,
        executed_state::par(6, 0),
        executed_state::par(1, 4),
        executed_state::ap(0),
        executed_state::par(1, 2),
        executed_state::ap(0),
        executed_state::par(1, 0),
        executed_state::ap(0),
        executed_state::canon(json!({
            "tetraplet": {"function_name": "", "lens": "", "peer_pk": peer_1_id, "service_id": ""},
            "values": [
                {
                    "result": 1,
                    "tetraplet": {"function_name": "nodes", "lens": ".$.[0]", "peer_pk": peer_1_id, "service_id": "getdatasrv..1"},
                    "provenance": Provenance::service_result(cid_2.clone()),
                },
                {
                    "result": 2,
                    "tetraplet": {"function_name": "nodes", "lens": ".$.[1]", "peer_pk": peer_1_id, "service_id": "getdatasrv..1"},
                    "provenance": Provenance::service_result(cid_2.clone()),
                },
                {
                    "result": 3,
                    "tetraplet": {"function_name": "nodes", "lens": ".$.[2]", "peer_pk": peer_1_id, "service_id": "getdatasrv..1"},
                    "provenance": Provenance::service_result(cid_2.clone()),
                },
            ]
        })),
        unused!(
            "expected result",
            peer_name = peer_1_name,
            service = "op..2",
            function = "noop",
            args = vec![json!(3), json!([1, 2, 3])]
        ),
        executed_state::canon(json!({
            "tetraplet": {"function_name": "", "lens": "", "peer_pk": peer_1_id, "service_id": ""},
            "values": [
                {
                    "result": 1,
                    "tetraplet": {"function_name": "nodes", "lens": ".$.[0]", "peer_pk": peer_1_id, "service_id": "getdatasrv..1"},
                    "provenance": Provenance::service_result(cid_2.clone()),
                },
                {
                    "result": 2,
                    "tetraplet": {"function_name": "nodes", "lens": ".$.[1]", "peer_pk": peer_1_id, "service_id": "getdatasrv..1"},
                    "provenance": Provenance::service_result(cid_2.clone()),
                },
                {
                    "result": 3,
                    "tetraplet": {"function_name": "nodes", "lens": ".$.[2]", "peer_pk": peer_1_id, "service_id": "getdatasrv..1"},
                    "provenance": Provenance::service_result(cid_2),
                },
            ]
        })),
        scalar!(
            "expected result",
            peer_name = peer_1_name,
            service = "op..3",
            function = "identity",
            args = vec![json!([1, 2, 3])]
        ),
    ]);

    let actual_data = data_from_result(&result);
    assert_eq!(actual_data.trace, expected_trace, "{:?}", actual_data.cid_info);
}
