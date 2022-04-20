/*
 * Copyright 2021 Fluence Labs Limited
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

use std::collections::HashSet;

#[test]
// https://github.com/fluencelabs/aquavm/issues/178
fn par_ap_behaviour() {
    let client_id = "client_id";
    let relay_id = "relay_id";
    let variable_setter_id = "variable_setter_id";
    let mut client = create_avm(unit_call_service(), client_id);
    let mut relay = create_avm(unit_call_service(), relay_id);
    let mut variable_setter = create_avm(unit_call_service(), variable_setter_id);

    let script = f!(r#"
        (par
            (call "{variable_setter_id}" ("peer" "timeout") [] join_it)
            (seq
                (par
                    (call "{relay_id}" ("peer" "timeout") [join_it] $result)
                    (ap "fast_result" $result) ;; ap doesn't affect the subtree_complete flag
                )
                (call "{client_id}" ("op" "return") [$result.$[0]])
            )
        )
        "#);

    let mut client_result_1 = checked_call_vm!(client, <_>::default(), &script, "", "");
    let actual_next_peers: HashSet<_> = client_result_1.next_peer_pks.drain(..).collect();
    let expected_next_peers: HashSet<_> = maplit::hashset!(relay_id.to_string(), variable_setter_id.to_string());
    assert_eq!(actual_next_peers, expected_next_peers);

    let setter_result = checked_call_vm!(
        variable_setter,
        <_>::default(),
        &script,
        "",
        client_result_1.data.clone()
    );
    assert!(setter_result.next_peer_pks.is_empty());

    let relay_result = checked_call_vm!(relay, <_>::default(), script, "", client_result_1.data);
    assert!(relay_result.next_peer_pks.is_empty());
}
