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

use air_interpreter_data::ExecutionTrace;
use air_test_utils::prelude::*;

use pretty_assertions::assert_eq;

#[tokio::test]
// test for github.com/fluencelabs/aquavm/issues/214
async fn issue_214() {
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
    )
    .await;

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
