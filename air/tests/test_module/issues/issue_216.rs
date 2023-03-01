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
// test for github.com/fluencelabs/aquavm/issues/216
fn issue_216() {
    let some_peer_id = "relay_peer_id";
    let variables_mapping = maplit::hashmap! {
        "value".to_string() => json!([]),
    };

    let mut some_peer = create_avm(
        set_variables_call_service(variables_mapping, VariableOptionSource::FunctionName),
        some_peer_id,
    );

    let client_id = "client_peer_id";
    let mut client = create_avm(echo_call_service(), client_id);

    let error_message = "error message";
    let script = f!(r#"
        (xor
            (seq
                (call "{some_peer_id}" ("" "value") [] value)
                (ap value.$.non_exist_field $stream) ;; (1)
            )
            (call %init_peer_id% ("" "") ["{error_message}"]) ;; (2)
        )
    "#);

    let test_params = TestRunParameters::from_init_peer_id(client_id);
    let result = checked_call_vm!(some_peer, test_params.clone(), &script, "", "");
    let result = checked_call_vm!(client, test_params, &script, "", result.data); // before 0.20.4 it's just failed
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        scalar!((json!([])), peer = some_peer_id, function = "value"),
        scalar_unused!(error_message, peer = client_id, args = vec![error_message]),
    ];
    assert_eq!(actual_trace, expected_trace);
}
