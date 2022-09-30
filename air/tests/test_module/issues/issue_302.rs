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

use pretty_assertions::assert_eq;

use std::ops::Deref;

#[test]
// test for github.com/fluencelabs/aquavm/issues/302
fn issue_302() {
    let peer_id_1 = "peer_id_1";
    let mut peer_vm_1 = create_avm(echo_call_service(), peer_id_1);
    let peer_id_2 = "peer_id_2";
    let mut peer_vm_2 = create_avm(echo_call_service(), peer_id_2);
    let peer_id_3 = "peer_id_3";
    let mut peer_vm_3 = create_avm(echo_call_service(), peer_id_3);

    let script = f!(r#"
        (new $stream
            (par
                (call "{peer_id_1}" ("" "") [2] $stream)
                (seq
                    (call "{peer_id_2}" ("" "") [1] $stream)
                    (seq
                        (call "{peer_id_3}" ("" "") [0] $stream)
                        (seq
                            (canon "{peer_id_2}" $stream #canon_stream)
                            (call "{peer_id_2}" ("" "") [#canon_stream])
                        )
                    )
                )
            )
        )
    "#);

    let result_1 = checked_call_vm!(peer_vm_2, <_>::default(), &script, "", "");
    let result_2 = checked_call_vm!(peer_vm_1, <_>::default(), &script, "", result_1.data.clone());
    let result_3 = checked_call_vm!(peer_vm_3, <_>::default(), &script, "", result_2.data);
    let result_4 = checked_call_vm!(peer_vm_2, <_>::default(), &script, result_1.data, result_3.data);
    let actual_trace = trace_from_result(&result_4);

    let expected_trace = vec![
        executed_state::par(1, 4),
        executed_state::stream_number(2, 1),
        executed_state::stream_number(1, 0),
        executed_state::stream_number(0, 1),
        executed_state::canon(vec![2.into(), 1.into(), 3.into()]),
        executed_state::scalar(json!([1, 2, 0])),
    ];
    assert_eq!(actual_trace.deref(), expected_trace);
}
