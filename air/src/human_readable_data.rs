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

use crate::preparation_step::check_version_compatibility;

use air_interpreter_data::{InterpreterData, InterpreterDataEnvelope};
use serde_json::json;

use std::error::Error as StdError;

pub fn to_human_readable_data(data: Vec<u8>) -> Result<String, Box<dyn StdError>> {
    let envelope = InterpreterDataEnvelope::try_from_slice(&data)?;

    check_version_compatibility(&envelope.versions)?;

    let data = InterpreterData::try_from_slice(&envelope.inner_data)?;

    // TODO convert value store strings to JSON
    let envelope_json = json!({
        "versions": envelope.versions,
        "data": data,
    });

    // it may produce quite a big string (whitespaces, escaping, etc), but this function
    // is intended to be executed on user machine, not on chain or in a cloud.
    Ok(serde_json::to_string_pretty(&envelope_json)?)
}
