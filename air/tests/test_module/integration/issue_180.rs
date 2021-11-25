/*
 * Copyright 2021 Fluence Labs Limited
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

use fstrings::f;
use fstrings::format_args_f;

#[test]
// test for github.com/fluencelabs/aquavm/issues/180
fn issue_180() {
    let peer_1_id = "peer_1_id";
    let peer_2_id = "peer_2_id";
    let mut peer_1 = create_avm(unit_call_service(), peer_1_id);

    let script = f!(r#"
        (par
            (call "{peer_2_id}" ("" "") [] join_var)
            (seq
                (par
                    (call "{peer_1_id}" ("" "") [join_var]) ;; will trigger VariableNotFound exception
                    (fold join_var iterator ;; will trigger VariableNotFound exception
                        (null)
                    )
                )
                (call "{peer_1_id}" ("" "") []) ;; this should be called only when join_var is set
            )
        )
        "#);

    let peer_1_result = checked_call_vm!(peer_1, "", &script, "", "");
    let trace = trace_from_result(&peer_1_result);
    assert_eq!(trace.len(), 3);
}
