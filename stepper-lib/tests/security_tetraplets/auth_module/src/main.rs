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
use fluence::WasmLoggerBuilder;

#[fce]
pub struct CallServiceResult {
    pub ret_code: i32,
    pub result: String,
}

const ADMIN_PEER_PK: &str = "12D3KooWEXNUbCXooUwHrHBbrmjsrpHXoEphPwbjQXEGyzbqKnE1";

fn main() {
    WasmLoggerBuilder::new().build().unwrap();
}

#[fce]
pub fn call_service(service_id: String, fn_name: String, args: String, tetraplets: String) -> CallServiceResult {
    println!(
        "call service called with {} {} {} {}",
        service_id, fn_name, args, tetraplets
    );

    CallServiceResult {
        ret_code: 0,
        result: String::new(),
    }
}

fn is_authorized(user_peer_pk: &str) -> bool {
    user_peer_pk ==
}
