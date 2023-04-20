/*
 * Copyright 2023 Fluence Labs Limited
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

use air_test_framework::AirScriptExecutor;
use air_test_utils::prelude::*;

#[test]
fn lambda_result_iteration() {
    let init_peer_id = "A";

    let air_script = r#"
        (seq
            (seq
                (call "A" ("service" "func") [] x) ; ok = [{"field": [10, 11, 12]}]
                (fold x y
                    (seq
                        (fold y.$.field a
                        (seq
                            (ap a $s)
                            (next a)))
                        (next y))))
            (canon "A" $s #c))
    "#;
    let runner = AirScriptExecutor::simple(TestRunParameters::from_init_peer_id(init_peer_id), air_script).unwrap();

    let result = runner.execute_one(init_peer_id).unwrap();
    assert_eq!(result.ret_code, 0, "{:?}", result.error_message);

    let data = data_from_result(&result);
    let last_state = data.trace.last().unwrap();

    let val = scalar!(
        json!([{"field": [10, 11, 12]}]),
        peer = init_peer_id,
        service = "service..0",
        function = "func"
    );
    let val_cid = extract_service_result_cid(&val);

    let expected_state = canon(json!({
        "tetraplet": {
            "peer_pk": init_peer_id,
            "service_id": "",
            "function_name": "",
            "json_path": "",
        },
        "values": [{
            "result": 10,
            "tetraplet": {
                "peer_pk": init_peer_id,
                "service_id": "service..0",
                "function_name": "func",
                "json_path": ".$.[0].$.field.$.[0]",
            },
            "provenance": Provenance::service_result(val_cid.clone(), None),
        }, {
            "result": 11,
            "tetraplet": {
                "peer_pk": init_peer_id,
                "service_id": "service..0",
                "function_name": "func",
                "json_path": ".$.[0].$.field.$.[1]",
            },
            "provenance": Provenance::service_result(val_cid.clone(), None),
        }, {
            "result": 12,
            "tetraplet": {
                "peer_pk": init_peer_id,
                "service_id": "service..0",
                "function_name": "func",
                "json_path": ".$.[0].$.field.$.[2]",
            },
            "provenance": Provenance::service_result(val_cid, None),
        }]
    }));

    assert_eq!(last_state, &expected_state);
}
