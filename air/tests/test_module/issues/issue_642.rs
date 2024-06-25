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
async fn issue_642() {
    let peer_id_1 = "peer_id_1";
    let peer_id_2 = "peer_id_2";
    let peer_id_3 = "peer_id_3";

    let mut vm_1 = create_avm(unit_call_service(), peer_id_1).await;
    let mut vm_2 = create_avm(unit_call_service(), peer_id_2).await;
    let mut vm_3 = create_avm(unit_call_service(), peer_id_3).await;

    let script = format!(
        r#"
    (seq
        (seq
            (seq
                 (ap %init_peer_id% $peers)
                 (ap "{peer_id_2}" $peers)
            )
            (canon "{peer_id_3}" $peers #peers)
        )
        (seq
            (fold #peers peer
                 (par
                     (seq
                         (call peer ("" "") [])
                         (new $stream
                             (seq
                                  (call peer ("" "") [] $stream)
                                  (seq
                                      (call peer ("" "") [] $stream)
                                      (ap 1 $stream)
                                  )
                             )
                         )
                     )
                    (next peer)
                 )
            )
            (call %init_peer_id% ("" "") [])
        )
    )
    "#
    );

    let parameters = TestRunParameters::from_init_peer_id(peer_id_1);
    let result_1_1 = checked_call_vm!(vm_1, parameters.clone(), &script, "", "");
    let result_3_1 = checked_call_vm!(vm_3, parameters.clone(), &script, "", result_1_1.data.clone());
    let result_2 = checked_call_vm!(vm_2, parameters.clone(), &script, "", result_3_1.data.clone());
    let result_1_2 = checked_call_vm!(
        vm_1,
        parameters.clone(),
        &script,
        result_1_1.data.clone(),
        result_3_1.data.clone()
    );
    let _ = checked_call_vm!(
        vm_1,
        parameters.clone(),
        &script,
        result_1_2.data.clone(),
        result_2.data
    ); // crashes
}
