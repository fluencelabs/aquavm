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

use fluence::fce;

use serde::Deserialize;
use serde::Serialize;
use fluence_it_types::IValue;

pub const STEPPER_SUCCESS: i32 = 0;

/// Describes a result returned at the end of the stepper execution.
#[fce]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StepperOutcome {
    /// A return code, where STEPPER_SUCCESS means success.
    pub ret_code: i32,

    /// Contains error message if ret_code != STEPPER_SUCCESS.
    pub error_message: String,

    /// Contains script data that should be preserved in an executor of this stepper
    /// regardless of ret_code value.
    pub data: Vec<u8>,

    /// Public keys of peers that should receive data.
    pub next_peer_pks: Vec<String>,
}

impl StepperOutcome {
    pub fn from_ivalues(mut ivalues: Vec<IValue>) -> Result<Self, String> {
        const OUTCOME_FIELDS_COUNT: usize = 4;

        let record_values = match ivalues.remove(0) {
            IValue::Record(record_values) => record_values,
            v => return Err(format!("expected record for StepperOutcome, got {:?}", v)),
        };

        let mut record_values = record_values.into_vec();
        if record_values.len() != OUTCOME_FIELDS_COUNT {
            return Err(format!(
                "expected StepperOutcome struct with {} fields, got {:?}",
                OUTCOME_FIELDS_COUNT, record_values
            ));
        }

        let ret_code = match record_values.remove(0) {
            IValue::S32(ret_code) => ret_code,
            v => return Err(format!("expected i32 for ret_code, got {:?}", v)),
        };

        let error_message = match record_values.remove(0) {
            IValue::String(str) => str,
            v => return Err(format!("expected string for data, got {:?}", v)),
        };

        let data = match record_values.remove(0) {
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
            v => return Err(format!("expected Vec<u8> for data, got {:?}", v)),
        };

        let next_peer_pks = match record_values.remove(0) {
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
            v => Err(format!("expected array for next_peer_pks, got {:?}", v)),
        }?;

        let outcome = Self {
            ret_code,
            error_message,
            data,
            next_peer_pks,
        };

        Ok(outcome)
    }
}
