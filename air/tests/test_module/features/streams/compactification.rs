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

use air::ExecutionCidState;
use air_test_framework::AirScriptExecutor;
use air_test_utils::prelude::TestRunParameters;
use air_test_utils::*;

#[tokio::test]
async fn global_streams_are_compactified() {
    let peer_name = "peer_id";
    let service_result = "service_result";
    let script = format!(
        r#"
        (seq
            (ap 1 $stream)
            (call "{peer_name}" ("" "") [] $stream) ; ok = "{service_result}"
        )
    "#
    );

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(peer_name), &script)
        .await
        .unwrap();
    let result = executor.execute_all(peer_name).await.unwrap();
    let actual_trace = trace_from_result(result.last().unwrap());

    let mut cid_state = ExecutionCidState::new();
    let expected_trace = vec![
        executed_state::ap(0),
        stream_tracked!(
            service_result,
            1,
            cid_state,
            peer_name = peer_name,
            service = "..0",
            function = ""
        ),
    ];

    assert_eq!(&actual_trace, &expected_trace);
}

#[tokio::test]
async fn global_stream_maps_are_compactified() {
    let peer_name = "peer_id";
    let service_result = "service_result";
    let script = format!(
        r#"
        (seq
            (ap (1 1) %stream_map)
            (seq
                (call "{peer_name}" ("" "") [] $stream) ; ok = "{service_result}"
                (ap (1 1) %stream_map)
            )
        )
    "#
    );

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(peer_name), &script)
        .await
        .unwrap();
    let result = executor.execute_all(peer_name).await.unwrap();
    let actual_trace = trace_from_result(result.last().unwrap());

    let mut cid_state = ExecutionCidState::new();
    let expected_trace = vec![
        executed_state::ap(0),
        stream_tracked!(
            service_result,
            0,
            cid_state,
            peer_name = peer_name,
            service = "..0",
            function = ""
        ),
        executed_state::ap(1),
    ];

    assert_eq!(&actual_trace, &expected_trace);
}

#[tokio::test]
async fn local_streams_are_compactified() {
    let peer_name = "peer_id";
    let service_result = "service_result";
    let script = format!(
        r#"
        (new $stream
            (seq
                (ap 1 $stream)
                (call "{peer_name}" ("" "") [] $stream) ; ok = "{service_result}"
            )
        )
    "#
    );

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(peer_name), &script)
        .await
        .unwrap();
    let result = executor.execute_all(peer_name).await.unwrap();
    let actual_trace = trace_from_result(result.last().unwrap());

    let mut cid_state = ExecutionCidState::new();
    let expected_trace = vec![
        executed_state::ap(0),
        stream_tracked!(
            service_result,
            1,
            cid_state,
            peer_name = peer_name,
            service = "..0",
            function = ""
        ),
    ];

    assert_eq!(actual_trace, expected_trace);
}

#[tokio::test]
async fn local_stream_maps_are_compactified() {
    let peer_name = "peer_id";
    let service_result = "service_result";
    let script = format!(
        r#"
        (new $stream
            (seq
                (ap (1 1) %stream_map)
                (seq
                    (call "{peer_name}" ("" "") [] $stream) ; ok = "{service_result}"
                    (ap (1 1) %stream_map)
                )
            )
        )
    "#
    );

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(peer_name), &script)
        .await
        .unwrap();
    let result = executor.execute_all(peer_name).await.unwrap();
    let actual_trace = trace_from_result(result.last().unwrap());

    let mut cid_state = ExecutionCidState::new();
    let expected_trace = vec![
        executed_state::ap(0),
        stream_tracked!(
            service_result,
            0,
            cid_state,
            peer_name = peer_name,
            service = "..0",
            function = ""
        ),
        executed_state::ap(1),
    ];

    assert_eq!(actual_trace, expected_trace);
}
