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

use air_test_utils::prelude::*;

//https://github.com/fluencelabs/aquavm/issues/363
#[tokio::test]
fn issue_363() {
    let client_peer_id = "client";
    let mut client_vm = create_avm(
        set_variable_call_service(json!([
            ["p1", [[["p1", 1], ["p2", 3]], [["p1", 4], ["p2", 5]]]],
            ["p2", [[["p2", 13], ["p1", 14]], [["p2", 16], ["p1", 18]]]]
        ])),
        client_peer_id,
    );

    let p1_peer_id = "p1";
    let mut p1_vm = create_avm(set_variable_call_service(json!("p1")), p1_peer_id);
    let p2_peer_id = "p2";
    let mut p2_vm = create_avm(set_variable_call_service(json!("p2")), p2_peer_id);

    let script = r#"
        (seq
            (seq
                (call "client" ("get" "data") [] permutations)
                (seq
                    (fold permutations pair
                        (seq
                            (fold pair.$.[1] peer_ids
                                (seq
                                    (seq
                                        (call pair.$.[0] ("op" "noop") [])
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
                                            (call pair.$.[0] ("op" "noop") [])
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
    let p1_result_1 = checked_call_vm!(p1_vm, <_>::default(), script, "", client_result.data);
    let p2_result_1 = checked_call_vm!(p2_vm, <_>::default(), script, "", p1_result_1.data.clone());

    let p2_trace_1 = trace_from_result(&p2_result_1);
    let fold_position = TracePos::from(9);
    let fold_p2 = p2_trace_1.get(fold_position).unwrap();
    if let ExecutedState::Fold(fold) = fold_p2 {
        assert_eq!(fold.lore.len(), 4);
        assert_eq!(fold.lore[0].subtraces_desc[0].subtrace_len, 2);
        assert_eq!(fold.lore[1].subtraces_desc[0].subtrace_len, 2);
        assert_eq!(fold.lore[2].subtraces_desc[0].subtrace_len, 4);
        assert_eq!(fold.lore[3].subtraces_desc[0].subtrace_len, 4);
    } else {
        panic!("expected fold at pos 9")
    }

    let p1_result_2 = checked_call_vm!(p1_vm, <_>::default(), script, p1_result_1.data, p2_result_1.data);
    let p1_trace_2 = trace_from_result(&p1_result_2);
    let fold_p1 = p1_trace_2.get(fold_position).unwrap();
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
