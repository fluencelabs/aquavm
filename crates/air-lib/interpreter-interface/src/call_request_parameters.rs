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

use air_interpreter_sede::define_simple_representation;
use air_interpreter_sede::derive_serialized_type;
use air_interpreter_sede::Format;
use air_interpreter_sede::FromSerialized;
use air_interpreter_sede::JsonFormat;
use air_interpreter_sede::Representation;

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

pub type CallArgumentsFormat = JsonFormat;
pub type TetrapletsFormat = JsonFormat;
pub type CallRequestsFormat = JsonFormat;

define_simple_representation! {
    CallArgumentsRepr,
    Vec<serde_json::Value>,
    CallArgumentsFormat,
    SerializedCallArguments
}

pub type CallArgumentsDeserializeError = <CallArgumentsRepr as Representation>::DeserializeError;

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
