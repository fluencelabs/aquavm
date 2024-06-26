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
// test for github.com/fluencelabs/aquavm/issues/216
async fn issue_216() {
    let some_peer_id = "relay_peer_id";
    let variables_mapping = maplit::hashmap! {
        "value".to_string() => json!([]),
    };

    let mut some_peer = create_avm(
        set_variables_call_service(variables_mapping, VariableOptionSource::FunctionName),
        some_peer_id,
    )
    .await;

    let client_id = "client_peer_id";
    let mut client = create_avm(echo_call_service(), client_id).await;

    let error_message = "error message";
    let script = format!(
        r#"
        (xor
            (seq
                (call "{some_peer_id}" ("" "value") [] value)
                (ap value.$.non_exist_field $stream) ;; (1)
            )
            (call %init_peer_id% ("" "") ["{error_message}"]) ;; (2)
        )
    "#
    );

    let test_params = TestRunParameters::from_init_peer_id(client_id);
    let result = checked_call_vm!(some_peer, test_params.clone(), &script, "", "");
    let result = checked_call_vm!(client, test_params, &script, "", result.data); // before 0.20.4 it's just failed
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        scalar!(json!([]), peer = some_peer_id, function = "value"),
        unused!(error_message, peer = client_id, args = vec![error_message]),
    ];
    assert_eq!(actual_trace, expected_trace);
}
