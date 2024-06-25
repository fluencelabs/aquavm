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
