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

use marine_rs_sdk::marine;

const ADMIN_PEER_PK: &str = "12D3KooWEXNUbCXooUwHrHBbrmjsrpHXoEphPwbjQXEGyzbqKnE1";

fn main() {}

#[marine]
struct AuthResult {
    pub is_authorized: bool,
}

#[marine]
fn is_authorized() -> AuthResult {
    let call_parameters = marine_rs_sdk::get_call_parameters();
    let is_authorized = call_parameters.particle.init_peer_id == ADMIN_PEER_PK;

    AuthResult {
        is_authorized
    }
}
