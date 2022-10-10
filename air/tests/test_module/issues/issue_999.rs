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
    let mut client_vm = create_avm(set_variable_call_service(json!([["p1",[[["p1",1],["p3",3]],[["p1",4],["p3",5]]]],["p3",[[["p3",13],["p1",14]],[["p3",16],["p1",18]]]]])), client_peer_id);

    let relay_peer_id = "relay";
    let mut relay_vm = create_avm(echo_call_service(), relay_peer_id);

    let p1 = "p1";
    let mut p1_vm = create_avm(set_variable_call_service(json!("p1")), p1);
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
            ) (null)
        )
        "#;

    let client_result = checked_call_vm!(client_vm, <_>::default(), script, "", "");

    let p1_result_1 = checked_call_vm!(p1_vm, <_>::default(), script, "", client_result.data.clone());

    let p3_result_1 = checked_call_vm!(p3_vm, <_>::default(), script, "", p1_result_1.data.clone());
    let p3_trace_1 = trace_from_result(&p3_result_1);


    let fold_p3 = p3_trace_1.get(TracePos::from(9)).unwrap();
    if let ExecutedState::Fold(fold) = fold_p3 {
        assert_eq!(fold.lore.len(), 4);
        assert_eq!(fold.lore[0].subtraces_desc[0].subtrace_len, 2);
        assert_eq!(fold.lore[1].subtraces_desc[0].subtrace_len, 2);
        assert_eq!(fold.lore[2].subtraces_desc[0].subtrace_len, 4);
        assert_eq!(fold.lore[3].subtraces_desc[0].subtrace_len, 4);
    } else {
        panic!("expected fold at pos 9")
    }

    let p1_result_2 = checked_call_vm!(p1_vm, <_>::default(), script, p1_result_1.data.clone(), p3_result_1.data.clone());
    let p1_trace_2 = trace_from_result(&p1_result_2);
    let fold_p1 = p1_trace_2.get(TracePos::from(9)).unwrap();
    if let ExecutedState::Fold(fold) = fold_p1 {
        assert_eq!(fold.lore.len(), 4);
        assert_eq!(fold.lore[0].subtraces_desc[0].subtrace_len, 4);
        assert_eq!(fold.lore[1].subtraces_desc[0].subtrace_len, 4);
        assert_eq!(fold.lore[2].subtraces_desc[0].subtrace_len, 5);
        assert_eq!(fold.lore[3].subtraces_desc[0].subtrace_len, 5);
    } else {
        panic!("expected fold at pos 9")
    }
}
