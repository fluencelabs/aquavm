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
async fn par_remote_remote() {
    use std::collections::HashSet;

    let mut vm = create_avm(unit_call_service(), "").await;

    let script = r#"
            (par
                (call "remote_peer_id_1" ("local_service_id" "local_fn_name") [] result_name)
                (call "remote_peer_id_2" ("service_id" "fn_name") [] g)
            )"#;

    let mut result = checked_call_vm!(vm, <_>::default(), script, "", "");

    let actual_peers: HashSet<_> = result.next_peer_pks.drain(..).collect();
    let expected_peers: HashSet<_> =
        maplit::hashset!(String::from("remote_peer_id_1"), String::from("remote_peer_id_2"));

    assert_eq!(actual_peers, expected_peers);
}

#[tokio::test]
async fn par_local_remote() {
    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(unit_call_service(), local_peer_id).await;

    let script = format!(
        r#"
            (par
                (call "{local_peer_id}" ("local_service_id" "local_fn_name") [] result_name)
                (call "remote_peer_id_2" ("service_id" "fn_name") [] g)
            )"#
    );

    let result = checked_call_vm!(vm, <_>::default(), script, "", "");

    assert_eq!(result.next_peer_pks, vec![String::from("remote_peer_id_2")]);
}
