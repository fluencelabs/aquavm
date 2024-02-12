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

use air_test_framework::AirScriptExecutor;
use air_test_utils::{key_utils::at, prelude::*};

use futures::stream::StreamExt;

#[tokio::test]
async fn merging_fold_iterations_extensively() {
    let script = r#"
        (seq
            (seq
                (call "client" ("get" "data") [] permutations) ; ok = [[@"p1",[[[@"p1",1],[@"p2",2],[@"p3",3]],[[@"p1",4],[@"p3",5],[@"p2",6]]]],[@"p2",[[[@"p2",7],[@"p1",8],[@"p3",9]],[[@"p2",10],[@"p3",11],[@"p1",12]]]],[@"p3",[[[@"p3",13],[@"p1",14],[@"p2",15]],[[@"p3",16],[@"p2",17],[@"p1",18]]]]]
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

    let engine = <AirScriptExecutor>::new(
        TestRunParameters::from_init_peer_id("client"),
        vec![],
        vec!["relay", "p1", "p2", "p3"].into_iter().map(Into::into),
        script,
    )
    .await
    .unwrap();

    let mut queue = std::collections::vec_deque::VecDeque::new();
    let mut relay_outcomes = Vec::<RawAVMOutcome>::new();
    queue.push_back("client".to_string());
    while !queue.is_empty() {
        let peer = queue.pop_front().unwrap();
        if let Some(outcomes) = engine.execution_iter(peer.as_str()) {
            for outcome in outcomes.collect::<Vec<_>>().await {
                assert_eq!(outcome.ret_code, 0, "{outcome:?}");

                for peer in &outcome.next_peer_pks {
                    queue.push_back(peer.clone());
                }

                if peer == at("relay") {
                    relay_outcomes.push(outcome);
                }
            }
        } else {
            println!("peer: {peer}, no executions");
        }
    }

    let last_relay_data = relay_outcomes.last().unwrap();
    let last_relay_trace = trace_from_result(last_relay_data);
    let last_fold = last_relay_trace
        .iter()
        .filter_map(|state| match state {
            ExecutedState::Fold(fold_result) => Some(fold_result),
            _ => None,
        })
        .last()
        .unwrap();

    assert_eq!(last_fold.lore.len(), 18);
}

#[tokio::test]
async fn merging_fold_iterations_extensively_2() {
    let script = r#"
        (seq
            (seq
                (call "client" ("get" "data") [] permutations) ; ok = [[@"p1",[[[@"p1",1],[@"p2",2],[@"p3",3]],[[@"p1",4],[@"p3",5],[@"p2",6]]]],[@"p2",[[[@"p2",7],[@"p1",8],[@"p3",9]],[[@"p2",10],[@"p3",11],[@"p1",12]]]],[@"p3",[[[@"p3",13],[@"p1",14],[@"p2",15]],[[@"p3",16],[@"p2",17],[@"p1",18]]]]]
                (seq
                    (seq
                        (fold permutations pair
                            (seq
                                (null)
                                (seq
                                    (fold pair.$.[1] pid-num-arr
                                        (seq
                                            (seq
                                                (call pair.$.[0] ("op" "noop") []) ; ok = null
                                                (ap pid-num-arr $pid-num-arrs)
                                            )
                                            (seq
                                                (null)
                                                (next pid-num-arr)
                                            )
                                        )
                                    )
                                    (next pair)
                                )
                            )
                        )
                        (seq
                            (canon "p1" $pid-num-arrs #pid-num-arrs-1)
                            (call "p1" ("test" "print") [#pid-num-arrs-1]) ; behaviour = echo
                        )
                    )
                    (seq
                        (seq
                            (canon "p1" $pid-num-arrs #pid-num-arrs-2)
                            (call "p1" ("test" "print") [#pid-num-arrs-2]) ; behaviour = echo
                        )
                        (new $result
                            (fold $pid-num-arrs pid-num-arr
                                (seq
                                    (seq
                                        (call "p1" ("test" "print") [pid-num-arr]) ; behaviour = echo
                                        (fold pid-num-arr pid-num
                                            (seq
                                                (seq
                                                    (null)
                                                    (seq
                                                        (call pid-num.$.[0] ("op" "noop") []) ; ok = null
                                                        (ap pid-num.$.[1] $result)
                                                    )
                                                )
                                                (seq
                                                    (seq
                                                        (canon pid-num.$.[0] $result #mon_res)
                                                        (call pid-num.$.[0] ("test" "print") [#mon_res]) ; behaviour = echo
                                                    )
                                                    (next pid-num)
                                                )
                                            )
                                        )
                                    )
                                    (seq
                                        (seq
                                            (canon "p1" $result #mon_res)
                                            (call "p1" ("test" "print") [#mon_res]) ; behaviour = echo
                                        )
                                        (xor
                                            (match #mon_res.length 18
                                                (call "p1" ("test" "print") [#mon_res.length]) ; behaviour = echo
                                            )
                                            (seq
                                                (call "p1" ("test" "print") ["not enought length"]) ; behaviour = echo
                                                (next pid-num-arr)
                                            )
                                        )
                                    )
                                )
                            )
                        )
                    )
                )
            )
            (seq
                (call "p1" ("op" "noop") ["final p1"]) ; behaviour = echo
                (seq
                    (canon "client" $result #end_result)
                    (call "p1" ("return" "") [#end_result]) ; behaviour = echo
                )
            )
        )
                "#;

    let engine = <AirScriptExecutor>::new(
        TestRunParameters::from_init_peer_id("client"),
        vec![],
        vec!["relay", "p1", "p2", "p3"].into_iter().map(Into::into),
        script,
    )
    .await
    .unwrap();

    let mut queue = std::collections::vec_deque::VecDeque::new();
    let mut p1_outcomes = Vec::<RawAVMOutcome>::new();
    queue.push_back("client".to_string());

    while !queue.is_empty() {
        let peer = queue.pop_front().unwrap();
        if let Some(outcomes) = engine.execution_iter(peer.as_str()) {
            for outcome in outcomes.collect::<Vec<_>>().await {
                assert_eq!(outcome.ret_code, 0, "{outcome:?}");

                for peer in &outcome.next_peer_pks {
                    if !queue.contains(peer) {
                        queue.push_back(peer.clone());
                    }
                }

                if peer == at("p1") {
                    p1_outcomes.push(outcome);
                }
            }
        } else {
            println!("peer: {peer}, no executions");
        }
    }

    let last_p1_data = p1_outcomes.last().unwrap();
    let last_p1_trace = trace_from_result(last_p1_data);
    let last_fold = last_p1_trace
        .iter()
        .filter_map(|state| match state {
            ExecutedState::Fold(fold_result) => Some(fold_result),
            _ => None,
        })
        .last()
        .unwrap();

    assert_eq!(last_fold.lore.len(), 6);
}
