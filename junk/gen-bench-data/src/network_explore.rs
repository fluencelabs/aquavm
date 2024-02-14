/*
 * Copyright 2023 Fluence Labs Limited
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

use super::Data;

use air_test_utils::key_utils::derive_dummy_keypair;
use air_test_utils::prelude::*;
use maplit::hashmap;

pub(crate) async fn network_explore() -> Data {
    let relay_name = "relay_id";
    let client_name = "client_id";

    let client_1_name = "client_1_id";
    let client_2_name = "client_2_id";
    let client_3_name = "client_3_id";

    let (relay_key, relay_id) = derive_dummy_keypair(relay_name);
    let (client_key, client_id) = derive_dummy_keypair(client_name);
    let (client_1_key, client_1_id) = derive_dummy_keypair(client_1_name);
    let (client_2_key, client_2_id) = derive_dummy_keypair(client_2_name);
    let (client_3_key, client_3_id) = derive_dummy_keypair(client_3_name);

    let set_variables_state = maplit::hashmap!(
        "relay".to_string() => json!(&relay_id),
        "client".to_string() => json!(&client_id),
    );

    let client_call_service =
        set_variables_call_service(set_variables_state, VariableOptionSource::Argument(0));
    let mut client = create_avm_with_key::<NativeAirRunner>(client_key, client_call_service).await;

    let relay_call_service =
        set_variable_call_service(json!([&client_1_id, &client_2_id, &client_3_id, &relay_id]));
    let mut relay = create_avm_with_key::<NativeAirRunner>(relay_key.clone(), relay_call_service).await;

    let client_1_call_service =
        set_variable_call_service(json!([&client_1_id, &client_3_id, &relay_id, &client_2_id]));
    let mut client_1 = create_avm_with_key::<NativeAirRunner>(client_1_key, client_1_call_service).await;

    let client_2_call_service =
        set_variable_call_service(json!([&relay_id, &client_3_id, &client_1_id, &client_2_id]));
    let mut client_2 = create_avm_with_key::<NativeAirRunner>(client_2_key, client_2_call_service).await;

    let client_3_call_service =
        set_variable_call_service(json!([&relay_id, &client_3_id, &client_1_id, &client_2_id]));
    let mut client_3 = create_avm_with_key::<NativeAirRunner>(client_3_key, client_3_call_service).await;

    let raw_script = include_str!("network_explore.air");

    // kind of hack: transform peer id in calls
    let script = {
        let network = air_test_framework::Network::<NativeAirRunner>::new(
            std::iter::empty::<air_test_framework::ephemeral::PeerId>(),
            vec![],
        ).await;
        let transformed_script =
            air_test_framework::TransformedAirScript::new(raw_script, network).await.unwrap();
        &(*transformed_script).to_string()
    };

    let client_result = checked_call_vm!(client, <_>::default(), script, "", "");
    assert_next_pks!(&client_result.next_peer_pks, &[relay_id.as_str()]);

    let relay_result = checked_call_vm!(
        relay,
        <_>::default(),
        script,
        "",
        client_result.data.clone()
    );
    assert_next_pks!(&relay_result.next_peer_pks, &[client_1_id.as_str()]);

    let client_1_result = checked_call_vm!(
        client_1,
        <_>::default(),
        script,
        "",
        relay_result.data.clone()
    );
    assert_next_pks!(&client_1_result.next_peer_pks, &[client_2_id.as_str()]);

    let client_2_result = checked_call_vm!(
        client_2,
        <_>::default(),
        script,
        "",
        client_1_result.data.clone()
    );
    assert_next_pks!(&client_2_result.next_peer_pks, &[client_3_id.as_str()]);

    let client_3_result = checked_call_vm!(
        client_3,
        <_>::default(),
        script,
        "",
        client_2_result.data.clone()
    );
    assert_next_pks!(&client_3_result.next_peer_pks, &[relay_id.as_str()]);

    let relay_result = checked_call_vm!(
        relay,
        <_>::default(),
        script,
        relay_result.data,
        client_3_result.data.clone()
    );
    // assert_next_pks!(&relay_result.next_peer_pks, &[client_1_id.as_str()]);

    let client_1_result = checked_call_vm!(
        client_1,
        <_>::default(),
        script,
        client_1_result.data,
        relay_result.data.clone()
    );
    // assert_next_pks!(&client_1_result.next_peer_pks, &[client_3_id.as_str()]);

    let client_3_result = checked_call_vm!(
        client_3,
        <_>::default(),
        script,
        client_3_result.data,
        client_1_result.data
    );
    // assert_next_pks!(&client_3_result.next_peer_pks, &[relay_id.as_str()]);

    let relay_result = checked_call_vm!(
        relay,
        <_>::default(),
        script,
        relay_result.data,
        client_3_result.data.clone()
    );
    assert_next_pks!(&relay_result.next_peer_pks, &[client_2_id.as_str()]);

    let client_2_result = checked_call_vm!(
        client_2,
        <_>::default(),
        script,
        client_2_result.data,
        relay_result.data.clone()
    );
    assert_next_pks!(&client_2_result.next_peer_pks, &[client_3_id.as_str()]);

    let client_3_result = checked_call_vm!(
        client_3,
        <_>::default(),
        script,
        client_3_result.data,
        client_2_result.data.clone()
    );
    assert_next_pks!(&client_3_result.next_peer_pks, &[relay_id.as_str()]);

    Data {
        air: script.to_string(),
        prev_data: relay_result.data,
        cur_data: client_3_result.data,
        params_json: hashmap! {
            "comment".to_owned() => "5 peers of network are discovered".to_owned(),
            "particle-id".to_owned() => "".to_owned(),
            "current-peer-id".to_owned() => relay_id,
            "init-peer-id".to_owned() => "".to_owned(),
        },
        call_results: None,
        keypair: bs58::encode(relay_key.as_inner().to_vec()).into_string(),
    }
}
