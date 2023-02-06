use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize, Debug, Serialize)]
pub struct PreCidInterpeterData {
    trace: Vec<serde_json::Value>,

    #[serde(flatten)]
    other_fields: serde_json::Value,
}

fn main() {
    let stdin = std::io::stdin();
    let mut data: PreCidInterpeterData =
        serde_json::from_reader(stdin).expect("Expect to be readable");
    let mut values = air_interpreter_data::CidTracker::<Value>::new();
    for elt in &mut data.trace {
        let obj = elt.as_object_mut().unwrap();
        if let Some(call) = obj.get_mut("call") {
            if let Some(executed) = call.as_object_mut().unwrap().get_mut("executed") {
                if let Some(scalar) = executed.as_object_mut().unwrap().get_mut("scalar") {
                    let cid = values.record_value(scalar.clone()).expect("Expect to CID");
                    *scalar = json!(cid);
                }
            }
        }
    }
    data.other_fields.as_object_mut().unwrap().insert(
        "cid_info".to_owned(),
        json!({
            "value_store": Into::<air_interpreter_data::CidStore<_>>::into(values),
            "tetraplet_store": {},
            "canon_store": {},
        }),
    );
    data.other_fields
        .as_object_mut()
        .unwrap()
        .insert("interpreter_version".to_owned(), json!("0.35.1"));
    serde_json::to_writer(std::io::stdout(), &data).unwrap();
}
