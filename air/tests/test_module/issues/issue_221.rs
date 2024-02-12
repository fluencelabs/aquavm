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
use air_test_framework::AirScriptExecutor;
use air_test_utils::key_utils::at;
use air_test_utils::prelude::*;
use pretty_assertions::assert_eq;

#[tokio::test]
// test for github.com/fluencelabs/aquavm/issues/221
async fn issue_221() {
    let peer_1_name = "peer_1_id";
    let peer_2_name = "peer_2_id";
    let join_1_name = "join_1_id";
    let join_2_name = "join_2_id";
    let set_variable_name = "set_variable_id";

    let peer_1_value = "peer_1_value";
    let peer_2_value = "peer_2_value";

    let script = format!(
        r#"
        (seq
            (seq
                (seq
                    ;; let's peers be an array of two values [peer_1_id, peer_2_id]
                    (call "{set_variable_name}" ("" "") [] peers) ; ok = [@"{peer_1_name}", @"{peer_2_name}"]
                    (fold peers peer
                        (par
                            (seq
                                (call peer ("" "") [peer] value) ; map = {{@"{peer_1_name}": "{peer_1_value}", @"{peer_2_name}": "{peer_2_value}"}}
                                ;; it's crucial to reproduce this bug to add value to stream
                                ;; with help of ap instruction
                                (ap value $stream)
                            )
                            (next peer)
                        )
                    )
                )
                ;; join streams on join_1/join_2 peers in such a way that will have different state:
                ;; join_1 $stream: [peer_1_value, peer_2_value]
                ;; join_2 $stream: [peer_2_value, peer_1_value]
                (fold $stream iterator
                    ;; here it'll encounter a bug in trace handler, because fold won't shuffle lores in
                    ;; appropriate way and state for (1) is returned
                    (par
                        (par
                            (call "{join_1_name}" ("" "") [iterator]) ; behaviour = echo
                            (call "{join_2_name}" ("" "") [iterator]) ; behaviour = echo
                        )
                        (next iterator)
                    )
                )
            )
            (call "some_peer_name" ("" "") []) ;; (1)
        )
    "#
    );

    let executor = <AirScriptExecutor>::new(
        TestRunParameters::from_init_peer_id("set_variable_id"),
        vec![],
        vec![peer_1_name, peer_2_name].into_iter().map(Into::into),
        &script,
    )
    .await
    .expect("Invalid annotated AIR script");

    let peer_1_id = at(peer_1_name);
    let peer_2_id = at(peer_2_name);
    let join_1_id = at(join_1_name);

    let _result = executor.execute_one(set_variable_name).await.unwrap();
    let _peer_1_result = executor.execute_one(peer_1_name).await.unwrap();
    let _peer_2_result = executor.execute_one(peer_2_name).await.unwrap();

    let _join_1_result = executor.execute_one(join_1_name).await.unwrap();
    let join_1_result = executor.execute_one(join_1_name).await.unwrap(); // before 0.20.9 it fails here
    let actual_trace = trace_from_result(&join_1_result);
    let expected_trace = ExecutionTrace::from(vec![
        scalar!(
            json!([peer_1_id, peer_2_id]),
            peer_name = set_variable_name,
            service = "..0"
        ),
        executed_state::par(2, 3),
        scalar!(
            peer_1_value,
            peer_name = peer_1_name,
            service = "..1",
            args = vec![peer_1_id.as_str()]
        ),
        executed_state::ap(0),
        executed_state::par(2, 0),
        scalar!(
            peer_2_value,
            peer_name = peer_2_name,
            service = "..1",
            args = vec![peer_2_id.as_str()]
        ),
        executed_state::ap(1),
        executed_state::fold(vec![
            executed_state::subtrace_lore(3, SubTraceDesc::new(8.into(), 4), SubTraceDesc::new(12.into(), 0)),
            executed_state::subtrace_lore(6, SubTraceDesc::new(12.into(), 4), SubTraceDesc::new(16.into(), 0)),
        ]),
        executed_state::par(3, 0),
        executed_state::par(1, 1),
        unused!(
            peer_1_value,
            peer_name = join_1_name,
            service = "..2",
            args = vec![peer_1_value]
        ),
        executed_state::request_sent_by(peer_1_id),
        executed_state::par(3, 0),
        executed_state::par(1, 1),
        unused!(
            peer_2_value,
            peer_name = join_1_name,
            service = "..2",
            args = vec![peer_2_value]
        ),
        executed_state::request_sent_by(peer_2_id),
        executed_state::request_sent_by(join_1_id),
    ]);

    assert_eq!(actual_trace, expected_trace);
}
