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

fn main() {}

#[marine]
fn delete(is_authorized: bool, _record_id: String) -> String {
    let call_parameters = marine_rs_sdk::get_call_parameters();
    let tetraplets = call_parameters.tetraplets;
    let tetraplet = &tetraplets[0];

    if tetraplet[0].lens != "$.is_authorized" {
        return String::from("invalid lambda in tetraplet");
    }

    if is_authorized {
        String::from("Ok")
    } else {
        String::from("not authorized")
    }
}
