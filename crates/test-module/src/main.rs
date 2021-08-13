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

#![warn(rust_2018_idioms)]
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]

use marine_rs_sdk::marine;

fn main() {}

#[marine]
pub struct CallServiceResult {
    pub ret_code: i32,
    pub result: String,
}

#[marine]
pub fn call_service(
    service_id: String,
    fn_name: String,
    args: String,
    tetraplets: String,
) -> CallServiceResult {
    println!(
        "call service invoked with:\n  service_id: {}\n  fn_name: {}\n  args: {}\n  tetraples: {}",
        service_id, fn_name, args, tetraplets
    );

    CallServiceResult {
        ret_code: 0,
        result: String::from("[\"result string\"]"),
    }
}
