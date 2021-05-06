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
use air_test_utils::create_aqua_vm;
use air_test_utils::IValue;
use air_test_utils::NEVec;
use air_test_utils::{CallServiceClosure, AVM};

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

type JValue = serde_json::Value;

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

    return result;
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
        move |service_name: &str, function_name: &str, arguments: Vec<&str>| -> JValue {
            match (service_name, function_name, arguments.as_slice()) {
                ("", "load", &["relayId"]) => relay_id.clone(),
                ("", "load", &["knownPeers"]) => known_peers.clone(),
                ("", "load", &["clientId"]) => client_id.clone(),
                _ => JValue::Null,
            }
        },
    );

    let all_info_inner = all_info.clone();
    let host_function: CallServiceClosure = Box::new(move |_, args| -> Option<IValue> {
        let service_name = match &args[0] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };

        let function_name = match &args[1] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };

        let function_args = match &args[2] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };

        let ret_value = match serde_json::from_str(function_args) {
            Ok(args) => to_ret_value(service_name.as_str(), function_name.as_str(), args),
            Err(_) => {
                *all_info_inner.borrow_mut() = function_args.clone();
                JValue::Null
            }
        };

        Some(IValue::Record(
            NEVec::new(vec![IValue::S32(0), IValue::String(ret_value.to_string())]).unwrap(),
        ))
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

    Box::new(move |_, args| -> Option<IValue> {
        let service_name = match &args[0] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };

        let function_name = match &args[1] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };

        let function_args = match &args[2] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };

        let args: Vec<&str> = serde_json::from_str(function_args).unwrap();

        let ret_value = to_ret_value(service_name.as_str(), function_name.as_str(), args);

        Some(IValue::Record(
            NEVec::new(vec![IValue::S32(0), IValue::String(ret_value.to_string())]).unwrap(),
        ))
    })
}

#[rustfmt::skip]
fn create_peer_host_function(peer_id: String, known_peer_ids: Vec<String>) -> CallServiceClosure {
    let relay_blueprints = (0..=2).map(|id| format!("{}_blueprint_{}", peer_id, id)).collect::<Vec<_>>();
    let relay_modules = (0..=2).map(|id| format!("{}_module_{}", peer_id, id)).collect::<Vec<_>>();
    let relay_interfaces = (0..=2).map(|id| format!("{}_interface_{}", peer_id, id)).collect::<Vec<_>>();
    let relay_ident = format!("{}_ident", peer_id);

    peer_host_function(
        known_peer_ids,
        relay_blueprints,
        relay_modules,
        relay_interfaces,
        relay_ident,
    )
}

struct AquaVMState {
    vm: AVM,
    peer_id: String,
    prev_result: Vec<u8>,
}

