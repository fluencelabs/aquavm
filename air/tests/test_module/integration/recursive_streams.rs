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

use fstrings::f;
use fstrings::format_args_f;

#[test]
fn recursive_stream_basic() {
    let vm_peer_id = "vm_peer_id";
    let variable_mappings = maplit::hashmap! {
        "stream_value".to_string() => json!(1),
        "stop".to_string() => json!("stop1"),
    };
    let mut vm = create_avm(
        set_variables_call_service(variable_mappings, VariableOptionSource::FunctionName),
        vm_peer_id,
    );

    let script = f!(r#"
        (seq
            (seq
                (call "{vm_peer_id}" ("" "stream_value") [] $stream)
                (call "{vm_peer_id}" ("" "stream_value") [] $stream)
            )
            (fold $stream iterator
                (seq
                    (call "{vm_peer_id}" ("" "stop") [] value)
                    (xor
                        (match value "stop"
                            (null)
                        )
                        (seq
                            (ap value $stream)
                            (next iterator)
                        )
                    )
                )
            )
        )"#);

    let result = checked_call_vm!(vm, "", script, "", "");
    print_trace(&result, "first");
}
