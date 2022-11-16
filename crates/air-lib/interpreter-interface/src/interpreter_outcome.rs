/*
 * Copyright 2021 Fluence Labs Limited
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

#[cfg(feature = "marine")]
use marine_rs_sdk::marine;

#[cfg(feature = "marine")]
use fluence_it_types::IValue;
use serde::Deserialize;
use serde::Serialize;

pub const INTERPRETER_SUCCESS: i64 = 0;

/// Describes a result returned at the end of the interpreter execution_step.
#[cfg_attr(feature = "marine", marine)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InterpreterOutcome {
    /// A return code, where INTERPRETER_SUCCESS means success.
    pub ret_code: i64,

    /// Contains error message if ret_code != INTERPRETER_SUCCESS.
    pub error_message: String,

    /// Contains script data that should be preserved in an executor of this interpreter
    /// regardless of ret_code value.
    pub data: Vec<u8>,

    /// Public keys of peers that should receive data.
    pub next_peer_pks: Vec<String>,

    /// Collected parameters of all met call instructions that could be executed on a current peer.
    pub call_requests: Vec<u8>,

    /// IPLD CID of the data field.
    pub cid: String,
}

impl InterpreterOutcome {
    pub fn new(
        ret_code: i64,
        error_message: String,
        data: Vec<u8>,
        next_peer_pks: Vec<String>,
        call_requests: Vec<u8>,
    ) -> Self {
        let cid = json_data_cid(&data);

        Self {
            ret_code,
            error_message,
            data,
            next_peer_pks,
            call_requests,
            cid,
        }
    }
}

// TODO we might refactor this to `SerializationFormat` trait
// that both transform data to binary/text form (be it JSON, CBOR or something else)
// and produces CID too
fn json_data_cid(data: &[u8]) -> String {
    use cid::Cid;
    use multihash::{Code, MultihashDigest};

    // the Sha2_256 is current IPFS default hash
    let digest = Code::Sha2_256.digest(data);
    // seems to be better than RAW_CODEC = 0x55
    const JSON_CODEC: u64 = 0x0200;

    let cid = Cid::new_v1(JSON_CODEC, digest);
    cid.to_string()
}

#[cfg(feature = "marine")]
impl InterpreterOutcome {
    pub fn from_ivalue(ivalue: IValue) -> Result<Self, String> {
        const OUTCOME_FIELDS_COUNT: usize = 5;

        let mut record_values = try_as_record(ivalue)?.into_vec();
        if record_values.len() != OUTCOME_FIELDS_COUNT {
            return Err(format!(
                "expected InterpreterOutcome struct with {} fields, got {:?}",
                OUTCOME_FIELDS_COUNT, record_values
            ));
        }

        let call_requests = try_as_byte_vec(record_values.pop().unwrap(), "call_requests")?;
        let next_peer_pks = try_as_string_vec(record_values.pop().unwrap(), "next_peer_pks")?;
        let data = try_as_byte_vec(record_values.pop().unwrap(), "data")?;
        let error_message = try_as_string(record_values.pop().unwrap(), "error_message")?;
        let ret_code = try_as_i64(record_values.pop().unwrap(), "ret_code")?;

        let outcome = Self::new(ret_code, error_message, data, next_peer_pks, call_requests);

        Ok(outcome)
    }
}

#[cfg(feature = "marine")]
use fluence_it_types::ne_vec::NEVec;

#[cfg(feature = "marine")]
fn try_as_record(ivalue: IValue) -> Result<NEVec<IValue>, String> {
    match ivalue {
        IValue::Record(record_values) => Ok(record_values),
        v => Err(format!(
            "expected record for InterpreterOutcome, got {:?}",
            v
        )),
    }
}

#[cfg(feature = "marine")]
fn try_as_i64(ivalue: IValue, field_name: &str) -> Result<i64, String> {
    match ivalue {
        IValue::S64(value) => Ok(value),
        v => Err(format!("expected an i64 for {}, got {:?}", field_name, v)),
    }
}

#[cfg(feature = "marine")]
fn try_as_string(ivalue: IValue, field_name: &str) -> Result<String, String> {
    match ivalue {
        IValue::String(value) => Ok(value),
        v => Err(format!("expected a string for {}, got {:?}", field_name, v)),
    }
}

#[cfg(feature = "marine")]
fn try_as_byte_vec(ivalue: IValue, field_name: &str) -> Result<Vec<u8>, String> {
    let byte_vec = match ivalue {
        IValue::Array(array) => {
            let array: Result<Vec<_>, _> = array
                .into_iter()
                .map(|v| match v {
                    IValue::U8(byte) => Ok(byte),
                    v => Err(format!("expected a byte, got {:?}", v)),
                })
                .collect();
            array?
        }
        IValue::ByteArray(array) => array,
        v => {
            return Err(format!(
                "expected a Vec<u8> for {}, got {:?}",
                field_name, v
            ))
        }
    };

    Ok(byte_vec)
}

#[cfg(feature = "marine")]
fn try_as_string_vec(ivalue: IValue, field_name: &str) -> Result<Vec<String>, String> {
    match ivalue {
        IValue::Array(ar_values) => {
            let array = ar_values
                .into_iter()
                .map(|v| match v {
                    IValue::String(str) => Ok(str),
                    v => Err(format!("expected string for next_peer_pks, got {:?}", v)),
                })
                .collect::<Result<Vec<String>, _>>()?;

            Ok(array)
        }
        v => Err(format!("expected an array for {}, got {:?}", field_name, v)),
    }
}
