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

use air::no_error_object;
use air::ExecutionCidState;
use air_test_framework::AirScriptExecutor;
use air_test_utils::prelude::*;

#[test]
fn fail_with_rebubble_error() {
    let peer_id = "peer_id";
    let script = r#"
    (seq
        (xor
            (xor
                (match 1 2 (null) )
                (fail :error:)
            )
            (call "peer_id" ("m" "f1") [:error:] scalar1) ; behaviour = echo
        )
        (call "peer_id" ("m" "f2") [:error:] scalar2) ; behaviour = echo
    )
    "#
    .to_string();

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(peer_id), &script)
        .expect("invalid test AIR script");
    let result = executor.execute_all(peer_id).unwrap();
    let actual_trace = trace_from_result(&result.last().unwrap());

    let mut cid_tracker: ExecutionCidState = ExecutionCidState::new();
    let expected_error_json = {
        json!({
          "error_code": 10006,
          "instruction": "xor",
          "message": "fail with '{\"error_code\":10001,\"instruction\":\"match 1 2\",\"message\":\"compared values do not match\"}' is used without corresponding xor"
        })
    };

    let expected_trace: Vec<ExecutedState> = vec![
        scalar_tracked!(
            expected_error_json.clone(),
            cid_tracker,
            peer_name = peer_id,
            service = "m..0",
            function = "f1",
            args = [expected_error_json]
        ),
        scalar_tracked!(
            no_error_object(),
            cid_tracker,
            peer_name = peer_id,
            service = "m..1",
            function = "f2",
            args = [no_error_object()]
        ),
    ];

    assert_eq!(actual_trace, expected_trace,);
}
