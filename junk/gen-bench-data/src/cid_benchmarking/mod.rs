use air::interpreter_version;
use air_interpreter_data::ExecutedState;
use air_interpreter_signatures::KeyPair;
use air_interpreter_signatures::PeerCidTracker;
use air_interpreter_signatures::SignatureStore;
use air_test_utils::key_utils::derive_dummy_keypair;
use air_test_utils::prelude::*;

use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Serialize)]
struct PreCidInterpeterData {
    trace: Vec<Value>,

    #[serde(flatten)]
    other_fields: Value,
}

#[derive(Debug, Deserialize)]
struct TraceCalls {
    cycle: Option<bool>,
    call_info: Vec<CallInfo>,
}

impl TraceCalls {
    fn into_iter(self) -> Box<dyn Iterator<Item = CallInfo>> {
        if self.cycle.unwrap_or_default() {
            Box::new(self.call_info.into_iter().cycle()) as _
        } else {
            Box::new(self.call_info.into_iter()) as _
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
struct CallInfo {
    peer: String,
    service: Option<String>,
    function: Option<String>,
    args: Option<Vec<Value>>,
    json_path: Option<String>,
    kind: Option<Kind>,
}

#[derive(Clone, Debug, Deserialize)]
enum Kind {
    Stream,
    Scalar,
    Unused,
}

fn derive_peer_id(peer_name: &str, peer_id_cache: &mut HashMap<String, String>) -> String {
    peer_id_cache
        .entry(peer_name.to_owned())
        .or_insert_with(|| derive_dummy_keypair(peer_name).1)
        .clone()
}

// input data is always JSON and doesn't depend on InterpreterData representation
fn read_data<T: DeserializeOwned>(path: &str) -> T {
    let inp = std::fs::File::open(path).unwrap();
    serde_json::from_reader(inp).unwrap()
}

fn transform_cid(
    value: Value,
    meta: CallInfo,
    cid_state: &mut air::ExecutionCidState,
    peer_id_state: &mut HashMap<String, String>,
    signature_tracker: &mut PeerCidTracker,
) -> ExecutedState {
    let peer = derive_peer_id(&meta.peer, peer_id_state);

    let mut builder = ExecutedCallBuilder::new(value);
    builder = builder.peer(peer.clone());

    if let Some(service) = meta.service {
        builder = builder.service(service);
    }
    if let Some(function) = meta.function {
        builder = builder.function(function);
    }
    if let Some(args) = meta.args {
        builder = builder.args(args);
    }
    if let Some(json_path) = meta.json_path {
        builder = builder.json_path(json_path);
    }

    match meta.kind {
        Some(Kind::Scalar) | None => {
            let state = builder.scalar_tracked(cid_state);
            let cid = extract_service_result_cid(&state);
            signature_tracker.register(&peer, &cid);
            state
        }
        Some(Kind::Unused) => builder.unused(),
        Some(Kind::Stream) => unimplemented!("no stream in test data"),
    }
}

pub fn cid_benchmarking_data(
    curr_data_filename: &str,
    keypair: &KeyPair,
    peer_id: String,
    particle_id: &str,
) -> Vec<u8> {
    let mut curr_data: PreCidInterpeterData = read_data(curr_data_filename);
    let calls: TraceCalls = read_data("src/cid_benchmarking/simple-calls-info.json");
    let mut calls = calls.into_iter();
    // STUB
    let mut peer_id_cache = HashMap::<String, String>::new();
    let mut signature_tracker = PeerCidTracker::new(peer_id);

    let mut cid_state = air::ExecutionCidState::new();

    for elt in &mut curr_data.trace {
        let obj = elt.as_object_mut().unwrap();
        if let Some(call) = obj.get_mut("call") {
            if let Some(executed) = call.as_object_mut().unwrap().get_mut("executed") {
                if let Some(scalar) = executed.as_object_mut().unwrap().get_mut("scalar") {
                    let call_info = calls.next().expect("More calls than call_info");
                    let state = transform_cid(
                        scalar.take(),
                        call_info,
                        &mut cid_state,
                        &mut peer_id_cache,
                        &mut signature_tracker,
                    );
                    *elt = json!(state);
                }
            }
        }
    }

    curr_data.other_fields.as_object_mut().unwrap().insert(
        "cid_info".to_owned(),
        serde_json::to_value(Into::<air_interpreter_data::CidInfo>::into(cid_state)).unwrap(),
    );
    curr_data.other_fields.as_object_mut().unwrap().insert(
        "interpreter_version".to_owned(),
        json!(interpreter_version()),
    );
    let mut ss = <SignatureStore>::new();
    ss.put(
        keypair.public(),
        signature_tracker
            .gen_signature(particle_id, keypair)
            .unwrap(),
    );
    curr_data
        .other_fields
        .as_object_mut()
        .unwrap()
        .insert("signatures".to_owned(), json!(ss));

    let to_value = serde_json::to_value(curr_data).unwrap();
    let inner_data = serde_json::from_value::<InterpreterData>(to_value).unwrap().serialize().unwrap();

    let data_env = InterpreterDataEnv {
        versions: Versions::new(interpreter_version().clone()),
        inner_data,
    };

    data_env.serialize().unwrap()
}

pub fn cid_benchmarking_long_data(
    keypair: &KeyPair,
    peer_id: String,
    particle_id: &str,
) -> Vec<u8> {
    cid_benchmarking_data(
        "src/cid_benchmarking/anomaly_long.json",
        keypair,
        peer_id,
        particle_id,
    )
}

pub fn cid_benchmarking_big_values_data(
    keypair: &KeyPair,
    peer_id: String,
    particle_id: &str,
) -> Vec<u8> {
    cid_benchmarking_data(
        "src/cid_benchmarking/anomaly_big.json",
        keypair,
        peer_id,
        particle_id,
    )
}
