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

use air::PreparationError;
use air::ToErrorCode;
use air_test_utils::prelude::*;

#[test]
fn lfold() {
    let mut vm = create_avm(echo_call_service(), "A");
    let mut set_variable_vm = create_avm(
        set_variable_call_service(json!(["1", "2", "3", "4", "5"])),
        "set_variable",
    );

    let lfold = r#"
            (seq
                (call "set_variable" ("" "") [] Iterable)
                (fold Iterable i
                    (seq
                        (call "A" ("" "") [i] $acc)
                        (next i)
                    )
                )
            )"#;

    let result = checked_call_vm!(set_variable_vm, "", lfold, "", "");
    let result = checked_call_vm!(vm, "", lfold, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_state = executed_state::scalar_string_array(vec!["1", "2", "3", "4", "5"]);

    assert_eq!(actual_trace.len(), 6);
    assert_eq!(actual_trace[0], expected_state);

    for i in 1..=5 {
        let expected_state = executed_state::stream_string(format!("{}", i), 0);
        assert_eq!(actual_trace[i], expected_state);
    }
}

#[test]
fn rfold() {
    let mut vm = create_avm(echo_call_service(), "A");
    let mut set_variable_vm = create_avm(
        set_variable_call_service(json!(["1", "2", "3", "4", "5"])),
        "set_variable",
    );

    let rfold = r#"
            (seq
                (call "set_variable" ("" "") [] Iterable)
                (fold Iterable i
                    (seq
                        (next i)
                        (call "A" ("" "") [i] $acc)
                    )
                )
            )"#;

    let result = checked_call_vm!(set_variable_vm, "", rfold, "", "");
    let result = checked_call_vm!(vm, "", rfold, "", result.data);

    let actual_trace = trace_from_result(&result);
    assert_eq!(actual_trace.len(), 6);

    let expected_state = executed_state::scalar_string_array(vec!["1", "2", "3", "4", "5"]);
    assert_eq!(actual_trace[0], expected_state);

    for i in 1..=5 {
        let expected_state = executed_state::stream_string(format!("{}", 6 - i), 0);
        assert_eq!(actual_trace[i], expected_state);
    }
}

#[test]
fn inner_fold() {
    let mut vm = create_avm(echo_call_service(), "A");
    let mut set_variable_vm = create_avm(
        set_variable_call_service(json!(["1", "2", "3", "4", "5"])),
        "set_variable",
    );

    let script = r#"
            (seq
                (seq
                    (call "set_variable" ("" "") [] Iterable1)
                    (call "set_variable" ("" "") [] Iterable2)
                )
                (fold Iterable1 i
                    (seq
                        (fold Iterable2 j
                            (seq
                                (call "A" ("" "") [i] $acc)
                                (next j)
                            )
                        )
                        (next i)
                    )
                )
            )"#;

    let result = checked_call_vm!(set_variable_vm, "", script, "", "");
    let result = checked_call_vm!(vm, "", script, "", result.data);

    let actual_trace = trace_from_result(&result);
    assert_eq!(actual_trace.len(), 27);

    let expected_state = executed_state::scalar_string_array(vec!["1", "2", "3", "4", "5"]);
    assert_eq!(actual_trace[0], expected_state);
    assert_eq!(actual_trace[1], expected_state);

    for i in 1..=5 {
        for j in 1..=5 {
            let expected_state = executed_state::stream_string(i.to_string(), 0);
            assert_eq!(actual_trace[1 + 5 * (i - 1) + j], expected_state);
        }
    }
}

#[test]
fn inner_fold_with_same_iterator() {
    let mut vm = create_avm(
        set_variable_call_service(json!(["1", "2", "3", "4", "5"])),
        "set_variable",
    );

    let script = r#"
            (seq
                (seq
                    (call "set_variable" ("" "") [] Iterable1)
                    (call "set_variable" ("" "") [] Iterable2)
                )
                (fold Iterable1 i
                    (seq
                        (fold Iterable2 i
                            (seq
                                (call "A" ("" "") [i] $acc)
                                (next i)
                            )
                        )
                        (next i)
                    )
                )
            )"#;

    let result = call_vm!(vm, "", script, "", "");

    let expected_error = PreparationError::AIRParseError("".to_string());
    assert_eq!(result.ret_code, expected_error.to_error_code());
}

#[test]
fn empty_iterable_fold() {
    let mut vm = create_avm(echo_call_service(), "A");
    let mut set_variable_vm = create_avm(set_variable_call_service(json!([])), "set_variable");

    let empty_fold = r#"
            (seq
                (call "set_variable" ("" "") [] Iterable)
                (fold Iterable i
                    (seq
                        (call "A" ("" "") [i] $acc)
                        (next i)
                    )
                )
            )"#;

    let result = checked_call_vm!(set_variable_vm, "", empty_fold, "", "");
    let result = checked_call_vm!(vm, "", empty_fold, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_state = executed_state::scalar(json!([]));

    assert_eq!(actual_trace.len(), 1);
    assert_eq!(actual_trace[0], expected_state);
}

#[test]
fn empty_literal_array_fold() {
    let mut vm = create_avm(echo_call_service(), "A");

    let empty_fold = r#"
        (fold [] i
            (seq
                (call "A" ("" "") [i] $acc)
                (next i)
            )
        )"#;

    let result = checked_call_vm!(vm, "", empty_fold, "", "");
    let actual_trace = trace_from_result(&result);

    assert!(actual_trace.is_empty());
}

#[test]
fn empty_fold_json_path() {
    let mut vm = create_avm(echo_call_service(), "A");
    let mut set_variable_vm = create_avm(set_variable_call_service(json!({ "messages": [] })), "set_variable");

    let empty_fold = r#"
            (seq
                (call "set_variable" ("" "") [] messages)
                (fold messages.$.messages! i
                    (seq
                        (call "A" ("" "") [i] $acc)
                        (next i)
                    )
                )
            )"#;

    let result = checked_call_vm!(set_variable_vm, "", empty_fold, "", "");
    let result = checked_call_vm!(vm, "", empty_fold, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![executed_state::scalar(json!({ "messages": [] }))];

    assert_eq!(actual_trace, expected_trace);
}

// Check that fold works with the join behaviour without hanging up.
#[test]
fn fold_with_join() {
    let mut vm = create_avm(echo_call_service(), "A");
    let mut set_variable_vm = create_avm(set_variable_call_service(json!(["1", "2"])), "set_variable");

    let fold_with_join = r#"
            (seq
                (call "set_variable" ("" "") [] iterable)
                (par
                    (call "unknown_peer" ("" "") [] lazy_def_variable)
                    (fold iterable i
                        (seq
                            (call "A" ("" "") [lazy_def_variable.$.hash!] $acc)
                            (next i)
                        )
                    )
                )
            )"#;

    let result = checked_call_vm!(set_variable_vm, "", fold_with_join, "", "");
    let result = checked_call_vm!(vm, "", fold_with_join, "", result.data);

    let actual_trace = trace_from_result(&result);
    assert_eq!(actual_trace.len(), 4);
}

#[test]
fn lambda() {
    let mut vm = create_avm(echo_call_service(), "A");
    let mut set_variable_vm = create_avm(
        set_variable_call_service(json!({ "array": ["1","2","3","4","5"] })),
        "set_variable",
    );

    let script = r#"
            (seq
                (call "set_variable" ("" "") [] iterable)
                (fold iterable.$.array! i
                    (seq
                        (call "A" ("" "") [i] $acc)
                        (next i)
                    )
                )
            )"#;

    let result = checked_call_vm!(set_variable_vm, "", script, "", "");
    let result = checked_call_vm!(vm, "", script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_state = executed_state::scalar(json!({ "array": ["1", "2", "3", "4", "5"] }));

    assert_eq!(actual_trace.len(), 6);
    assert_eq!(actual_trace[0], expected_state);

    for i in 1..=5 {
        let expected_state = executed_state::stream_string(format!("{}", i), 0);
        assert_eq!(actual_trace[i], expected_state);
    }
}

#[test]
fn shadowing() {
    use executed_state::*;

    let mut set_variables_vm = create_avm(set_variable_call_service(json!(["1", "2"])), "set_variable");
    let mut vm_a = create_avm(echo_call_service(), "A");
    let mut vm_b = create_avm(echo_call_service(), "B");

    let script = r#"
            (seq
                (seq
                    (call "set_variable" ("" "") [] iterable1)
                    (call "set_variable" ("" "") [] iterable2)
                )
                (fold iterable1 i
                    (seq
                        (seq
                            (fold iterable2 j
                                (seq
                                    (seq
                                        (call "A" ("" "") [i] local_j)
                                        (call "B" ("" "") [local_j])
                                    )
                                    (next j)
                                )
                            )
                            (par
                                (call "A" ("" "") [i] local_i)
                                (call "B" ("" "") [i])
                            )
                        )
                        (next i)
                    )
                )
            )"#;

    let result = checked_call_vm!(set_variables_vm, "", script, "", "");
    let result = checked_call_vm!(vm_a, "", script, "", result.data);
    let result = checked_call_vm!(vm_b, "", script, "", result.data);
    let result = checked_call_vm!(vm_a, "", script, "", result.data);
    let result = checked_call_vm!(vm_b, "", script, "", result.data);
    let result = checked_call_vm!(vm_a, "", script, "", result.data);
    let result = checked_call_vm!(vm_b, "", script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![
        scalar_string_array(vec!["1", "2"]),
        scalar_string_array(vec!["1", "2"]),
        scalar_string("1"),
        scalar_string("1"),
        scalar_string("1"),
        scalar_string("1"),
        par(1, 1),
        scalar_string("1"),
        scalar_string("1"),
        scalar_string("2"),
        scalar_string("2"),
        request_sent_by("B"),
    ];

    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn shadowing_scope() {
    use executed_state::*;

    fn execute_script(script: String) -> Result<RawAVMOutcome, String> {
        let mut set_variables_vm = create_avm(set_variable_call_service(json!(["1", "2"])), "set_variable");
        let mut vm_a = create_avm(echo_call_service(), "A");
        let mut vm_b = create_avm(echo_call_service(), "B");

        let result = checked_call_vm!(set_variables_vm, "", script.clone(), "", "");
        let result = checked_call_vm!(vm_a, "", script.clone(), "", result.data);
        let result = checked_call_vm!(vm_b, "", script.clone(), "", result.data);
        let result = checked_call_vm!(vm_a, "", script.clone(), "", result.data);
        let result = checked_call_vm!(vm_b, "", script.clone(), "", result.data);

        vm_a.call(script, "", result.data, "")
    }

    let variable_shadowing_script = r#"
            (seq
                (seq
                    (call "set_variable" ("" "") [] iterable1)
                    (call "set_variable" ("" "") [] iterable2)
                )
                (fold iterable1 i
                    (seq
                        (seq
                            (call "A" ("" "") ["value"] local_j)
                            (seq
                                (fold iterable2 j
                                    (seq
                                        (seq
                                            (call "A" ("" "") [i] local_j)
                                            (call "B" ("" "") [local_j])
                                        )
                                        (next j)
                                    )
                                )
                                (call "A" ("" "") [local_j])
                            )
                        )
                        (next i)
                    )
                )
            )"#;

    let result = execute_script(String::from(variable_shadowing_script)).unwrap();

    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![
        scalar_string_array(vec!["1", "2"]),
        scalar_string_array(vec!["1", "2"]),
        scalar_string("value"),
        scalar_string("1"),
        scalar_string("1"),
        scalar_string("1"),
        scalar_string("1"),
        scalar_string("value"),
        scalar_string("value"),
        scalar_string("2"),
        request_sent_by("A"),
    ];

    assert_eq!(actual_trace, expected_trace);
}
