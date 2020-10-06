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

use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::collections::LinkedList;

/// This file contains defines the same things for both FCE and browser targets.

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum AValue {
    SerdeValue(SerdeValue),
    Iterator(Vec<SerdeValue>, usize),
    Accumulator(LinkedList<SerdeValue>),
}

#[macro_export]
macro_rules! to_svalue {
    ($avalue:expr) => {{
        match $avalue {
            AValue::SerdeValue(value) => Ok(value),
            v => {
                return Err(AquamarineError::IncompatibleAValueType(
                    v.clone(),
                    String::from("SerdeValue"),
                ))
            }
        }
    }};
}

#[macro_export]
macro_rules! to_iterator {
    ($avalue:expr) => {{
        match $avalue {
            AValue::Iterator(values, cursor) => Ok((values, cursor)),
            v => {
                return Err(AquamarineError::IncompatibleAValueType(
                    v.clone(),
                    String::from("Iterator"),
                ))
            }
        }
    }};
}

#[macro_export]
macro_rules! to_acc {
    ($avalue:expr) => {
        match $avalue {
            AValue::Accumulator(acc) => Ok(acc),
            v => {
                return Err(AquamarineError::IncompatibleAValueType(
                    v.clone(),
                    String::from("Accumulator"),
                ))
            }
        }
    };
}

pub(crate) type Result<T> = std::result::Result<T, AquamarineError>;
pub(crate) type AquaData = std::collections::HashMap<String, AValue>;
pub(crate) type SerdeValue = serde_json::Value;
pub(crate) use crate::errors::AquamarineError;
pub(crate) use crate::stepper_outcome::StepperOutcome;

pub(crate) const CALL_SERVICE_SUCCESS: i32 = 0;

#[fluence::fce]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallServiceResult {
    pub ret_code: i32,
    pub result: String,
}
