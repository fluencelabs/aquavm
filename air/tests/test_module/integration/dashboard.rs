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

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

fn parse_peers() -> Vec<String> {
    use csv::ReaderBuilder;

    let data = include_str!("dashboard/peers");

    let mut rdr = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .from_reader(data.as_bytes());

    let mut result = Vec::new();

    while let Some(record) = rdr.records().next() {
        let record = record.unwrap();
        result.push(record.as_slice().to_string());
    }

    result
}

fn into_hashset(peers: Vec<String>) -> HashSet<String> {
    peers.into_iter().collect()
}

fn client_host_function(
    known_peers: Vec<String>,
    client_id: String,
    relay_id: String,
) -> (CallServiceClosure, Rc<RefCell<String>>) {
    let all_info = Rc::new(RefCell::new(String::new()));
    let known_peers = JValue::Array(known_peers.iter().cloned().map(JValue::String).collect::<Vec<_>>());
    let client_id = JValue::String(client_id);
    let relay_id = JValue::String(relay_id);

    let to_ret_value = Box::new(
        move |service_name: &str, function_name: &str, arguments: Vec<String>| -> JValue {
            if !service_name.is_empty() || function_name != "load" || arguments.len() != 1 {
                return JValue::Null;
            }

            match arguments[0].as_str() {
                "relayId" => relay_id.clone(),
                "knownPeers" => known_peers.clone(),
                "clientId" => client_id.clone(),
                _ => JValue::Null,
            }
        },
    );

    let all_info_inner = all_info.clone();
    let host_function: CallServiceClosure = Box::new(move |params| -> CallServiceResult {
        let ret_value = match serde_json::from_value(JValue::Array(params.arguments.clone())) {
            Ok(args) => to_ret_value(params.service_id.as_str(), params.function_name.as_str(), args),
            Err(_) => {
                *all_info_inner.borrow_mut() = JValue::Array(params.arguments).to_string();
                JValue::Null
            }
        };

        CallServiceResult::ok(ret_value)
    });

    (host_function, all_info)
}

fn peer_host_function(
    known_peers: Vec<String>,
    blueprints: Vec<String>,
    modules: Vec<String>,
    interfaces: Vec<String>,
    ident: String,
) -> CallServiceClosure {
    let known_peers = JValue::Array(known_peers.into_iter().map(JValue::String).collect());
    let blueprints = JValue::Array(blueprints.into_iter().map(JValue::String).collect());
    let modules = JValue::Array(modules.into_iter().map(JValue::String).collect());
    let interfaces = JValue::Array(interfaces.into_iter().map(JValue::String).collect());
    let identify = JValue::String(ident.clone());
    let ident = JValue::String(ident);

    let to_ret_value = Box::new(
        move |service_name: &str, function_name: &str, arguments: Vec<&str>| -> JValue {
            match (service_name, function_name, arguments.as_slice()) {
                ("op", "identity", _) => ident.clone(),
                ("op", "identify", _) => identify.clone(),
                ("dist", "get_blueprints", _) => blueprints.clone(),
                ("dist", "get_modules", _) => modules.clone(),
                ("srv", "get_interfaces", _) => interfaces.clone(),
                ("dht", "neighborhood", _) => known_peers.clone(),
                _ => JValue::Null,
            }
        },
    );

    Box::new(move |params| -> CallServiceResult {
        let args: Vec<String> = serde_json::from_value(JValue::Array(params.arguments)).unwrap();
        let t_args = args.iter().map(|s| s.as_str()).collect::<Vec<_>>();
        let ret_value = to_ret_value(params.service_id.as_str(), params.function_name.as_str(), t_args);

        CallServiceResult::ok(ret_value)
    })
}

