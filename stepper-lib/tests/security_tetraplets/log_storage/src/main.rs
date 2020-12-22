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

use fluence::fce;

#[fce]
pub struct CallServiceResult {
    pub ret_code: i32,
    pub result: String,
}

fn main() {}

#[fce]
fn delete(is_authorized: Vec<i32>, _record_id: String) -> String {
    let call_parameters = fluence::get_call_parameters();
    let tetraplets = call_parameters.tetraplets;
    let tetraplet = &tetraplets[0];

    if tetraplet[0].json_path != "$.is_authorized" {
        return String::from("invalid json path in tetraplet");
    }

    match is_authorized[0] {
        1 => String::from("Ok"),
        _ => String::from("not authorized"),
    }
}
