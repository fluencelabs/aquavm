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
fn recursive_stream_with_early_exit() {
    let vm_peer_id = "vm_peer_id";
    let variable_mappings = maplit::hashmap! {
        "stream_value".to_string() => json!(1),
        "stop".to_string() => json!("stop"),
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
    let actual_trace = trace_from_result(&result);
    let expected_state = vec![
        executed_state::stream_number(1, 0),
        executed_state::stream_number(1, 0),
        executed_state::fold(vec![executed_state::subtrace_lore(
            0,
            SubTraceDesc::new(3, 1),
            SubTraceDesc::new(4, 0),
        )]),
        executed_state::scalar_string("stop"),
    ];

    assert_eq!(actual_trace, expected_state);
}

#[test]
fn recursive_stream_many_iterations() {
    let vm_peer_id_1 = "vm_peer_id_1";

    let request_id = std::cell::Cell::new(0);
    let stop_request_id = 10;
    let give_n_results_and_then_stop: CallServiceClosure = Box::new(move |_params| {
        let uncelled_request_id = request_id.get();

        let result = if uncelled_request_id >= stop_request_id {
            CallServiceResult::ok(json!("stop"))
        } else {
            CallServiceResult::ok(json!("non_stop"))
        };

        request_id.set(uncelled_request_id + 1);
        result
    });

    let mut vm_1 = create_avm(give_n_results_and_then_stop, vm_peer_id_1);

    let vm_peer_id_2 = "vm_peer_id_2";
    let mut vm_2 = create_avm(echo_call_service(), vm_peer_id_2);

    let result_value = "result_value";
    let script = f!(r#"
    (seq
        (seq
            (seq
                (call "{vm_peer_id_1}" ("" "stream_value") [] $stream)
                (call "{vm_peer_id_1}" ("" "stream_value") [] $stream)
            )
            (fold $stream iterator
                (seq
                    (call "{vm_peer_id_1}" ("" "stop") [] value)
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
        )
        (call "{vm_peer_id_2}" ("" "") ["{result_value}"])
    )"#);

    let result = checked_call_vm!(vm_1, "", &script, "", "");
    let actual_trace = trace_from_result(&result);
    let actual_fold = &actual_trace[2];
    let expected_fold = executed_state::fold(vec![
        executed_state::subtrace_lore(0, SubTraceDesc::new(3, 2), SubTraceDesc::new(7, 0)),
        executed_state::subtrace_lore(1, SubTraceDesc::new(5, 2), SubTraceDesc::new(7, 0)),
        executed_state::subtrace_lore(4, SubTraceDesc::new(7, 2), SubTraceDesc::new(9, 0)),
        executed_state::subtrace_lore(6, SubTraceDesc::new(9, 2), SubTraceDesc::new(11, 0)),
        executed_state::subtrace_lore(8, SubTraceDesc::new(11, 2), SubTraceDesc::new(15, 0)),
        executed_state::subtrace_lore(10, SubTraceDesc::new(13, 2), SubTraceDesc::new(15, 0)),
        executed_state::subtrace_lore(12, SubTraceDesc::new(15, 2), SubTraceDesc::new(19, 0)),
        executed_state::subtrace_lore(14, SubTraceDesc::new(17, 2), SubTraceDesc::new(19, 0)),
        executed_state::subtrace_lore(16, SubTraceDesc::new(19, 1), SubTraceDesc::new(20, 0)),
    ]);
    assert_eq!(actual_fold, &expected_fold);

    let actual_last_state = &actual_trace[20];
    let expected_last_state = executed_state::request_sent_by(vm_peer_id_1);
    assert_eq!(actual_last_state, &expected_last_state);

    let result = checked_call_vm!(vm_2, "", script, "", result.data);
    let actual_trace = trace_from_result(&result);
    let actual_last_state = &actual_trace[20];
    let expected_last_state = executed_state::scalar_string(result_value);
    assert_eq!(actual_last_state, &expected_last_state);
}
