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

#[test]
fn empty_array() {
    let vm_peer_id = "some_peer_id";
    let mut vm = create_avm(echo_call_service(), vm_peer_id);

    let script = f!(r#"
        (seq 
           (call "{vm_peer_id}" ("" "") [[]] result)
           (call "{vm_peer_id}" ("" "") [result])
        )"#);

    let result = checked_call_vm!(vm, <_>::default(), script, "", "");
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![scalar!(json!([])), scalar!(json!([]))];

    assert_eq!(actual_trace, expected_trace);
    assert!(result.next_peer_pks.is_empty());
}
