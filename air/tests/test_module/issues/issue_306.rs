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
// test for github.com/fluencelabs/aquavm/issues/306
fn issue_306() {
    let peer_id_1 = "peer_id_1";
    let mut peer_vm_1 = create_avm(echo_call_service(), peer_id_1);

    let script = f!(r#"
        (new $stream
            (seq
                (canon "{peer_id_1}" $stream #canon_stream)
                (fold #canon_stream iterator
                    (ap iterator $stream))))
    "#);

    let result = call_vm!(peer_vm_1, <_>::default(), &script, "", "");
    assert_eq!(result.ret_code, INTERPRETER_SUCCESS)
}
