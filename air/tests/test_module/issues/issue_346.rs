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

use air_interpreter_interface::INTERPRETER_SUCCESS;
use air_test_utils::prelude::*;

#[tokio::test]
// test for github.com/fluencelabs/aquavm/issues/346
async fn issue_346() {
    let vm_peer_id = "peer_id_1";
    let mut peer_vm = create_avm(echo_call_service(), vm_peer_id).await;

    let script = format!(
        r#"
        (par
            (call "unknown_peer" ("" "") [] $stream) ; to make validator happy
            (xor
                (canon "{vm_peer_id}" $stream #canon_stream) ; it returns a catchable error
                (call "{vm_peer_id}" ("" "") [""])
            )
        )
    "#
    );

    let result = call_vm!(peer_vm, <_>::default(), &script, "", "");
    assert_eq!(result.ret_code, INTERPRETER_SUCCESS);
}
