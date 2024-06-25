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

use air_test_utils::prelude::*;

#[tokio::test]
// test for github.com/fluencelabs/aquavm/issues/348
async fn issue_348() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let mut peer_vm_1 = create_avm(echo_call_service(), vm_peer_id_1).await;
    let vm_peer_id_2 = "vm_peer_id_2";
    let mut peer_vm_2 = create_avm(echo_call_service(), vm_peer_id_2).await;
    let vm_peer_id_3 = "vm_peer_id_3";
    let mut peer_vm_3 = create_avm(echo_call_service(), vm_peer_id_3).await;

    let script = format!(
        r#"
        (seq
            (seq
                (ap 1 $inner)
                (seq
                    (call "{vm_peer_id_2}" ("op" "noop") [1])
                    (ap 2 $inner)
                )
            )
            (seq
                (canon "{vm_peer_id_2}" $inner #inner)
                (seq
                    (call "{vm_peer_id_3}" ("op" "noop") [1])
                    (call "{vm_peer_id_2}" ("op" "noop") [1])
                )
            )
        )
    "#
    );

    let result11 = checked_call_vm!(peer_vm_1, <_>::default(), &script, "", "");
    let result21 = checked_call_vm!(peer_vm_2, <_>::default(), &script, "", result11.data);
    let result31 = checked_call_vm!(peer_vm_3, <_>::default(), &script, "", result21.data.clone());
    let _result22 = checked_call_vm!(peer_vm_2, <_>::default(), &script, result21.data, result31.data);
}
