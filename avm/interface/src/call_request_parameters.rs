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

use super::JValue;
use crate::CallSeDeErrors;

use air_interpreter_interface::SerializedCallRequests;
use polyplets::SecurityTetraplet;
use serde::Deserialize;
use serde::Serialize;

use std::collections::HashMap;

pub type CallRequests = HashMap<u32, CallRequestParams>;

/// Contains arguments of a call instruction and all other necessary information
/// required for calling a service.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallRequestParams {
    /// Id of a service that should be called.
    pub service_id: String,

    /// Name of a function from service identified by service_id that should be called.
    pub function_name: String,

    /// Arguments that should be passed to the function.
    pub arguments: Vec<JValue>,

    /// Tetraplets that should be passed to the service.
    pub tetraplets: Vec<Vec<SecurityTetraplet>>,
}

impl CallRequestParams {
    pub fn new(
        service_id: impl Into<String>,
        function_name: impl Into<String>,
        arguments: Vec<JValue>,
        tetraplets: Vec<Vec<SecurityTetraplet>>,
    ) -> Self {
        Self {
            service_id: service_id.into(),
            function_name: function_name.into(),
            arguments,
            tetraplets,
        }
    }

    pub(crate) fn from_raw(
        call_params: air_interpreter_interface::CallRequestParams,
    ) -> Result<Self, CallSeDeErrors> {
        use air_interpreter_interface::CallArgumentsRepr;
        use air_interpreter_interface::TetrapletsRepr;
        use air_interpreter_sede::FromSerialized;

        // TODO that's different JValue!
        let arguments: Vec<JValue> = CallArgumentsRepr
            .deserialize(&call_params.arguments)
            .map_err(|de_error| CallSeDeErrors::CallParamsArgsDeFailed {
                call_params: call_params.clone(),
                de_error,
            })?;

        let tetraplets: Vec<Vec<SecurityTetraplet>> = TetrapletsRepr
            .deserialize(&call_params.tetraplets)
            .map_err(|de_error| CallSeDeErrors::CallParamsTetrapletsDeFailed {
                call_params: call_params.clone(),
                de_error,
            })?;

        let call_params = Self {
            service_id: call_params.service_id,
            function_name: call_params.function_name,
            arguments,
            tetraplets,
        };

        Ok(call_params)
    }
}

pub(crate) fn from_raw_call_requests(
    raw_call_params: SerializedCallRequests,
) -> Result<CallRequests, CallSeDeErrors> {
    use air_interpreter_interface::CallRequestsRepr;
    use air_interpreter_sede::FromSerialized;

    let call_requests: air_interpreter_interface::CallRequests =
        match CallRequestsRepr.deserialize(&raw_call_params) {
            Ok(requests) => requests,
            Err(error) => {
                return Err(CallSeDeErrors::CallRequestsDeError {
                    raw_call_request: raw_call_params,
                    error,
                })
                .map_err(Into::into)
            }
        };

    call_requests
        .into_iter()
        .map(|(call_id, call_params)| -> Result<_, _> {
            let call_params = CallRequestParams::from_raw(call_params)?;
            Ok((call_id, call_params))
        })
        .collect::<Result<_, _>>()
}
