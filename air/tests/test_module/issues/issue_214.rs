/*
 * Copyright 2022 Fluence Labs Limited
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
// test for github.com/fluencelabs/aquavm/issues/214
fn issue_214() {
    let client_id = "client_peer_id";
    let relay_id = "relay_peer_id";
    let scalar = json!([]);
    let error_handler = "error handler is called";

    let script = f!(r#"
        (xor
         (seq
          (seq
           (seq
            (call %init_peer_id% ("getDataSrv" "-relay-") [] -relay-) ; result="relay_peer_id"
            (call %init_peer_id% ("getDataSrv" "s") [] s) ; result=[]
           )
           (xor
            (call -relay- ("op" "identity") [s.$.field] res)
            (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 1]) ; result="error handler is called"
           )
          )
          (xor
           (call %init_peer_id% ("callbackSrv" "response") [res]) ; result = "default"
           (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 2]) ; result="error handler is not called"
          )
         )
         (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 3]) ; result="error handler is called"
        )
    "#);

    let test_params = TestRunParameters::from_init_peer_id(client_id);
    let engine =
        air_test_framework::TestExecutor::simple(test_params, &script).expect("Invalid test executor configuration");

    let result = engine.execute_one(client_id).unwrap();
    let expected_trace = vec![
        executed_state::scalar_string(relay_id),
        executed_state::scalar(scalar),
        executed_state::scalar_string(error_handler),
    ];
    let actual_trace = trace_from_result(&result);

    assert_eq!(actual_trace, expected_trace);
}
