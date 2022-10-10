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
fn issue_999() {
    //env_logger::init();
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
                        (null)
                        (fold $inner ns
                            (par
                                (fold ns pair
                                    (seq
                                        (seq
                                            (call pair.$.[0] ("op" "noop") []) ; ok = null
                                            (seq
                                                (ap pair.$.[1] $result)
                                                (call pair.$.[0] ("" "") [$result]) ; behaviour = echo
                                            )
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
                        (seq
                            (call "relay" ("" "") [$result]) ; behaviour = echo
                            (fold $result elem
                                (seq
                                    (seq
                                        (call "relay" ("" "") [$result]) ; behaviour = echo
                                        (ap elem $monotonic_stream)
                                    )
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
                        )
                        (canon "relay" $result #joined_result)
                    )
                )
                (call "client" ("return" "") [$inner #joined_result])  ; ok = null
            )
        )
        "#;

    /*
    17 - [71, 2], [75, 0]
    20 - [91, 2], [93, 0]
    25 - [73, 2], [75, 0]
    33 - [81, 2], [85, 0]
    36 - [85, 2], [87, 0]
    43 - [83, 2], [85, 0]
    46 - [95, 2], [97, 0]
    51 - [75, 2], [79, 0]
    54 - [79, 2], [81, 0]
    57 - [93, 2], [95, 0]
    61 - [77, 2], [79, 0]
    64 - [87, 2], [89, 0]
    67 - [89, 2], [91, 0]
     */
    let engine = TestExecutor::new(
        TestRunParameters::from_init_peer_id("client"),
        vec![],
        vec!["relay", "p1", "p2", "p3"].into_iter().map(Into::into),
        &script,
    )
    .unwrap();


    let mut queue = std::collections::vec_deque::VecDeque::new();
    queue.push_back("client".to_string());
    while !queue.is_empty() {
        let peer = queue.pop_front().unwrap();
        if let Some(outcomes) = engine.execution_iter(peer.as_str()) {
            //let mut next_peers = std::collections::HashSet::<String>::new();
            for outcome in outcomes {
                print_trace(&outcome, &format!("peer: {}",peer));
                assert_eq!(outcome.ret_code, 0, "{:?}", outcome);
                println!("next_peer_pks: {:?}", &outcome.next_peer_pks);
                for peer in outcome.next_peer_pks {
                    queue.push_back(peer);
                }
            }
        } else {
            println!("peer: {}, no executions", peer);
        }
    }

/*
    for cycle in 0..8 {
        for peer in ["client", "relay", "p1", "p2", "p3"] {
            if let Some(outcomes) = engine.execution_iter(peer) {
                for (iter, outcome) in outcomes.enumerate() {
                    print_trace(&outcome, &format!("cycle: {}, peer: {}, iter: {}", cycle, peer, iter));
                    println!("next_peer_pks: {:?}", outcome.next_peer_pks);
                    assert_eq!(outcome.ret_code, 0, "{:?}", outcome);
                }
            }
        }
    }*/

    println!("Execution finished!");
}
