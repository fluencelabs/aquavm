/*
 * Copyright 2020 Fluence Labs Limited
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

use air_test_utils::call_vm;
use air_test_utils::create_avm;
use air_test_utils::unit_call_service;

#[test]
fn par_remote_remote() {
    use std::collections::HashSet;

    let mut vm = create_avm(unit_call_service(), "");

    let script = r#"
            (par
                (call "remote_peer_id_1" ("local_service_id" "local_fn_name") [] result_name)
                (call "remote_peer_id_2" ("service_id" "fn_name") [] g)
            )"#;

    let mut result = call_vm!(vm, "", script, "", "");

    let actual_peers: HashSet<_> = result.next_peer_pks.drain(..).collect();
    let expected_peers: HashSet<_> =
        maplit::hashset!(String::from("remote_peer_id_1"), String::from("remote_peer_id_2"));

    assert_eq!(actual_peers, expected_peers);
}

#[test]
fn par_local_remote() {
    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(unit_call_service(), local_peer_id);

    let script = format!(
        r#"
            (par
                (call "{}" ("local_service_id" "local_fn_name") [] result_name)
                (call "remote_peer_id_2" ("service_id" "fn_name") [] g)
            )"#,
        local_peer_id
    );

    let result = call_vm!(vm, "", script, "", "");

    assert_eq!(result.next_peer_pks, vec![String::from("remote_peer_id_2")]);
}
