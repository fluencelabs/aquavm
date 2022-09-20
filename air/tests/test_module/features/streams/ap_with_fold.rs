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

#[test]
fn ap_with_fold() {
    let nums: Vec<String> = (1..10).map(|i| i.to_string()).collect();
    let vec = vec![nums.clone(), nums.clone(), nums.clone()];
    let elems: Vec<(String, Vec<Vec<String>>)> = vec![
        ("a".into(), vec.clone()),
        ("a".into(), vec.clone()),
        ("a".into(), vec.clone()),
        ("a".into(), vec.clone()),
        ("a".into(), vec),
    ];
    let set_variable_id = "set_variable_peer_id";
    let mut set_variable_vm = create_avm(set_variable_call_service(json!(elems)), set_variable_id);

    let local_vm_peer_id = "local_peer_id";
    let mut local_vm = create_avm(unit_call_service(), local_vm_peer_id);

    let script = f!(r#"
        (seq
            (call "{set_variable_id}" ("" "") [] permutations)
            (seq
                (seq
                    (fold permutations pair
                        (par
                            (fold pair.$.[1]! peer_ids
                                (par
                                    (ap peer_ids $inner)
                                    (next peer_ids)))
                            (next pair)))
                    (fold $inner ns
                        (par
                            (next ns)
                            (null))))
                (seq
                    (call "{local_vm_peer_id}" ("op" "noop") [])
                    (call "{local_vm_peer_id}" ("return" "") [$inner]))))
        "#);

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    assert_eq!(result.next_peer_pks, vec![local_vm_peer_id.to_string()]);

    let result = checked_call_vm!(local_vm, <_>::default(), &script, "", result.data);
    assert!(result.next_peer_pks.is_empty());
}
