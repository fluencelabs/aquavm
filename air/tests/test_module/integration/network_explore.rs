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
// TODO: adjust test
#[ignore]
async fn network_explore() {
    let relay_id = "relay_id";
    let client_id = "client_id";
    let set_variables_state = maplit::hashmap!(
        "relay".to_string() => json!(relay_id),
        "client".to_string() => json!(client_id),
    );

    let client_call_service = set_variables_call_service(set_variables_state, VariableOptionSource::Argument(0));
    let mut client = create_avm(client_call_service, client_id).await;

    let client_1_id = "client_1_id";
    let client_2_id = "client_2_id";
    let client_3_id = "client_3_id";

    let relay_call_service = set_variable_call_service(json!([client_1_id, client_2_id, client_3_id, relay_id]));
    let mut relay = create_avm(relay_call_service, relay_id).await;

    let client_1_call_service = set_variable_call_service(json!([client_1_id, client_3_id, relay_id, client_2_id]));
    let mut client_1 = create_avm(client_1_call_service, client_1_id).await;

    let client_2_call_service = set_variable_call_service(json!([relay_id, client_3_id, client_1_id, client_2_id]));
    let mut client_2 = create_avm(client_2_call_service, client_2_id).await;

    let client_3_call_service = set_variable_call_service(json!([relay_id, client_3_id, client_1_id, client_2_id]));
    let mut client_3 = create_avm(client_3_call_service, client_3_id).await;

    let script = include_str!("./scripts/network_explore.air");

    let client_result = checked_call_vm!(client, <_>::default(), script, "", "");
    assert_next_pks!(&client_result.next_peer_pks, &[relay_id]);

    let relay_result = checked_call_vm!(relay, <_>::default(), script, "", client_result.data.clone());
    assert_next_pks!(&relay_result.next_peer_pks, &[client_1_id]);

    let client_1_result = checked_call_vm!(client_1, <_>::default(), script, "", relay_result.data.clone());
    assert_next_pks!(&client_1_result.next_peer_pks, &[client_2_id]);

    let client_2_result = checked_call_vm!(client_2, <_>::default(), script, "", client_1_result.data.clone());
    assert_next_pks!(&client_2_result.next_peer_pks, &[client_3_id]);

    let client_3_result = checked_call_vm!(client_3, <_>::default(), script, "", client_2_result.data.clone());
    assert_next_pks!(&client_3_result.next_peer_pks, &[relay_id]);

    let relay_result = checked_call_vm!(
        relay,
        <_>::default(),
        script,
        relay_result.data,
        client_3_result.data.clone()
    );
    assert_next_pks!(&relay_result.next_peer_pks, &[client_1_id]);

    let client_1_result = checked_call_vm!(
        client_1,
        <_>::default(),
        script,
        client_1_result.data,
        relay_result.data.clone()
    );
    assert_next_pks!(&client_1_result.next_peer_pks, &[client_3_id]);

    let client_3_result = checked_call_vm!(
        client_3,
        <_>::default(),
        script,
        client_3_result.data,
        client_1_result.data.clone()
    );
    assert_next_pks!(&client_3_result.next_peer_pks, &[relay_id]);

    let relay_result = checked_call_vm!(
        relay,
        <_>::default(),
        script,
        relay_result.data,
        client_3_result.data.clone()
    );
    assert_next_pks!(&relay_result.next_peer_pks, &[client_2_id]);

    let client_2_result = checked_call_vm!(
        client_2,
        <_>::default(),
        script,
        client_2_result.data,
        relay_result.data.clone()
    );
    assert_next_pks!(&client_2_result.next_peer_pks, &[relay_id]);

    let relay_result = checked_call_vm!(
        relay,
        <_>::default(),
        script,
        relay_result.data,
        client_2_result.data.clone()
    );
    assert_next_pks!(&relay_result.next_peer_pks, &[client_3_id]);

    let client_3_result = checked_call_vm!(
        client_3,
        <_>::default(),
        script,
        client_3_result.data,
        relay_result.data.clone()
    );
    assert_next_pks!(&client_3_result.next_peer_pks, &[client_1_id]);

    let client_1_result = checked_call_vm!(
        client_1,
        <_>::default(),
        script,
        client_1_result.data,
        client_3_result.data.clone()
    );
    assert_next_pks!(&client_1_result.next_peer_pks, &[client_2_id]);

    let client_2_result = checked_call_vm!(
        client_2,
        <_>::default(),
        script,
        client_2_result.data,
        client_1_result.data.clone()
    );
    assert_next_pks!(&client_2_result.next_peer_pks, &[relay_id]);

    let relay_result = checked_call_vm!(
        relay,
        <_>::default(),
        script,
        relay_result.data,
        client_2_result.data.clone()
    );
    assert_next_pks!(&relay_result.next_peer_pks, &[client_3_id]);

    let client_3_result = checked_call_vm!(
        client_3,
        <_>::default(),
        script,
        client_3_result.data,
        relay_result.data.clone()
    );
    assert_next_pks!(&client_3_result.next_peer_pks, &[client_1_id]);

    let client_1_result = checked_call_vm!(
        client_1,
        <_>::default(),
        script,
        client_1_result.data,
        client_3_result.data.clone()
    );
    assert_next_pks!(&client_1_result.next_peer_pks, &[client_2_id]);

    let client_2_result = checked_call_vm!(
        client_2,
        <_>::default(),
        script,
        client_2_result.data,
        client_1_result.data.clone()
    );
    assert_next_pks!(&client_2_result.next_peer_pks, &[client_1_id]);

    let client_1_result = checked_call_vm!(
        client_1,
        <_>::default(),
        script,
        client_1_result.data,
        client_2_result.data.clone()
    );
    assert_next_pks!(&client_1_result.next_peer_pks, &[client_2_id]);

    let client_2_result = checked_call_vm!(
        client_2,
        <_>::default(),
        script,
        client_2_result.data,
        client_1_result.data.clone()
    );
    assert_next_pks!(&client_2_result.next_peer_pks, &[client_3_id]);

    let client_3_result = checked_call_vm!(
        client_3,
        <_>::default(),
        script,
        client_3_result.data,
        client_2_result.data.clone()
    );
    assert_next_pks!(&client_3_result.next_peer_pks, &[relay_id]);

    let relay_result = checked_call_vm!(
        relay,
        <_>::default(),
        script,
        relay_result.data,
        client_3_result.data.clone()
    );
    assert_next_pks!(&relay_result.next_peer_pks, &[client_id]);

    let client_result = checked_call_vm!(
        client,
        <_>::default(),
        script,
        client_result.data,
        relay_result.data.clone()
    );
    assert_next_pks!(&client_result.next_peer_pks, &[]);
}
