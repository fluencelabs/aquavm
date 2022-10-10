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
    let client_peer_id = "client";
    let mut client_vm = create_avm(set_variable_call_service(json!([["p1",[[["p1",1],["p2",2],["p3",3]],[["p1",4],["p3",5],["p2",6]]]],["p2",[[["p2",7],["p1",8],["p3",9]],[["p2",10],["p3",11],["p1",12]]]],["p3",[[["p3",13],["p1",14],["p2",15]],[["p3",16],["p2",17],["p1",18]]]]])), client_peer_id);

    let relay_peer_id = "relay";
    let mut relay_vm = create_avm(echo_call_service(), relay_peer_id);

    let p1 = "p1";
    let mut p1_vm = create_avm(set_variable_call_service(json!("p1")), p1);
    let p2 = "p2";
    let mut p2_vm = create_avm(set_variable_call_service(json!("p2")), p2);
    let p3 = "p3";
    let mut p3_vm = create_avm(set_variable_call_service(json!("p3")), p3);

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
                                                (null)
                                                 ; (call pair.$.[0] ("" "") [$result]) ; behaviour = echo
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

    let client_result = checked_call_vm!(client_vm, <_>::default(), script, "", "");
    //println!("client next peer id = {:?}", client_result.next_peer_pks);

    let p1_result_1 = checked_call_vm!(p1_vm, <_>::default(), script, "", client_result.data.clone());
    //println!("p1 1 next peer id = {:?}", p1_result_1.next_peer_pks);

    let p2_result_1 = checked_call_vm!(p2_vm, <_>::default(), script, "", p1_result_1.data.clone());
    //println!("p2 1 next peer id = {:?}", p2_result_1.next_peer_pks);

    let p3_result_1 = checked_call_vm!(p3_vm, <_>::default(), script, "", p2_result_1.data.clone());
    //println!("p3 1 next peer id = {:?}\n", p3_result_1.next_peer_pks);
    //print_trace(&p3_result_1, "p3 1");

    // p3 -> p2, p1
    //print_trace(&p1_result_1, "p1 1");
    //print_trace(&p3_result_1, "p3 1");
    println!("\n\n\n========\n\n\n");
    let ddd: InterpreterData = serde_json::from_slice(&p1_result_1.data).unwrap();
    let ddd1: InterpreterData = serde_json::from_slice(&p3_result_1.data).unwrap();
    println!("PREV DATA: {:?}", ddd.global_streams);
    println!("CURRENT DATA: {:?}", ddd1.global_streams);
    let p1_result_2 = checked_call_vm!(p1_vm, <_>::default(), script, p1_result_1.data.clone(), p3_result_1.data.clone());
    println!("p1 2 next peer id = {:?}", p1_result_2.next_peer_pks);
    print_trace(&p1_result_2, "p1 2");
    /*
    let p2_result_2 = checked_call_vm!(p2_vm, <_>::default(), script, p2_result_1.data.clone(), p3_result_1.data.clone());
    println!("p2 2 next peer id = {:?}\n", p2_result_2.next_peer_pks);
    //print_trace(&p2_result_2, "p2 2");

    // p1 -> p3, p2
    let p3_result_2 = checked_call_vm!(p3_vm, <_>::default(), script, p3_result_1.data.clone(), p1_result_2.data.clone());
    println!("p3 2 next peer id = {:?}", p3_result_2.next_peer_pks);
    let p2_result_3 = checked_call_vm!(p2_vm, <_>::default(), script, p2_result_2.data.clone(), p1_result_2.data.clone());
    println!("p1 3 next peer id = {:?}\n", p2_result_3.next_peer_pks);

    // p2 -> p3, p1
    let p3_result_3 = checked_call_vm!(p3_vm, <_>::default(), script, p3_result_2.data.clone(), p2_result_2.data.clone());
    println!("p3 3 next peer id = {:?}", p3_result_3.next_peer_pks);
    let p1_result_3 = checked_call_vm!(p1_vm, <_>::default(), script, p1_result_2.data.clone(), p2_result_2.data.clone());
    println!("p1 3 next peer id = {:?}\n", p1_result_3.next_peer_pks);

    // p3 -> p2
    // p1 -> p3
    // p3 -> p1
    // p1 -> p3
    let p2_result_4 = checked_call_vm!(p2_vm, <_>::default(), script, p2_result_3.data.clone(), p3_result_3.data.clone());
    println!("p2 4 next peer id = {:?}", p2_result_4.next_peer_pks);
    let p3_result_4 = checked_call_vm!(p3_vm, <_>::default(), script, p3_result_3.data.clone(), p1_result_3.data.clone());
    println!("p3 4 next peer id = {:?}", p3_result_4.next_peer_pks);
    let p1_result_4 = checked_call_vm!(p1_vm, <_>::default(), script, p1_result_3.data.clone(), p3_result_4.data.clone());
    println!("p1 4 next peer id = {:?}", p1_result_4.next_peer_pks);
    let p3_result_5 = checked_call_vm!(p3_vm, <_>::default(), script, p3_result_4.data.clone(), p1_result_4.data.clone());
    println!("p3 5 next peer id = {:?}\n", p3_result_5.next_peer_pks);

    //print_trace(&p3_result_4, "p3 result 4");
    //print_trace(&p3_result_5, "p3 result 5");

    // p2 = ["p1", "relay"]
    // p3 = ["relay"]
    // p1 = ["p2"]
    // p3 = []
    let p1_result_5 = checked_call_vm!(p1_vm, <_>::default(), script, p1_result_4.data.clone(), p2_result_4.data.clone());
    println!("p1 5 next peer id = {:?}", p1_result_5.next_peer_pks);
    let relay_result_1 = checked_call_vm!(relay_vm, <_>::default(), script, "", p2_result_4.data.clone());
    println!("relay 1 next peer id = {:?}", relay_result_1.next_peer_pks);
    let relay_result_2 = checked_call_vm!(relay_vm, <_>::default(), script, relay_result_1.data.clone(), p3_result_5.data.clone());
    println!("relay 2 next peer id = {:?}", relay_result_2.next_peer_pks);
    let p2_result_5 = checked_call_vm!(p2_vm, <_>::default(), script, p2_result_4.data.clone(), p1_result_5.data.clone());
    println!("p2 5 next peer id = {:?}\n", p2_result_5.next_peer_pks);
     */
    //print_trace(&relay_result_2, "relay 2");

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
    /*
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

     */

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
