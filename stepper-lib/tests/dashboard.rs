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

use aqua_test_utils::create_aqua_vm;
use aqua_test_utils::set_variables_call_service;
use aqua_test_utils::unit_call_service;
use aqua_test_utils::{call_vm, echo_string_call_service};
use aquamarine_vm::vec1::Vec1;
use aquamarine_vm::HostExportedFunc;
use aquamarine_vm::IValue;
use stepper_lib::CallEvidencePath;

use serde::Deserialize;
use serde_json::json;

use std::rc::Rc;

type JValue = serde_json::Value;

#[derive(Deserialize)]
struct CallEvidencePathPrintable(pub CallEvidencePath);

impl std::fmt::Display for CallEvidencePathPrintable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for state in self.0.iter() {
            writeln!(f, "  {}", state)?;
        }

        Ok(())
    }
}

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

fn client_host_function(
    known_peers: Vec<String>,
    client_id: String,
    relay_id: String,
) -> (HostExportedFunc, Rc<String>) {
    let all_info = Rc::new(String::new());
    let known_peers = JValue::Array(known_peers.iter().cloned().map(JValue::String).collect::<Vec<_>>());
    let client_id = JValue::String(client_id);
    let relay_id = JValue::String(relay_id);

    let to_ret_value = Box::new(
        move |service_name: &str, function_name: &str, arguments: Vec<&str>| -> JValue {
            println!(
                "to_ret_value of {} peer called with {} {} {:?}",
                client_id, service_name, function_name, arguments
            );

            match (service_name, function_name, arguments.as_slice()) {
                ("", "load", &["relayId"]) => relay_id.clone(),
                ("", "load", &["knownPeers"]) => known_peers.clone(),
                ("", "load", &["clientId"]) => client_id.clone(),
                ("event", "all_info", _) => JValue::Null,
                _ => JValue::Null,
            }
        },
    );

    let host_function: HostExportedFunc = Box::new(move |_, args| -> Option<IValue> {
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
            Vec1::new(vec![IValue::S32(0), IValue::String(ret_value.to_string())]).unwrap(),
        ))
    });

    (host_function, all_info)
}

fn peer_host_function(
    peer_id: String,
    known_peers: Vec<String>,
    blueprints: Vec<String>,
    modules: Vec<String>,
    interfaces: Vec<String>,
    ident: String,
) -> HostExportedFunc {
    let known_peers = JValue::Array(known_peers.into_iter().map(JValue::String).collect());
    let blueprints = JValue::Array(blueprints.into_iter().map(JValue::String).collect());
    let modules = JValue::Array(modules.into_iter().map(JValue::String).collect());
    let interfaces = JValue::Array(interfaces.into_iter().map(JValue::String).collect());
    let identify = JValue::String(String::from("identify"));
    let ident = JValue::String(ident);

    let to_ret_value = Box::new(
        move |service_name: &str, function_name: &str, arguments: Vec<&str>| -> JValue {
            /*
            println!(
                "to_ret_value of {} peer called with {} {} {:?}",
                peer_id, service_name, function_name, arguments
            );
             */

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
            Vec1::new(vec![IValue::S32(0), IValue::String(ret_value.to_string())]).unwrap(),
        ))
    })
}

#[rustfmt::skip]
fn create_peer_host_function(peer_id: String, known_peer_ids: Vec<String>) -> HostExportedFunc {
    let relay_blueprints = (0..=2).map(|id| format!("{}_blueprint_{}", peer_id, id)).collect::<Vec<_>>();
    let relay_modules = (0..=2).map(|id| format!("{}_module_{}", peer_id, id)).collect::<Vec<_>>();
    let relay_interfaces = (0..=2).map(|id| format!("{}_interface_{}", peer_id, id)).collect::<Vec<_>>();
    let relay_ident = format!("{}_ident", peer_id);

    peer_host_function(
        peer_id,
        known_peer_ids,
        relay_blueprints,
        relay_modules,
        relay_interfaces,
        relay_ident,
    )
}

#[test]
fn dashboard() {
    let script = include_str!("dashboard/script.clj");

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
        .map(|id| create_aqua_vm(create_peer_host_function(id.clone(), known_peer_ids.clone()), id))
        .collect::<Vec<_>>();

    let client_1_res = call_vm!(client, client_id.clone(), script.clone(), "", "");
    let client_1_res_path: CallEvidencePathPrintable = serde_json::from_str(&client_1_res.data).unwrap();

    println!("-> client 1:");
    // println!(" call evidence path:\n{}", client_1_res_path);
    println!(" next peers pks: {:?}\n\n", client_1_res.next_peer_pks);

    // client 1 -> relay 1
    let relay_1_res = call_vm!(relay, client_id.clone(), script.clone(), client_1_res.data.clone(), "");
    let relay_1_res_path: CallEvidencePathPrintable = serde_json::from_str(&relay_1_res.data).unwrap();

    println!("client 1 -> relay 1");
    // println!(" call evidence path:\n{}", relay_1_res_path);
    println!(" next peers pks: {:?}\n\n", relay_1_res.next_peer_pks);

    // client 1 -> peers
    let mut known_peers_1_results = Vec::new();
    for (id, known_peer) in known_peers.iter_mut().enumerate() {
        let known_peer_res = call_vm!(
            known_peer,
            client_id.clone(),
            script.clone(),
            client_1_res.data.clone(),
            ""
        );
        let known_peer_res_path: CallEvidencePathPrintable = serde_json::from_str(&known_peer_res.data).unwrap();

        println!("client 1 -> {} ", &known_peer_ids[id]);
        // println!(" call evidence path:\n{}", known_peer_res_path);
        println!(" next peers pks: {:?}\n\n", known_peer_res.next_peer_pks);

        known_peers_1_results.push(known_peer_res);
    }

    // env_logger::init();

    // client 1 -> peers
    // client 1 -> relay 1 -> peers
    let mut known_peers_2_results = Vec::new();
    for (id, known_peer) in known_peers.iter_mut().enumerate() {
        let known_peer_res = call_vm!(
            known_peer,
            client_id.clone(),
            script.clone(),
            known_peers_1_results[id].data.clone(),
            relay_1_res.data.clone()
        );
        let known_peer_res_path: CallEvidencePathPrintable = serde_json::from_str(&known_peer_res.data).unwrap();

        println!("client 1 -> peers");
        println!("client 1 -> relay 1 -> {} ", &known_peer_ids[id]);
        // println!(" call evidence path:\n{}", known_peer_res_path);
        println!(" next peers pks: {:?}\n\n", known_peer_res.next_peer_pks);

        known_peers_2_results.push(known_peer_res);
    }
}
