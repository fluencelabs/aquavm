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

use air_test_framework::TestExecutor;
use air_test_utils::prelude::*;

#[test]
fn merging_fold_iterations_extensively() {
    let script = r#"
        (seq
            (seq
                (call "client" ("get" "data") [] permutations) ; ok = [["p1",[[["p1",1],["p2",2],["p3",3]],[["p1",4],["p3",5],["p2",6]]]],["p2",[[["p2",7],["p1",8],["p3",9]],[["p2",10],["p3",11],["p1",12]]]],["p3",[[["p3",13],["p1",14],["p2",15]],[["p3",16],["p2",17],["p1",18]]]]]
                (seq
                    (fold permutations pair
                        (seq
                            (fold pair.$.[1] peer_ids
                                (seq
                                    (seq
                                        (call pair.$.[0] ("op" "noop") []) ; ok = null
                                        (ap peer_ids $inner)
                                    )
                                    (next peer_ids)
                                )
                            )
                            (next pair)
                        )
                    )
                    (seq
                        (canon "relay" $inner #inner)
                        (fold $inner ns
                            (par
                                (fold ns pair
                                    (seq
                                        (seq
                                            (call pair.$.[0] ("op" "noop") []) ; ok = null
                                            (ap pair.$.[1] $result)
                                        )
                                        (next pair)
                                    )
                                )
                                (next ns)
                            )
                        )
                    )
                )
            )
            (seq
                (new $monotonic_stream
                    (seq
                        (fold $result elem
                            (seq
                                (ap elem $monotonic_stream)
                                (seq
                                    (canon "relay" $monotonic_stream #canon_stream)
                                    (xor
                                        (match #canon_stream.length 18
                                            (null)
                                        )
                                        (next elem)
                                    )
                                )
                            )
                        )
                        (canon "relay" $result #joined_result)
                    )
                )
                (call "client" ("return" "") [#inner #joined_result])  ; ok = null
            )
        )
        "#;

    let engine = TestExecutor::new(
        TestRunParameters::from_init_peer_id("client"),
        vec![],
        vec!["relay", "p1", "p2", "p3"].into_iter().map(Into::into),
        &script,
    ).unwrap();


    let mut queue = std::collections::vec_deque::VecDeque::new();
    let mut relay_outcomes = Vec::<RawAVMOutcome>::new();
    queue.push_back("client".to_string());
    while !queue.is_empty() {
        let peer = queue.pop_front().unwrap();
        if let Some(outcomes) = engine.execution_iter(peer.as_str()) {
            for outcome in outcomes {
                assert_eq!(outcome.ret_code, 0, "{:?}", outcome);

                for peer in &outcome.next_peer_pks {
                    queue.push_back(peer.clone());
                }

                if peer == "relay" {
                    relay_outcomes.push(outcome);
                }
            }
        } else {
            println!("peer: {}, no executions", peer);
        }
    }

    let last_relay_data = relay_outcomes.last().unwrap();
    let last_relay_trace = trace_from_result(last_relay_data);
    let last_fold = last_relay_trace
        .iter()
        .filter_map(|state| {
            match state {
                ExecutedState::Fold(fold_result) => Some(fold_result),
                _ => None
            }
        })
        .last()
        .unwrap();

    assert_eq!(last_fold.lore.len(), 18);
}
