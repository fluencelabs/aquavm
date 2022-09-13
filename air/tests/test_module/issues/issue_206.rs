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
// test for github.com/fluencelabs/aquavm/issues/206
fn issue_206() {
    let peer_1_id = "peer_1_id";
    let mut peer_1 = create_avm(echo_call_service(), peer_1_id);

    let script = f!(r#"
    (new $result
        (seq
            (xor
                (match $result []
                    (xor
                        (ap "is nil" $result)
                        (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 1])
                    )
                )
                (ap "not" $result)
            )
            (call %init_peer_id% ("op" "identity") [$result] result-fix)
        )
    )
    "#);

    let test_params = TestRunParameters::from_init_peer_id(peer_1_id);
    let result = checked_call_vm!(peer_1, test_params, &script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![executed_state::ap_stream(0), executed_state::scalar(json!(["is nil"]))];
    assert_eq!(actual_trace, expected_trace);
}
