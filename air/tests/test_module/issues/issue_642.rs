/*
 * Copyright 2023 Fluence Labs Limited
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

use air_test_utils::prelude::*;

#[test]
#[ignore] // will be resolved in https://github.com/fluencelabs/aquavm/pull/621
fn issue_642() {
    let peer_id_1 = "peer_id_1";
    let peer_id_2 = "peer_id_2";
    let peer_id_3 = "peer_id_3";

    let mut vm_1 = create_avm(unit_call_service(), peer_id_1);
    let mut vm_2 = create_avm(unit_call_service(), peer_id_2);
    let mut vm_3 = create_avm(unit_call_service(), peer_id_3);

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
