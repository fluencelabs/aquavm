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
use air_test_utils::set_variables_call_service;

use serde_json::json;

#[test]
fn network_explore() {
    let relay_id = String::from("12D3KooWSpr929UacQSTUWQeK7CQPhcW2TSmLrGTdE1NcFWkuCvY");
    let client_id = String::from("12D3KooWEDU1WwGtvHUKpGCaMjhcLWyCUq3MQiRKZBLLFcBVVMck");
    let set_variables_state = maplit::hashmap!(
        String::from("relay") => json!(relay_id).to_string(),
        String::from("client") => json!(client_id).to_string(),
    );

    let client_call_service = set_variables_call_service(set_variables_state);
    let mut client = create_avm(
        client_call_service,
        "12D3KooWEDU1WwGtvHUKpGCaMjhcLWyCUq3MQiRKZBLLFcBVVMck",
    );

    let client_1_id = String::from("12D3KooWFX27Tg3cNJkFk3W2iapnyRhwfwdQ4ZiTucsy1Go3MSGL");
    let client_2_id = String::from("12D3KooWGNJoFmCNEHq8NpunB4pZSUh9UBHM53W1NwE7gM8L3RjZ");
    let relay_call_service =
        air_test_utils::set_variable_call_service(format!(r#"["{}", "{}"]"#, client_1_id, client_2_id));
    let mut relay = create_avm(relay_call_service, relay_id.clone());

    let client_1_call_service =
        air_test_utils::set_variable_call_service(format!(r#"["{}", "{}"]"#, relay_id, client_2_id));
    let mut client_1 = create_avm(client_1_call_service, client_1_id.clone());

    let client_2_call_service =
        air_test_utils::set_variable_call_service(format!(r#"["{}", "{}"]"#, relay_id, client_1_id));
    let mut client_2 = create_avm(client_2_call_service, client_2_id.clone());

    let script = include_str!("./scripts/network_explore.clj");

    let client_res = call_vm!(client, "", script, "[]", "[]");
    assert_eq!(client_res.next_peer_pks, vec![relay_id.clone()]);

    let relay_res = call_vm!(relay, "", script, "", client_res.data);
    assert_eq!(relay_res.next_peer_pks, vec![client_1_id.clone()]);

    let client_1_res = call_vm!(client_1, "", script, "", relay_res.data.clone());
    assert_eq!(client_1_res.next_peer_pks, vec![client_2_id.clone()]);

    let client_2_res = call_vm!(client_2, "", script, "", client_1_res.data.clone());
    assert_eq!(client_2_res.next_peer_pks, vec![relay_id.clone()]);

    let relay_res = call_vm!(relay, "", script, relay_res.data, client_2_res.data.clone());
    assert_eq!(relay_res.next_peer_pks, vec![client_2_id.clone()]);

    let client_2_res = call_vm!(client_2, "", script, client_2_res.data, relay_res.data.clone());
    assert_eq!(client_2_res.next_peer_pks, vec![relay_id.clone()]);

    let relay_res = call_vm!(relay, "", script, relay_res.data, client_2_res.data);
    assert_eq!(relay_res.next_peer_pks, vec![client_1_id.clone()]);

    let client_1_res = call_vm!(client_1, "", script, client_1_res.data, relay_res.data);
    assert_eq!(client_1_res.next_peer_pks, Vec::<String>::new());
}
