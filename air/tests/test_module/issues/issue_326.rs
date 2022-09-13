/*
 * Copyright 2022 Fluence Labs Limited
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
// test for github.com/fluencelabs/aquavm/issues/326
fn issue_326() {
    let peer_id_1 = "peer_id_1";
    let mut peer_vm_1 = create_avm(echo_call_service(), peer_id_1);
    let peer_id_2 = "peer_id_2";
    let mut peer_vm_2 = create_avm(echo_call_service(), peer_id_2);

    let script = f!(r#"
        (seq
            (seq
                (seq
                     (ap "{peer_id_1}" scalar_1)
                     (ap scalar_1 $stream)
                )
                (seq
                     (ap "{peer_id_2}" scalar_2)
                     (ap scalar_2 $stream)
                )
            )
            (seq
                (fold $stream peer_id
                     (par
                         (call peer_id ("" "") [1]) ; behaviour = echo
                         (next peer_id)
                     )
                )
                (call "{peer_id_2}" ("" "") [1]) ; behaviour = echo
            )
        )
    "#);

    let result_1 = checked_call_vm!(peer_vm_1, <_>::default(), &script, "", "");
    let result_2 = checked_call_vm!(peer_vm_2, <_>::default(), &script, "", "");
    let result_3 = checked_call_vm!(peer_vm_2, <_>::default(), &script, result_2.data, result_1.data);

    print_trace(&result_3, "");
}
