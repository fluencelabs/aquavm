/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

mod behavior;
mod json;
pub(crate) mod parser;

use crate::services::JValue;

use air_test_utils::{CallRequestParams, CallServiceResult};
use serde_json::json;
use strum::{AsRefStr, EnumDiscriminants, EnumString};

use std::{borrow::Cow, cell::Cell, collections::HashMap};

use self::behavior::Behavior;

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
    #[strum_discriminants(strum(serialize = "seq_ok"))]
    SeqOk {
        call_number_seq: Cell<usize>,
        call_map: HashMap<String, JValue>,
    },
    #[strum_discriminants(strum(serialize = "seq_error"))]
    SeqError {
        call_number_seq: Cell<usize>,
        call_map: HashMap<String, CallServiceResult>,
    },
    /// Some known service by name: "echo", "unit" (more to follow).
    #[strum_discriminants(strum(serialize = "behaviour"))]
    Behaviour(Behavior),
    /// Same services as defined by the enum element above with dbg! applied to the arguments.
    #[strum_discriminants(strum(serialize = "dbg_behaviour"))]
    DbgBehaviour(Behavior),
    /// Maps first argument to a value
    #[strum_discriminants(strum(serialize = "map"))]
    Map(HashMap<String, JValue>),
}

impl ServiceDefinition {
    pub fn ok(value: JValue) -> Self {
        Self::Ok(value)
    }

    pub fn error(value: CallServiceResult) -> Self {
        Self::Error(value)
    }

    pub fn seq_ok(call_map: HashMap<String, JValue>) -> Self {
        Self::SeqOk {
            call_number_seq: 0.into(),
            call_map,
        }
    }

    pub fn seq_error(call_map: HashMap<String, CallServiceResult>) -> Self {
        Self::SeqError {
            call_number_seq: 0.into(),
            call_map,
        }
    }

    pub fn behaviour(name: Behavior) -> Self {
        Self::Behaviour(name)
    }

    pub fn map(map: HashMap<String, JValue>) -> Self {
        Self::Map(map)
    }

    pub async fn call(&self, params: CallRequestParams) -> CallServiceResult {
        match self {
            ServiceDefinition::Ok(ok) => CallServiceResult::ok(ok.clone()),
            ServiceDefinition::Error(call_result) => call_result.clone(),
            ServiceDefinition::SeqOk {
                ref call_number_seq,
                call_map,
            } => call_seq_ok(call_number_seq, call_map),
            ServiceDefinition::SeqError {
                ref call_number_seq,
                call_map,
            } => call_seq_error(call_number_seq, call_map),
            ServiceDefinition::Behaviour(name) => name.call(params).await,
            ServiceDefinition::DbgBehaviour(name) => dbg!(name.call(dbg!(params)).await),
            ServiceDefinition::Map(map) => call_map_service(map, params),
        }
    }
}

fn call_seq_ok(
    call_number_seq: &Cell<usize>,
    call_map: &HashMap<String, serde_json::Value>,
) -> CallServiceResult {
    let call_number = call_number_seq.get();
    let call_num_str = call_number.to_string();
    call_number_seq.set(call_number + 1);

    let value = call_map
        .get(&call_num_str)
        .or_else(|| call_map.get("default"))
        .unwrap_or_else(|| {
            panic!(
                r#"neither key {:?} nor "default" key not found in the {:?}"#,
                call_num_str, call_map
            )
        })
        .clone();
    CallServiceResult::ok(value)
}

fn call_seq_error(
    call_number_seq: &Cell<usize>,
    call_map: &HashMap<String, CallServiceResult>,
) -> CallServiceResult {
    let call_number = call_number_seq.get();
    let call_num_str = call_number.to_string();
    call_number_seq.set(call_number + 1);

    call_map
        .get(&call_num_str)
        .or_else(|| call_map.get("default"))
        .unwrap_or_else(|| {
            panic!(
                r#"neither key {:?} nor "default" key not found in the {:?}"#,
                call_num_str, call_map
            )
        })
        .clone()
}

fn call_map_service(
    map: &HashMap<String, serde_json::Value>,
    args: CallRequestParams,
) -> CallServiceResult {
    let key = args
        .arguments
        .get(0)
        .expect("At least one arugment expected");
    // Strings are looked up by value, other objects -- by their string representation.
    //
    // For example, `"key"` is looked up as `"key"`, `5` is looked up as `"5"`, `["test"]` is looked up
    // as `"[\"test\"]"`.
    let key_repr = match key {
        serde_json::Value::String(s) => Cow::Borrowed(s.as_str()),
        val => Cow::Owned(val.to_string()),
    };
    CallServiceResult::ok(json!(map.get(key_repr.as_ref()).cloned()))
}
