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

use air_interpreter_interface::INTERPRETER_SUCCESS;
use air_test_utils::prelude::*;

#[test]
// test for github.com/fluencelabs/aquavm/issues/331
fn issue_331() {
    let peer_id_1 = "peer_id_1";
    let mut peer_vm_1 = create_avm(set_variable_call_service(json!("")), peer_id_1);

    let script = f!(r#"
        (new $array-inline
            (seq
                (seq
                    (seq
                        (seq
                            (seq
                                (new $status
                                    (canon %init_peer_id% $status #status)
                                )
                                (call %init_peer_id% ("op" "array_length") [#status] array_length)
                            )
                            (ap array_length $array-inline)
                        )
                        (seq
                            (ap 2 $num)
                            (canon %init_peer_id% $num #num_canon)
                        )
                    )
                    (ap #num_canon.$.[0]! $array-inline)
                )
                (canon %init_peer_id% $array-inline #array-inline-0)
           )
        )
    "#);

    let parameters = TestRunParameters::new(peer_id_1, 1, 1);
    let result = call_vm!(peer_vm_1, parameters, &script, "", "");
    assert_eq!(result.ret_code, INTERPRETER_SUCCESS);
}