#[rustfmt::skip]
fn create_peer_host_function(peer_id: String, known_peer_ids: Vec<String>) -> CallServiceClosure {
    let relay_blueprints = (0..=2).map(|id| f!("{peer_id}_blueprint_{id}")).collect::<Vec<_>>();
    let relay_modules = (0..=2).map(|id| f!("{peer_id}_module_{id}")).collect::<Vec<_>>();
    let relay_interfaces = (0..=2).map(|id| f!("{peer_id}_interface_{id}")).collect::<Vec<_>>();
    let relay_ident = f!("{peer_id}_ident");

    peer_host_function(
        known_peer_ids,
        relay_blueprints,
        relay_modules,
        relay_interfaces,
        relay_ident,
    )
}

struct AVMState {
    vm: TestRunner,
    peer_id: String,
    prev_result: Vec<u8>,
}

#[test]
fn dashboard() {
    let script = include_str!("./scripts/dashboard.air");

    let known_peer_ids = parse_peers();
    let client_id = "client_id".to_string();
    let relay_id = "relay_id".to_string();

    let (host_function, all_info) = client_host_function(known_peer_ids.clone(), client_id.clone(), relay_id.clone());

    let mut client = create_avm(host_function, client_id.clone());
    let mut relay = create_avm(
        create_peer_host_function(relay_id.clone(), known_peer_ids.clone()),
        relay_id.clone(),
    );

    let mut known_peers = known_peer_ids
        .iter()
        .cloned()
        .map(|peer_id| {
            let vm = create_avm(
                create_peer_host_function(peer_id.clone(), known_peer_ids.clone()),
                peer_id.clone(),
            );
            AVMState {
                vm,
                peer_id,
                prev_result: vec![],
            }
        })
        .collect::<Vec<_>>();

    let test_params = TestRunParameters::from_init_peer_id(client_id.clone());

    // -> client 1
    let client_1_result = checked_call_vm!(client, test_params.clone(), script, "", "");
    let next_peer_pks = into_hashset(client_1_result.next_peer_pks);
    let mut all_peer_pks = into_hashset(known_peer_ids);
    all_peer_pks.insert(relay_id.clone());
    assert_eq!(next_peer_pks, all_peer_pks);

    // client 1 -> relay 1
    let relay_1_result = checked_call_vm!(relay, test_params.clone(), script, client_1_result.data.clone(), "");
    let next_peer_pks = into_hashset(relay_1_result.next_peer_pks.clone());
    all_peer_pks.remove(&relay_id);
    all_peer_pks.insert(client_id.clone());
    assert_eq!(next_peer_pks, all_peer_pks);

    // relay 1 -> client 2
    let client_2_result = checked_call_vm!(
        client,
        test_params.clone(),
        script,
        client_1_result.data.clone(),
        relay_1_result.data.clone()
    );
    assert!(client_2_result.next_peer_pks.is_empty());
    assert_eq!(
        *all_info.borrow(),
        String::from(
            r#"["relay_id","relay_id_ident",["relay_id_interface_0","relay_id_interface_1","relay_id_interface_2"],["relay_id_blueprint_0","relay_id_blueprint_1","relay_id_blueprint_2"],["relay_id_module_0","relay_id_module_1","relay_id_module_2"]]"#
        )
    );

    let mut relay_2_result = relay_1_result.clone();
    let mut client_3_result = client_2_result;

    // peers 1 -> relay 2 -> client 3
    for avm in known_peers.iter_mut() {
        let prev_result = std::mem::take(&mut avm.prev_result);
        let known_peer_result = checked_call_vm!(
            avm.vm,
            test_params.clone(),
            script,
            prev_result,
            client_1_result.data.clone()
        );
        assert_eq!(known_peer_result.next_peer_pks, vec![relay_id.clone()]);

        avm.prev_result = known_peer_result.data;

        relay_2_result = checked_call_vm!(
            relay,
            test_params.clone(),
            script,
            relay_2_result.data.clone(),
            avm.prev_result.clone()
        );
        assert_eq!(relay_2_result.next_peer_pks, vec![client_id.clone()]);

        client_3_result = checked_call_vm!(
            client,
            test_params.clone(),
            script,
            client_3_result.data.clone(),
            relay_2_result.data.clone()
        );
        assert!(client_3_result.next_peer_pks.is_empty());
        assert_eq!(
            *all_info.borrow(),
            format!(
                r#"["{peer_id}","{peer_id}_ident",["{peer_id}_interface_0","{peer_id}_interface_1","{peer_id}_interface_2"],["{peer_id}_blueprint_0","{peer_id}_blueprint_1","{peer_id}_blueprint_2"],["{peer_id}_module_0","{peer_id}_module_1","{peer_id}_module_2"]]"#,
                peer_id = avm.peer_id
            )
        )
    }

    all_peer_pks.remove(&client_id);
    all_peer_pks.insert(relay_id.to_string());

    let mut relay_3_result = relay_2_result;
    let mut client_4_result = client_3_result;

    // peers 2 -> relay 3 -> client 4
    for avm in known_peers.iter_mut() {
        let prev_result = std::mem::take(&mut avm.prev_result);
        let known_peer_result = checked_call_vm!(
            avm.vm,
            test_params.clone(),
            script,
            prev_result,
            relay_1_result.data.clone()
        );
        all_peer_pks.remove(&avm.peer_id);
        let next_peer_pks = into_hashset(known_peer_result.next_peer_pks.clone());
        assert_eq!(next_peer_pks, all_peer_pks);

        all_peer_pks.insert(avm.peer_id.clone());

        avm.prev_result = known_peer_result.data;

        relay_3_result = checked_call_vm!(
            relay,
            test_params.clone(),
            script,
            relay_3_result.data.clone(),
            avm.prev_result.clone()
        );
        assert_eq!(relay_3_result.next_peer_pks, vec![client_id.clone()]);

        // client -> peers -> relay -> client
        client_4_result = checked_call_vm!(
            client,
            test_params.clone(),
            script,
            client_4_result.data.clone(),
            relay_3_result.data.clone()
        );
        assert!(client_4_result.next_peer_pks.is_empty());
        assert_eq!(
            *all_info.borrow(),
            format!(
                r#"["{peer_id}","{peer_id}_ident",["{peer_id}_interface_0","{peer_id}_interface_1","{peer_id}_interface_2"],["{peer_id}_blueprint_0","{peer_id}_blueprint_1","{peer_id}_blueprint_2"],["{peer_id}_module_0","{peer_id}_module_1","{peer_id}_module_2"]]"#,
                peer_id = avm.peer_id
            )
        )
    }

    let mut relay_4_result = relay_3_result;
    let mut client_5_result = client_4_result;

    // peers 2 -> peers 3 -> relay 4 -> client 5
    for i in 0..known_peers.len() {
        for j in 0..known_peers.len() {
            if known_peers[i].peer_id == known_peers[j].peer_id {
                continue;
            }

            let prev_data = known_peers[j].prev_result.clone();
            let data = known_peers[i].prev_result.clone();
            let known_peer_i_j_result =
                checked_call_vm!(known_peers[j].vm, test_params.clone(), script, prev_data, data);
            assert_eq!(known_peer_i_j_result.next_peer_pks, vec![relay_id.clone()]);

            known_peers[j].prev_result = known_peer_i_j_result.data;

            relay_4_result = checked_call_vm!(
                relay,
                test_params.clone(),
                script,
                relay_4_result.data.clone(),
                known_peers[j].prev_result.clone()
            );
            assert_eq!(relay_4_result.next_peer_pks, vec![client_id.clone()]);

            // client -> peers -> relay -> client
            client_5_result = checked_call_vm!(
                client,
                test_params.clone(),
                script,
                client_5_result.data.clone(),
                relay_4_result.data.clone()
            );
            assert!(client_5_result.next_peer_pks.is_empty());
            assert_eq!(
                *all_info.borrow(),
                format!(
                    r#"["{peer_id}","{peer_id}_ident",["{peer_id}_interface_0","{peer_id}_interface_1","{peer_id}_interface_2"],["{peer_id}_blueprint_0","{peer_id}_blueprint_1","{peer_id}_blueprint_2"],["{peer_id}_module_0","{peer_id}_module_1","{peer_id}_module_2"]]"#,
                    peer_id = known_peers[j].peer_id
                )
            );
        }
    }
}
