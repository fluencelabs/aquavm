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

use air_interpreter_sede::define_simple_representation;
use air_interpreter_sede::derive_serialized_type;
use air_interpreter_sede::Format;
use air_interpreter_sede::FromSerialized;
use air_interpreter_sede::MsgPackFormat;
use air_interpreter_sede::MsgPackMultiformat;
use air_interpreter_sede::Representation;
use air_interpreter_value::JValue;

use marine_call_parameters::SecurityTetraplet;
#[cfg(feature = "marine")]
use marine_rs_sdk::marine;
use serde::Deserialize;
use serde::Serialize;

use std::collections::HashMap;
use std::rc::Rc;

pub type CallRequests = HashMap<u32, CallRequestParams>;

derive_serialized_type!(SerializedCallArguments);
derive_serialized_type!(SerializedTetraplets);
derive_serialized_type!(SerializedCallRequests);

pub type CallArgumentsFormat = MsgPackFormat;
pub type TetrapletsFormat = MsgPackFormat;
pub type CallRequestsFormat = MsgPackMultiformat;

define_simple_representation! {
    CallArgumentsRepr,
    Vec<JValue>,
    CallArgumentsFormat,
    SerializedCallArguments
}

pub type CallArgumentsDeserializeError = <CallArgumentsRepr as Representation>::DeserializeError;

impl FromSerialized<Vec<serde_json::Value>> for CallArgumentsRepr {
    fn deserialize(&self, repr: &[u8]) -> Result<Vec<serde_json::Value>, Self::DeserializeError> {
        Self.get_format().from_slice(repr)
    }
}

define_simple_representation! {
    TetrapletsRepr,
    // additional implementation for Vec<Vec<SecurityTetraplet>> is defined below
    // TODO allow this macro to define implementations for multiple types
    Vec<Vec<Rc<SecurityTetraplet>>>,
    TetrapletsFormat,
    SerializedTetraplets
}

pub type TetrapletDeserializeError = <TetrapletsRepr as Representation>::DeserializeError;

define_simple_representation! {
    CallRequestsRepr,
    CallRequests,
    CallRequestsFormat,
    SerializedCallRequests
}

pub type CallRequestsDeserializeError = <CallRequestsRepr as Representation>::DeserializeError;

impl FromSerialized<Vec<Vec<SecurityTetraplet>>> for TetrapletsRepr {
    fn deserialize(
        &self,
        repr: &[u8],
    ) -> Result<Vec<Vec<SecurityTetraplet>>, Self::DeserializeError> {
        Self.get_format().from_slice(repr)
    }
}

/// Contains arguments of a call instruction and all other necessary information
/// required for calling a service.
#[cfg_attr(feature = "marine", marine)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallRequestParams {
    /// Id of a service that should be called.
    pub service_id: String,

    /// Name of a function from service identified by service_id that should be called.
    pub function_name: String,

    /// Serialized to JSON string Vec<JValue> of arguments that should be passed to a service.
    pub arguments: SerializedCallArguments,

    /// Serialized to JSON string Vec<Vec<SecurityTetraplet>> that should be passed to a service.
    pub tetraplets: SerializedTetraplets,
}

impl CallRequestParams {
    pub fn new(
        service_id: String,
        function_name: String,
        arguments: SerializedCallArguments,
        tetraplets: SerializedTetraplets,
    ) -> Self {
        Self {
            service_id,
            function_name,
            arguments,
            tetraplets,
        }
    }
}
