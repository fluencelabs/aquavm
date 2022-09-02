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

use air::parser::parse;
use air_parser_utils::Interner;

/// Parse AIR script and return it as minified JSON
pub fn ast(script: String) -> String {
    let do_parse = || -> std::result::Result<_, Box<dyn std::error::Error>> {
        let mut interner = Interner::new();
        let ast = parse(&script, &mut interner)?;
        serde_json::to_string(&ast).map_err(Into::into)
    };

    match do_parse() {
        Ok(json) => json,
        Err(err) => err.to_string(),
    }
}
