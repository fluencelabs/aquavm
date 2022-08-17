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
fn canon_moves_execution_flow() {
    let mut vm = create_avm(echo_call_service(), "A");
    let peer_id_1 = "peer_id_1";
    let peer_id_2 = "peer_id_2";

    let script = f!(r#"
            (par
                (call "{peer_id_1}" ("" "") [] $stream)
                (canon "{peer_id_2}" $stream #canon_stream)
            )"#);

    let result = checked_call_vm!(vm, <_>::default(), script, "", "");

    assert_next_pks!(&result.next_peer_pks, &[peer_id_1, peer_id_2]);
}

#[test]
fn basic_canon() {
    let mut vm = create_avm(echo_call_service(), "A");
    let mut set_variable_vm = create_avm(
        set_variable_call_service(json!(["1", "2", "3", "4", "5"])),
        "set_variable",
    );

    let script = r#"
            (seq
                (call "set_variable" ("" "") [] Iterable)
                (seq
                    (fold Iterable i
                        (seq
                            (call "A" ("" "") [i] $stream)
                            (next i)
                        )
                    )
                    (canon "A" $stream #canon_stream)
                )
            )"#;

    let result = checked_call_vm!(set_variable_vm, <_>::default(), script, "", "");
    let result = checked_call_vm!(vm, <_>::default(), script, "", result.data);
    let actual_state = &trace_from_result(&result)[6.into()];

    let expected_state = executed_state::canon(vec![1.into(), 2.into(), 3.into(), 4.into(), 5.into()]);
    assert_eq!(actual_state, &expected_state);
}
