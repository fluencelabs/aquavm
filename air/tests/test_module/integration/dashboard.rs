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

use futures::FutureExt;
use futures::StreamExt;

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
) -> (CallServiceClosure<'static>, Rc<RefCell<String>>) {
    let all_info = Rc::new(RefCell::new(String::new()));
    let known_peers = serde_json::Value::Array(known_peers.iter().cloned().map(Into::into).collect());
    let client_id = serde_json::Value::String(client_id);
    let relay_id = serde_json::Value::String(relay_id);

    let to_ret_value = Box::new(
        move |service_name: &str, function_name: &str, arguments: Vec<String>| -> serde_json::Value {
            if !service_name.is_empty() || function_name != "load" || arguments.len() != 1 {
                return serde_json::Value::Null;
            }

            match arguments[0].as_str() {
                "relayId" => relay_id.clone(),
                "knownPeers" => known_peers.clone(),
                "clientId" => client_id.clone(),
                _ => serde_json::Value::Null,
            }
        },
    );

    let all_info_inner = all_info.clone();
    let host_function: CallServiceClosure = Box::new(move |params| {
        let ret_value = match serde_json::from_value(serde_json::Value::Array(params.arguments.clone())) {
            Ok(args) => to_ret_value(params.service_id.as_str(), params.function_name.as_str(), args),
            Err(_) => {
                *all_info_inner.borrow_mut() = serde_json::Value::Array(params.arguments).to_string();
                serde_json::Value::Null
            }
        };

        let result = CallServiceResult::ok(ret_value);
        async move { result }.boxed_local()
    });

    (host_function, all_info)
}

fn peer_host_function(
    known_peers: Vec<String>,
    blueprints: Vec<String>,
    modules: Vec<String>,
    interfaces: Vec<String>,
    ident: String,
) -> CallServiceClosure<'static> {
    let known_peers = serde_json::Value::Array(known_peers.into_iter().map(serde_json::Value::String).collect());
    let blueprints = serde_json::Value::Array(blueprints.into_iter().map(serde_json::Value::String).collect());
    let modules = serde_json::Value::Array(modules.into_iter().map(serde_json::Value::String).collect());
    let interfaces = serde_json::Value::Array(interfaces.into_iter().map(serde_json::Value::String).collect());
    let identify = serde_json::Value::String(ident.clone());
    let ident = serde_json::Value::String(ident);

    let to_ret_value = Box::new(
        move |service_name: &str, function_name: &str, arguments: Vec<&str>| -> serde_json::Value {
            match (service_name, function_name, arguments.as_slice()) {
                ("op", "identity", _) => ident.clone(),
                ("op", "identify", _) => identify.clone(),
                ("dist", "get_blueprints", _) => blueprints.clone(),
                ("dist", "get_modules", _) => modules.clone(),
                ("srv", "get_interfaces", _) => interfaces.clone(),
                ("dht", "neighborhood", _) => known_peers.clone(),
                _ => serde_json::Value::Null,
            }
        },
    );

    Box::new(move |params| {
        let args: Vec<String> = serde_json::from_value(serde_json::Value::Array(params.arguments)).unwrap();
        let t_args = args.iter().map(|s| s.as_str()).collect::<Vec<_>>();
        let ret_value = to_ret_value(params.service_id.as_str(), params.function_name.as_str(), t_args);

        let result = CallServiceResult::ok(ret_value);
        async move { result }.boxed_local()
    })
}

#[rustfmt::skip]
fn create_peer_host_function(peer_id: String, known_peer_ids: Vec<String>) -> CallServiceClosure<'static> {
    let relay_blueprints = (0..=2).map(|id| format!("{peer_id}_blueprint_{id}")).collect::<Vec<_>>();
    let relay_modules = (0..=2).map(|id| format!("{peer_id}_module_{id}")).collect::<Vec<_>>();
    let relay_interfaces = (0..=2).map(|id| format!("{peer_id}_interface_{id}")).collect::<Vec<_>>();
    let relay_ident = format!("{peer_id}_ident");

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

#[tokio::test]
async fn dashboard() {
    let script = include_str!("./scripts/dashboard.air");

    let known_peer_ids = parse_peers();
    let client_id = "client_id".to_string();
    let relay_id = "relay_id".to_string();

    let (host_function, all_info) = client_host_function(known_peer_ids.clone(), client_id.clone(), relay_id.clone());

    let mut client = create_avm(host_function, client_id.clone()).await;
    let mut relay = create_avm(
        create_peer_host_function(relay_id.clone(), known_peer_ids.clone()),
        relay_id.clone(),
    )
    .await;

    let mut known_peers = futures::stream::iter(known_peer_ids.iter().cloned())
        .then(|peer_id| async {
            let vm = create_avm(
                create_peer_host_function(peer_id.clone(), known_peer_ids.clone()),
                peer_id.clone(),
            )
            .await;
            AVMState {
                vm,
                peer_id,
                prev_result: vec![],
            }
        })
        .collect::<Vec<_>>()
        .await;

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
