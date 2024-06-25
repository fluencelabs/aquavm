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
#[ignore] // this test is not actual because streams are prohibited to be used as match operands
          // test for github.com/fluencelabs/aquavm/issues/206
async fn issue_206() {
    let peer_1_id = "peer_1_id";
    let mut peer_1 = create_avm(echo_call_service(), peer_1_id).await;

    let script = format!(
        r#"
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
    "#
    );

    let test_params = TestRunParameters::from_init_peer_id(peer_1_id);
    let result = checked_call_vm!(peer_1, test_params, &script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![executed_state::ap(0), scalar!(json!(["is nil"]))];
    assert_eq!(actual_trace, expected_trace);
}
