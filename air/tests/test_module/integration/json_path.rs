/*
 * Copyright 2020 Fluence Labs Limited
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

use air_test_utils::call_vm;
use air_test_utils::checked_call_vm;
use air_test_utils::create_avm;
use air_test_utils::echo_string_call_service;

#[test]
fn json_path_not_allowed_for_non_objects_and_arrays() {
    let set_variable_peer_id = "set_variable";
    let mut set_variable_vm = create_avm(echo_string_call_service(), set_variable_peer_id);

    let local_peer_id = "local_peer_id";
    let mut local_vm = create_avm(echo_string_call_service(), local_peer_id);

    let script = format!(
        r#"
        (seq
            (call "{0}" ("" "") ["some_string"] string_variable)
            (call "{1}" ("" "") [string_variable.$.some_json_path])
        )
        "#,
        set_variable_peer_id, local_peer_id
    );

    let result = checked_call_vm!(set_variable_vm, "asd", &script, "", "");
    let result = call_vm!(local_vm, "asd", script, "", result.data);

    assert_eq!(result.ret_code, 1017);
}
