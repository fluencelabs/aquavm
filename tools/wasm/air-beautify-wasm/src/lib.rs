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

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn beautify(air_script: String) -> Result<String, JsError> {
    let mut output = vec![];
    air_beautifier::beautify(&air_script, &mut output, true)?;
    Ok(unsafe { String::from_utf8_unchecked(output) })
}

#[wasm_bindgen]
pub fn beautify_raw(air_script: String) -> Result<String, JsError> {
    let mut output = vec![];
    air_beautifier::beautify(&air_script, &mut output, false)?;
    Ok(unsafe { String::from_utf8_unchecked(output) })
}
