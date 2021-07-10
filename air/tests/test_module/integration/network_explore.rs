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

use air_test_utils::checked_call_vm;
use air_test_utils::create_avm;
use air_test_utils::print_trace;
use air_test_utils::set_variables_call_service;

use serde_json::json;

#[test]
fn network_explore() {
    let relay_id = "relay_id";
    let client_id = "client_id";
    let set_variables_state = maplit::hashmap!(
        "relay".to_string() => json!(relay_id).to_string(),
        "client".to_string() => json!(client_id).to_string(),
    );

    let client_call_service = set_variables_call_service(set_variables_state);
    let mut client = create_avm(client_call_service, client_id);

    let client_1_id = "client_1_id";
    let client_2_id = "client_2_id";

    let relay_call_service = air_test_utils::set_variable_call_service(json!([client_1_id, client_2_id]).to_string());
    let mut relay = create_avm(relay_call_service, relay_id);

    let client_1_call_service = air_test_utils::set_variable_call_service(json!([relay_id, client_2_id]).to_string());
    let mut client_1 = create_avm(client_1_call_service, client_1_id);

    let client_2_call_service = air_test_utils::set_variable_call_service(json!([relay_id, client_1_id]).to_string());
    let mut client_2 = create_avm(client_2_call_service, client_2_id);

    let script = include_str!("./scripts/network_explore.clj");

    let client_result = checked_call_vm!(client, "", script, "", "");
    assert_eq!(client_result.next_peer_pks, vec![relay_id.to_string()]);

    let relay_result = checked_call_vm!(relay, "", script, "", client_result.data.clone());
    assert_eq!(relay_result.next_peer_pks, vec![client_1_id.to_string()]);

    let client_1_result = checked_call_vm!(client_1, "", script, "", relay_result.data.clone());
    assert_eq!(client_1_result.next_peer_pks, vec![client_2_id.to_string()]);

    let client_2_result = checked_call_vm!(client_2, "", script, "", client_1_result.data.clone());
    assert_eq!(client_2_result.next_peer_pks, vec![relay_id.to_string()]);

    let relay_result = checked_call_vm!(relay, "", script, relay_result.data, client_2_result.data.clone());
    assert_eq!(relay_result.next_peer_pks, vec![client_2_id.to_string()]);

    let client_2_result = checked_call_vm!(client_2, "", script, client_2_result.data, relay_result.data.clone());
    assert_eq!(client_2_result.next_peer_pks, vec![relay_id.to_string()]);

    let relay_result = checked_call_vm!(relay, "", script, relay_result.data, client_2_result.data);
    assert_eq!(relay_result.next_peer_pks, vec![client_1_id.clone()]);

    let client_1_result = checked_call_vm!(client_1, "", script, client_1_result.data, relay_result.data.clone());
    assert_eq!(client_1_result.next_peer_pks, vec![relay_id.to_string()]);

    let relay_result = checked_call_vm!(relay, "", script, relay_result.data.clone(), client_1_result.data);
    assert_eq!(relay_result.next_peer_pks, vec![client_id.to_string()]);

    let client_result = checked_call_vm!(client, "", script, client_result.data, relay_result.data);
    assert_eq!(client_result.next_peer_pks, Vec::<String>::new());
}
