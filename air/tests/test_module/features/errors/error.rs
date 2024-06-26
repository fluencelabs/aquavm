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

use air::no_error_object;
use air::ExecutionCidState;
use air_test_framework::AirScriptExecutor;
use air_test_utils::prelude::*;

#[tokio::test]
async fn fail_with_rebubble_error() {
    let peer_id = "peer_id";
    let script = r#"
    (seq
        (xor
            (xor
                (match 1 2 (null) )
                (fail :error:)
            )
            (call "peer_id" ("m" "f1") [:error:] scalar1) ; dbg_behaviour = echo
        )
        (call "peer_id" ("m" "f2") [:error:] scalar2) ; dbg_behaviour = echo
    )
    "#
    .to_string();

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(peer_id), &script)
        .await
        .expect("invalid test AIR script");
    let result = executor.execute_all(peer_id).await.unwrap();
    let actual_trace = trace_from_result(&result.last().unwrap());

    let mut cid_tracker: ExecutionCidState = ExecutionCidState::new();
    let expected_error_json = {
        json!({
          "error_code": 10001,
          "instruction": "match 1 2",
          "message": "compared values do not match"
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

#[tokio::test]
async fn rebubble_error_from_xor_right_branch() {
    let peer_id = "peer_id";
    let script = r#"
    (seq
        (xor
            (xor
                (xor
                    (match 1 2 (null) )
                    (fail :error:)
                )
                (seq
                    (call "peer_id" ("m" "f1") [:error:] scalar1) ; behaviour = echo
                    (match 3 2 (null) )
                )
            )
            (call "peer_id" ("m" "f2") [:error:] scalar2) ; behaviour = echo
        )
        (call "peer_id" ("m" "f3") [:error:] scalar3) ; behaviour = echo
    )
    "#
    .to_string();

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(peer_id), &script)
        .await
        .expect("invalid test AIR script");
    let result = executor.execute_all(peer_id).await.unwrap();
    let actual_trace = trace_from_result(&result.last().unwrap());

    let mut cid_tracker: ExecutionCidState = ExecutionCidState::new();
    let inner_expected_error_json = {
        json!({
          "error_code": 10001,
          "instruction": "match 1 2",
          "message": "compared values do not match"
        })
    };
    let outer_expected_error_json = {
        json!({
          "error_code": 10001,
          "instruction": "match 3 2",
          "message": "compared values do not match"
        })
    };

    let expected_trace: Vec<ExecutedState> = vec![
        scalar_tracked!(
            inner_expected_error_json.clone(),
            cid_tracker,
            peer_name = peer_id,
            service = "m..0",
            function = "f1",
            args = [inner_expected_error_json]
        ),
        scalar_tracked!(
            outer_expected_error_json.clone(),
            cid_tracker,
            peer_name = peer_id,
            service = "m..1",
            function = "f2",
            args = [outer_expected_error_json]
        ),
        scalar_tracked!(
            no_error_object(),
            cid_tracker,
            peer_name = peer_id,
            service = "m..2",
            function = "f3",
            args = [no_error_object()]
        ),
    ];

    print_trace(&result.last().unwrap(), "");

    assert_eq!(actual_trace, expected_trace);
}
