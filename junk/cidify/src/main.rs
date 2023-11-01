use air_interpreter_data::ExecutedState;
use air_interpreter_sede::Format;
use air_interpreter_signatures::{PeerCidTracker, SignatureStore};
use air_test_utils::key_utils::derive_dummy_keypair;
use air_test_utils::prelude::*;
use std::collections::HashMap;

use clap::Parser;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};

use std::path::PathBuf;

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

#[derive(Parser)]
struct Args {
    data_path: PathBuf,
    calls_path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let mut data: PreCidInterpeterData = read_data(args.data_path);
    let calls: TraceCalls = read_data(args.calls_path);
    let mut calls = calls.into_iter();
    // STUB
    let (keypair, id) = derive_dummy_keypair("init_peer_id");
    let mut peer_id_cache = HashMap::<String, String>::new();
    let mut signature_tracker = PeerCidTracker::new(id);

    let mut cid_state = air::ExecutionCidState::new();

    for elt in &mut data.trace {
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

    data.other_fields.as_object_mut().unwrap().insert(
        "cid_info".to_owned(),
        serde_json::to_value(Into::<air_interpreter_data::CidInfo>::into(cid_state)).unwrap(),
    );
    data.other_fields
        .as_object_mut()
        .unwrap()
        .insert("interpreter_version".to_owned(), json!("0.41.0"));
    let mut ss = <SignatureStore>::new();
    ss.put(
        keypair.public().into(),
        signature_tracker
            .gen_signature("particle_id", &keypair)
            .unwrap(),
    );
    data.other_fields
        .as_object_mut()
        .unwrap()
        .insert("signatures".to_owned(), json!(ss));
    InterpreterDataRepr::get_format::<Value>()
        .to_writer(&data, &mut std::io::stdout())
        .unwrap();
}

fn derive_peer_id(peer_name: &str, peer_id_cache: &mut HashMap<String, String>) -> String {
    peer_id_cache
        .entry(peer_name.to_owned())
        .or_insert_with(|| derive_dummy_keypair(peer_name).1)
        .clone()
}

fn read_data<T: DeserializeOwned>(path: PathBuf) -> T {
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
