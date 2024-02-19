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

/// This stores soft limits triggering flags.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SoftLimitsTriggering {
    pub air_size_limit_exceeded: bool,
    pub particle_size_limit_exceeded: bool,
    pub call_result_size_limit_exceeded: bool,
}

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

    /// This flag signals that AIR script size exceeds the limit.
    pub air_size_limit_exceeded: bool,

    /// This flag signals that particle data size exceeds the limit.
    pub particle_size_limit_exceeded: bool,

    /// This flag signals that call result size exceeds the limit.
    pub call_result_size_limit_exceeded: bool,
}

impl SoftLimitsTriggering {
    pub fn new(
        air_size_limit_exceeded: bool,
        particle_size_limit_exceeded: bool,
        call_result_size_limit_exceeded: bool,
    ) -> Self {
        Self {
            air_size_limit_exceeded,
            particle_size_limit_exceeded,
            call_result_size_limit_exceeded,
        }
    }

    pub fn is_triggered(&self) -> bool {
        self.air_size_limit_exceeded
            || self.particle_size_limit_exceeded
            || self.call_result_size_limit_exceeded
    }
}

impl InterpreterOutcome {
    pub fn new(
        ret_code: i64,
        error_message: String,
        data: Vec<u8>,
        next_peer_pks: Vec<String>,
        call_requests: SerializedCallRequests,
        soft_limits_triggering: SoftLimitsTriggering,
    ) -> Self {
        let call_requests = call_requests.into();
        Self {
            ret_code,
            error_message,
            data,
            next_peer_pks,
            call_requests,
            air_size_limit_exceeded: soft_limits_triggering.air_size_limit_exceeded,
            particle_size_limit_exceeded: soft_limits_triggering.particle_size_limit_exceeded,
            call_result_size_limit_exceeded: soft_limits_triggering.call_result_size_limit_exceeded,
        }
    }
}

#[cfg(feature = "marine")]
impl InterpreterOutcome {
    pub fn from_ivalue(ivalue: IValue) -> Result<Self, String> {
        const OUTCOME_FIELDS_COUNT: usize = 8;

        let mut record_values = try_as_record(ivalue)?.into_vec();
        if record_values.len() != OUTCOME_FIELDS_COUNT {
            return Err(format!(
                "expected InterpreterOutcome struct with {OUTCOME_FIELDS_COUNT} fields, got {record_values:?}"
            ));
        }

        let air_size_limit_exceeded =
            try_as_boolean(record_values.pop().unwrap(), "air_size_limit_exceeded")?;
        let particle_size_limit_exceeded =
            try_as_boolean(record_values.pop().unwrap(), "particle_size_limit_exceeded")?;
        let call_result_size_limit_exceeded = try_as_boolean(
            record_values.pop().unwrap(),
            "call_result_size_limit_exceeded",
        )?;
        let call_requests = try_as_byte_vec(record_values.pop().unwrap(), "call_requests")?;
        let next_peer_pks = try_as_string_vec(record_values.pop().unwrap(), "next_peer_pks")?;
        let data = try_as_byte_vec(record_values.pop().unwrap(), "data")?;
        let error_message = try_as_string(record_values.pop().unwrap(), "error_message")?;
        let ret_code = try_as_i64(record_values.pop().unwrap(), "ret_code")?;
        let soft_limits_triggering = SoftLimitsTriggering::new(
            air_size_limit_exceeded,
            particle_size_limit_exceeded,
            call_result_size_limit_exceeded,
        );

        let outcome = Self::new(
            ret_code,
            error_message,
            data,
            next_peer_pks,
            call_requests.into(),
            soft_limits_triggering,
        );

        Ok(outcome)
    }
}

#[cfg(feature = "marine")]
use fluence_it_types::ne_vec::NEVec;

use crate::SerializedCallRequests;

#[cfg(feature = "marine")]
fn try_as_record(ivalue: IValue) -> Result<NEVec<IValue>, String> {
    match ivalue {
        IValue::Record(record_values) => Ok(record_values),
        v => Err(format!("expected record for InterpreterOutcome, got {v:?}")),
    }
}

#[cfg(feature = "marine")]
fn try_as_i64(ivalue: IValue, field_name: &str) -> Result<i64, String> {
    match ivalue {
        IValue::S64(value) => Ok(value),
        v => Err(format!("expected an i64 for {field_name}, got {v:?}")),
    }
}

#[cfg(feature = "marine")]
pub fn try_as_string(ivalue: IValue, field_name: &str) -> Result<String, String> {
    match ivalue {
        IValue::String(value) => Ok(value),
        v => Err(format!("expected a string for {field_name}, got {v:?}")),
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
                    v => Err(format!("expected a byte, got {v:?}")),
                })
                .collect();
            array?
        }
        IValue::ByteArray(array) => array,
        v => return Err(format!("expected a Vec<u8> for {field_name}, got {v:?}")),
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
                    v => Err(format!("expected string for next_peer_pks, got {v:?}")),
                })
                .collect::<Result<Vec<String>, _>>()?;

            Ok(array)
        }
        v => Err(format!("expected an array for {field_name}, got {v:?}")),
    }
}

#[cfg(feature = "marine")]
fn try_as_boolean(ivalue: IValue, field_name: &str) -> Result<bool, String> {
    match ivalue {
        IValue::Boolean(value) => Ok(value),
        v => Err(format!("expected a bool for {field_name}, got {v:?}")),
    }
}
