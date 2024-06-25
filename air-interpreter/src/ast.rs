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

use air::parser::parse;

/// Parse AIR script and return it as minified JSON
pub fn ast(script: String) -> String {
    let do_parse = || -> std::result::Result<_, Box<dyn std::error::Error>> {
        let ast = parse(&script)?;
        serde_json::to_string(&ast).map_err(Into::into)
    };

    match do_parse() {
        Ok(json) => json,
        Err(err) => err.to_string(),
    }
}
