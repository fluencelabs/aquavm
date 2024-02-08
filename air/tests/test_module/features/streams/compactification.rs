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

use air::ExecutionCidState;
use air_test_framework::AirScriptExecutor;
use air_test_utils::prelude::TestRunParameters;
use air_test_utils::*;

#[tokio::test]
fn global_streams_are_compactified() {
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

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(peer_name), &script).unwrap();
    let result = executor.execute_all(peer_name).unwrap();
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
fn global_stream_maps_are_compactified() {
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

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(peer_name), &script).unwrap();
    let result = executor.execute_all(peer_name).unwrap();
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
fn local_streams_are_compactified() {
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

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(peer_name), &script).unwrap();
    let result = executor.execute_all(peer_name).unwrap();
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
fn local_stream_maps_are_compactified() {
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

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(peer_name), &script).unwrap();
    let result = executor.execute_all(peer_name).unwrap();
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
