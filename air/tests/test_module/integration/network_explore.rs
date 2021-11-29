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

use air_test_utils::prelude::*;

#[test]
fn network_explore() {
    let relay_id = "relay_id";
    let client_id = "client_id";
    let set_variables_state = maplit::hashmap!(
        "relay".to_string() => json!(relay_id),
        "client".to_string() => json!(client_id),
    );

    let client_call_service = set_variables_call_service(set_variables_state, VariableOptionSource::Argument(0));
    let mut client = create_avm(client_call_service, client_id);

    let client_1_id = "client_1_id";
    let client_2_id = "client_2_id";
    let client_3_id = "client_3_id";

    let relay_call_service = set_variable_call_service(json!([client_1_id, client_2_id, client_3_id, relay_id]));
    let mut relay = create_avm(relay_call_service, relay_id);

    let client_1_call_service = set_variable_call_service(json!([client_1_id, client_3_id, relay_id, client_2_id]));
    let mut client_1 = create_avm(client_1_call_service, client_1_id);

    let client_2_call_service = set_variable_call_service(json!([relay_id, client_3_id, client_1_id, client_2_id]));
    let mut client_2 = create_avm(client_2_call_service, client_2_id);

    let client_3_call_service = set_variable_call_service(json!([relay_id, client_3_id, client_1_id, client_2_id]));
    let mut client_3 = create_avm(client_3_call_service, client_3_id);

    let script = include_str!("./scripts/network_explore.clj");

    let client_result = checked_call_vm!(client, "", script, "", "");
    assert_next_pks!(&client_result.next_peer_pks, &[relay_id]);

    let relay_result = checked_call_vm!(relay, "", script, "", client_result.data.clone());
    assert_next_pks!(&relay_result.next_peer_pks, &[client_1_id]);

    let client_1_result = checked_call_vm!(client_1, "", script, "", relay_result.data.clone());
    assert_next_pks!(&client_1_result.next_peer_pks, &[client_2_id]);

    let client_2_result = checked_call_vm!(client_2, "", script, "", client_1_result.data.clone());
    assert_next_pks!(&client_2_result.next_peer_pks, &[client_3_id]);

    let client_3_result = checked_call_vm!(client_3, "", script, "", client_2_result.data.clone());
    assert_next_pks!(&client_3_result.next_peer_pks, &[relay_id]);

    let relay_result = checked_call_vm!(relay, "", script, relay_result.data, client_3_result.data.clone());
    assert_next_pks!(&relay_result.next_peer_pks, &[client_1_id]);

    let client_1_result = checked_call_vm!(client_1, "", script, client_1_result.data, relay_result.data.clone());
    assert_next_pks!(&client_1_result.next_peer_pks, &[client_3_id]);

    let client_3_result = checked_call_vm!(client_3, "", script, client_3_result.data, client_1_result.data.clone());
    assert_next_pks!(&client_3_result.next_peer_pks, &[relay_id]);

    let relay_result = checked_call_vm!(relay, "", script, relay_result.data, client_3_result.data.clone());
    assert_next_pks!(&relay_result.next_peer_pks, &[client_2_id]);

    let client_2_result = checked_call_vm!(client_2, "", script, client_2_result.data, relay_result.data.clone());
    assert_next_pks!(&client_2_result.next_peer_pks, &[relay_id]);

    let relay_result = checked_call_vm!(relay, "", script, relay_result.data, client_2_result.data.clone());
    assert_next_pks!(&relay_result.next_peer_pks, &[client_3_id]);

    let client_3_result = checked_call_vm!(client_3, "", script, client_3_result.data, relay_result.data.clone());
    assert_next_pks!(&client_3_result.next_peer_pks, &[client_1_id]);

    let client_1_result = checked_call_vm!(client_1, "", script, client_1_result.data, client_3_result.data.clone());
    assert_next_pks!(&client_1_result.next_peer_pks, &[client_2_id]);

    let client_2_result = checked_call_vm!(client_2, "", script, client_2_result.data, client_1_result.data.clone());
    assert_next_pks!(&client_2_result.next_peer_pks, &[relay_id]);

    let relay_result = checked_call_vm!(relay, "", script, relay_result.data, client_2_result.data.clone());
    assert_next_pks!(&relay_result.next_peer_pks, &[client_3_id]);

    let client_3_result = checked_call_vm!(client_3, "", script, client_3_result.data, relay_result.data.clone());
    assert_next_pks!(&client_3_result.next_peer_pks, &[client_1_id]);

    let client_1_result = checked_call_vm!(client_1, "", script, client_1_result.data, client_3_result.data.clone());
    assert_next_pks!(&client_1_result.next_peer_pks, &[client_2_id]);

    let client_2_result = checked_call_vm!(client_2, "", script, client_2_result.data, client_1_result.data.clone());
    assert_next_pks!(&client_2_result.next_peer_pks, &[client_1_id]);

    let client_1_result = checked_call_vm!(client_1, "", script, client_1_result.data, client_2_result.data.clone());
    assert_next_pks!(&client_1_result.next_peer_pks, &[client_2_id]);

    let client_2_result = checked_call_vm!(client_2, "", script, client_2_result.data, client_1_result.data.clone());
    assert_next_pks!(&client_2_result.next_peer_pks, &[client_3_id]);

    let client_3_result = checked_call_vm!(client_3, "", script, client_3_result.data, client_2_result.data.clone());
    assert_next_pks!(&client_3_result.next_peer_pks, &[relay_id]);

    let relay_result = checked_call_vm!(relay, "", script, relay_result.data, client_3_result.data.clone());
    assert_next_pks!(&relay_result.next_peer_pks, &[client_id]);

    let client_result = checked_call_vm!(client, "", script, client_result.data, relay_result.data.clone());
    assert_next_pks!(&client_result.next_peer_pks, &[]);
}
