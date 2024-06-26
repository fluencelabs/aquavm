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

use air_test_utils::prelude::*;

#[tokio::test]
async fn ap_with_fold() {
    let nums: Vec<String> = (1..10).map(|i| i.to_string()).collect();
    let vec = vec![nums.clone(), nums.clone(), nums];
    let elems: Vec<(String, Vec<Vec<String>>)> = vec![
        ("a".into(), vec.clone()),
        ("a".into(), vec.clone()),
        ("a".into(), vec.clone()),
        ("a".into(), vec.clone()),
        ("a".into(), vec),
    ];
    let set_variable_id = "set_variable_peer_id";
    let mut set_variable_vm = create_avm(set_variable_call_service(json!(elems)), set_variable_id).await;

    let local_vm_peer_id = "local_peer_id";
    let mut local_vm = create_avm(unit_call_service(), local_vm_peer_id).await;

    let script = format!(
        r#"
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
                            (null)
                            (next ns))))
                (seq
                    (call "{local_vm_peer_id}" ("op" "noop") [])
                    (seq
                        (canon "{local_vm_peer_id}" $inner #canon_stream)
                        (call "{local_vm_peer_id}" ("return" "") [#canon_stream])))))
        "#
    );

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    assert_eq!(result.next_peer_pks, vec![local_vm_peer_id.to_string()]);

    let result = checked_call_vm!(local_vm, <_>::default(), &script, "", result.data);
    assert!(result.next_peer_pks.is_empty());
}