#[test]
fn dashboard() {
    let script = include_str!("./scripts/dashboard.clj");

    let known_peer_ids = parse_peers();
    let client_id = String::from("client_id");
    let relay_id = String::from("relay_id");

    let (host_function, all_info) = client_host_function(known_peer_ids.clone(), client_id.clone(), relay_id.clone());

    let mut client = create_aqua_vm(host_function, client_id.clone());
    let mut relay = create_aqua_vm(
        create_peer_host_function(relay_id.clone(), known_peer_ids.clone()),
        relay_id.clone(),
    );

    let mut known_peers = known_peer_ids
        .iter()
        .cloned()
        .map(|peer_id| {
            let vm = create_aqua_vm(
                create_peer_host_function(peer_id.clone(), known_peer_ids.clone()),
                peer_id.clone(),
            );
            AquaVMState {
                vm,
                peer_id,
                prev_result: vec![],
            }
        })
        .collect::<Vec<_>>();

    // -> client 1
    let client_1_res = call_vm!(client, client_id.clone(), script.clone(), "", "");
    let next_peer_pks = into_hashset(client_1_res.next_peer_pks);
    let mut all_peer_pks = into_hashset(known_peer_ids.clone());
    all_peer_pks.insert(relay_id.clone());
    assert_eq!(next_peer_pks, all_peer_pks);

    // client 1 -> relay 1
    let relay_1_res = call_vm!(relay, client_id.clone(), script.clone(), client_1_res.data.clone(), "");
    let next_peer_pks = into_hashset(relay_1_res.next_peer_pks.clone());
    all_peer_pks.remove(&relay_id);
    all_peer_pks.insert(client_id.clone());
    assert_eq!(next_peer_pks, all_peer_pks);

    // relay 1 -> client 2
    let client_2_res = call_vm!(
        client,
        client_id.clone(),
        script.clone(),
        client_1_res.data.clone(),
        relay_1_res.data.clone()
    );
    assert!(client_2_res.next_peer_pks.is_empty());
    assert_eq!(
        *all_info.borrow(),
        String::from(
            r#"["relay_id","relay_id_ident",["relay_id_interface_0","relay_id_interface_1","relay_id_interface_2"],["relay_id_blueprint_0","relay_id_blueprint_1","relay_id_blueprint_2"],["relay_id_module_0","relay_id_module_1","relay_id_module_2"]]"#
        )
    );

    let mut relay_2_res = relay_1_res.clone();
    let mut client_3_res = client_2_res.clone();

    // peers 1 -> relay 2 -> client 3
    for aqua_vm in known_peers.iter_mut() {
        let prev_result = std::mem::replace(&mut aqua_vm.prev_result, vec![]);
        let known_peer_res = call_vm!(
            aqua_vm.vm,
            client_id.clone(),
            script.clone(),
            prev_result,
            client_1_res.data.clone()
        );
        assert_eq!(known_peer_res.next_peer_pks, vec![relay_id.clone()]);

        aqua_vm.prev_result = known_peer_res.data;

        relay_2_res = call_vm!(
            relay,
            client_id.clone(),
            script.clone(),
            relay_2_res.data.clone(),
            aqua_vm.prev_result.clone()
        );
        assert_eq!(relay_2_res.next_peer_pks, vec![client_id.clone()]);

        client_3_res = call_vm!(
            client,
            client_id.clone(),
            script.clone(),
            client_3_res.data.clone(),
            relay_2_res.data.clone()
        );
        assert!(client_3_res.next_peer_pks.is_empty());
        assert_eq!(
            *all_info.borrow(),
            format!(
                r#"["{peer_id}","{peer_id}_ident",["{peer_id}_interface_0","{peer_id}_interface_1","{peer_id}_interface_2"],["{peer_id}_blueprint_0","{peer_id}_blueprint_1","{peer_id}_blueprint_2"],["{peer_id}_module_0","{peer_id}_module_1","{peer_id}_module_2"]]"#,
                peer_id = aqua_vm.peer_id
            )
        )
    }

    all_peer_pks.remove(&client_id);
    all_peer_pks.insert(relay_id.clone());

    let mut relay_3_res = relay_2_res.clone();
    let mut client_4_res = client_3_res.clone();

    // peers 2 -> relay 3 -> client 4
    for aqua_vm in known_peers.iter_mut() {
        let prev_result = std::mem::replace(&mut aqua_vm.prev_result, vec![]);
        let known_peer_res = call_vm!(
            aqua_vm.vm,
            client_id.clone(),
            script.clone(),
            prev_result,
            relay_1_res.data.clone()
        );
        all_peer_pks.remove(&aqua_vm.peer_id);
        let next_peer_pks = into_hashset(known_peer_res.next_peer_pks.clone());
        assert_eq!(next_peer_pks, all_peer_pks);

        all_peer_pks.insert(aqua_vm.peer_id.clone());

        aqua_vm.prev_result = known_peer_res.data;

        relay_3_res = call_vm!(
            relay,
            client_id.clone(),
            script.clone(),
            relay_3_res.data.clone(),
            aqua_vm.prev_result.clone()
        );
        assert_eq!(relay_3_res.next_peer_pks, vec![client_id.clone()]);

        // client -> peers -> relay -> client
        client_4_res = call_vm!(
            client,
            client_id.clone(),
            script.clone(),
            client_4_res.data.clone(),
            relay_3_res.data.clone()
        );
        assert!(client_4_res.next_peer_pks.is_empty());
        assert_eq!(
            *all_info.borrow(),
            format!(
                r#"["{peer_id}","{peer_id}_ident",["{peer_id}_interface_0","{peer_id}_interface_1","{peer_id}_interface_2"],["{peer_id}_blueprint_0","{peer_id}_blueprint_1","{peer_id}_blueprint_2"],["{peer_id}_module_0","{peer_id}_module_1","{peer_id}_module_2"]]"#,
                peer_id = aqua_vm.peer_id
            )
        )
    }

    let mut relay_4_res = relay_3_res.clone();
    let mut client_5_res = client_4_res.clone();

    // peers 2 -> peers 3 -> relay 4 -> client 5
    for i in 0..known_peers.len() {
        for j in 0..known_peers.len() {
            if known_peers[i].peer_id == known_peers[j].peer_id {
                continue;
            }

            let prev_data = known_peers[j].prev_result.clone();
            let data = known_peers[i].prev_result.clone();
            let known_peer_i_j_res = call_vm!(known_peers[j].vm, client_id.clone(), script.clone(), prev_data, data);
            assert_eq!(known_peer_i_j_res.next_peer_pks, vec![relay_id.clone()]);

            known_peers[j].prev_result = known_peer_i_j_res.data;

            relay_4_res = call_vm!(
                relay,
                client_id.clone(),
                script.clone(),
                relay_4_res.data.clone(),
                known_peers[j].prev_result.clone()
            );
            assert_eq!(relay_4_res.next_peer_pks, vec![client_id.clone()]);

            // client -> peers -> relay -> client
            client_5_res = call_vm!(
                client,
                client_id.clone(),
                script.clone(),
                client_5_res.data.clone(),
                relay_4_res.data.clone()
            );
            assert!(client_5_res.next_peer_pks.is_empty());
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
