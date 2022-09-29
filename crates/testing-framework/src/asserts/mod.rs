/*
 * Copyright 2022 Fluence Labs Limited
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

pub(crate) mod parser;

use crate::services::JValue;

use air_test_utils::CallServiceResult;
use strum::{AsRefStr, EnumDiscriminants, EnumString};

use std::collections::HashMap;

/// Service definition in the testing framework comment DSL.
#[derive(Debug, PartialEq, Eq, Clone, EnumDiscriminants)]
#[strum_discriminants(derive(AsRefStr, EnumString))]
#[strum_discriminants(name(ServiceTagName))]
pub enum ServiceDefinition {
    /// Simple service that returns same value
    #[strum_discriminants(strum(serialize = "ok"))]
    Ok(JValue),
    /// Simple service that returns same call result (i.e. may return a error)
    #[strum_discriminants(strum(serialize = "err"))]
    Error(CallServiceResult),
    /// Service that may return a new value on subsequent call.  Its keys are either
    /// call number string starting from "0", or "default".
    // TODO We need to return error results too, so we need to define a call result
    // for default and individual errors.
    #[strum_discriminants(strum(serialize = "seq_result"))]
    SeqResult(HashMap<String, JValue>),
    /// Some known service by name: "echo", "unit" (more to follow).
    #[strum_discriminants(strum(serialize = "behaviour"))]
    Behaviour(String),
}
