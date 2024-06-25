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

use air::ExecutionCidState;
use air_test_utils::prelude::*;

#[tokio::test]
async fn seq_remote_remote() {
    let mut vm = create_avm(unit_call_service(), "").await;
    let mut cid_state = ExecutionCidState::new();

    let script = r#"
            (seq
                (call "remote_peer_id_1" ("local_service_id" "local_fn_name") [] result_name)
                (call "remote_peer_id_2" ("service_id" "fn_name") [] g)
            )"#;

    let result = checked_call_vm!(vm, <_>::default(), script, "", "");
    assert_eq!(result.next_peer_pks, vec![String::from("remote_peer_id_1")]);

    let initial_trace = vec![scalar_tracked!(
        "",
        cid_state,
        peer = "remote_peer_id_1",
        service = "local_service_id",
        function = "local_fn_name"
    )];
    let initial_data = raw_data_from_trace(initial_trace, cid_state.into());

    let result = checked_call_vm!(vm, <_>::default(), script, "", initial_data);

    assert_eq!(result.next_peer_pks, vec![String::from("remote_peer_id_2")]);
}

#[tokio::test]
async fn seq_local_remote() {
    let local_peer_id = "local_peer_id";
    let remote_peer_id = String::from("remote_peer_id");
    let mut vm = create_avm(unit_call_service(), local_peer_id).await;

    let script = format!(
        r#"
            (seq
                (call "{local_peer_id}" ("local_service_id" "local_fn_name") [] result_name)
                (call "{remote_peer_id}" ("service_id" "fn_name") [] g)
            )"#
    );

    let result = checked_call_vm!(vm, <_>::default(), script, "", "");
    assert_eq!(result.next_peer_pks, vec![remote_peer_id]);
}
