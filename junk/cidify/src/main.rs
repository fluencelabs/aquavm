use air_interpreter_data::ExecutedState;

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

    let mut cid_state = air::ExecutionCidState::new();

    for elt in &mut data.trace {
        let obj = elt.as_object_mut().unwrap();
        if let Some(call) = obj.get_mut("call") {
            if let Some(executed) = call.as_object_mut().unwrap().get_mut("executed") {
                if let Some(scalar) = executed.as_object_mut().unwrap().get_mut("scalar") {
                    let call_info = calls.next().expect("More calls than call_info");
                    let state = transform_cid(scalar.take(), call_info, &mut cid_state);
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
        .insert("interpreter_version".to_owned(), json!("0.35.1"));
    serde_json::to_writer(std::io::stdout(), &data).unwrap();
}

fn read_data<T: DeserializeOwned>(path: PathBuf) -> T {
    let inp = std::fs::File::open(path).unwrap();
    serde_json::from_reader(inp).unwrap()
}

fn transform_cid(
    value: Value,
    meta: CallInfo,
    cid_state: &mut air::ExecutionCidState,
) -> ExecutedState {
    use air_test_utils::executed_state::ExecutedCallBuilder;

    let mut builder = ExecutedCallBuilder::new(value);
    builder = builder.peer(meta.peer);

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
        Some(Kind::Scalar) | None => builder.scalar_tracked(cid_state),
        Some(Kind::Unused) => builder.unused(),
        Some(Kind::Stream) => unimplemented!("no stream in test data"),
    }
}
