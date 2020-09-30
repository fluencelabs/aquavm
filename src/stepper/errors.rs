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

use super::StepperOutcome;

use serde_sexpr::Error as SExprError;

use std::convert::Into;
use std::error::Error;

#[derive(Debug)]
pub enum AquamarineError {
    /// FaaS errors.
    ParseError(SExprError),

    /// Aquamarine result deserialization errors.
    ExecutionError(String),
}

impl Error for AquamarineError {}

impl std::fmt::Display for AquamarineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            AquamarineError::ParseError(err) => write!(f, "{}", err),
            AquamarineError::ExecutionError(err_msg) => write!(f, "{}", err_msg),
        }
    }
}

impl From<SExprError> for AquamarineError {
    fn from(err: SExprError) -> Self {
        AquamarineError::ParseError(err)
    }
}

impl From<std::convert::Infallible> for AquamarineError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}

impl Into<StepperOutcome> for AquamarineError {
    fn into(self) -> StepperOutcome {
        match self {
            AquamarineError::ParseError(err) => StepperOutcome {
                ret_code: 1,
                data: format!("{}", err),
                next_peer_pks: vec![],
            },
            AquamarineError::ExecutionError(err) => StepperOutcome {
                ret_code: 2,
                data: err,
                next_peer_pks: vec![],
            },
        }
    }
}
