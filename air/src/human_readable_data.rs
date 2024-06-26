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

use crate::preparation_step::check_version_compatibility;

use air_interpreter_data::InterpreterData;
use air_interpreter_data::InterpreterDataEnvelope;
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
