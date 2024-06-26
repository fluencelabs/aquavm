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

use air::PreparationError;
use air_test_utils::prelude::*;

#[tokio::test]
async fn invalid_air() {
    let vm_peer_id = "some_peer_id";
    let mut vm = create_avm(unit_call_service(), vm_peer_id).await;

    let script = r#"(seq )"#;

    let result = call_vm!(vm, <_>::default(), script, "", "");

    let error_message = air_parser::parse(script).expect_err("air parser should fail on this script");
    let expected_error = PreparationError::AIRParseError(error_message);
    assert!(check_error(&result, expected_error));
}
