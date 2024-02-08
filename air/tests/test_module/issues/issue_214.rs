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

use air_interpreter_data::ExecutionTrace;
use air_test_utils::prelude::*;

use pretty_assertions::assert_eq;

#[tokio::test]
// test for github.com/fluencelabs/aquavm/issues/214
fn issue_214() {
    let client_id = "client_peer_id";
    let relay_id = "relay_peer_id";
    let scalar = json!([]);
    let error_handler = "error handler is called";
    let variables_mapping = maplit::hashmap! {
        "-relay-".to_string() => json!(relay_id),
        "s".to_string() => scalar.clone(),
        "error".to_string() => json!(error_handler), // this result should be returned by (2) call
    };

    let mut client = create_avm(
        set_variables_call_service(variables_mapping, VariableOptionSource::FunctionName),
        client_id,
    );

    let script = format!(
        r#"
        (xor
         (seq
          (seq
           (seq
            (call %init_peer_id% ("getDataSrv" "-relay-") [] -relay-)
            (call %init_peer_id% ("getDataSrv" "s") [] s)
           )
           (xor
            (call -relay- ("op" "identity") [s.$.field!] res) ;; (1) should not produce data after calling on relay
            (call %init_peer_id% ("errorHandlingSrv" "error") ["%last_error%" 1]) ;; (2) should be called
           )
          )
          (xor
           (call %init_peer_id% ("callbackSrv" "response") [res]) ;; join behaviour
           (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 2])
          )
         )
         (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 3])
        )
    "#
    );

    let test_params = TestRunParameters::from_init_peer_id(client_id);
    let result = checked_call_vm!(client, test_params, &script, "", "");
    let expected_trace = ExecutionTrace::from(vec![
        scalar!(relay_id, peer = client_id, service = "getDataSrv", function = "-relay-"),
        scalar!(scalar, peer = client_id, service = "getDataSrv", function = "s"),
        unused!(
            error_handler,
            peer = client_id,
            service = "errorHandlingSrv",
            function = "error",
            args = vec![json!("%last_error%"), json!(1)]
        ),
    ]);
    let actual_trace = trace_from_result(&result);

    assert_eq!(actual_trace, expected_trace);
}
