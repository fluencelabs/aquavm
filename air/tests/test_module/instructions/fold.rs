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
use air_test_utils::create_avm;
use air_test_utils::echo_number_call_service;
use air_test_utils::echo_string_call_service;
use air_test_utils::executed_state;
use air_test_utils::set_variable_call_service;
use air_test_utils::AVMError;
use air_test_utils::ExecutionTrace;
use air_test_utils::InterpreterOutcome;

use serde_json::json;

#[test]
fn lfold() {
    let mut vm = create_avm(echo_number_call_service(), "A");
    let mut set_variable_vm = create_avm(set_variable_call_service(r#"["1","2","3","4","5"]"#), "set_variable");

    let lfold = String::from(
        r#"
            (seq
                (call "set_variable" ("" "") [] Iterable)
                (fold Iterable i
                    (seq
                        (call "A" ("" "") [i] $acc)
                        (next i)
                    )
                )
            )"#,
    );

    let res = call_vm!(set_variable_vm, "", lfold.clone(), "[]", "[]");
    let res = call_vm!(vm, "", lfold, "[]", res.data);
    let actual_trace: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be valid executed trace");
    let expected_state = executed_state::scalar_string_array(vec!["1", "2", "3", "4", "5"]);

    assert_eq!(actual_trace.len(), 6);
    assert_eq!(actual_trace[0], expected_state);

    for i in 1..=5 {
        let expected_state = executed_state::stream_number(i, "$acc");
        assert_eq!(actual_trace[i], expected_state);
    }
}

#[test]
fn rfold() {
    let mut vm = create_avm(echo_number_call_service(), "A");
    let mut set_variable_vm = create_avm(set_variable_call_service(r#"["1","2","3","4","5"]"#), "set_variable");

    let rfold = String::from(
        r#"
            (seq
                (call "set_variable" ("" "") [] Iterable)
                (fold Iterable i
                    (seq
                        (next i)
                        (call "A" ("" "") [i] $acc)
                    )
                )
            )"#,
    );

    let res = call_vm!(set_variable_vm, "", rfold.clone(), "[]", "[]");
    let res = call_vm!(vm, "", rfold, "[]", res.data);

    let actual_trace: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be valid executed trace");
    assert_eq!(actual_trace.len(), 6);

    let expected_state = executed_state::scalar_string_array(vec!["1", "2", "3", "4", "5"]);
    assert_eq!(actual_trace[0], expected_state);

    for i in 1..=5 {
        let expected_state = executed_state::stream_number(6 - i, "$acc");
        assert_eq!(actual_trace[i], expected_state);
    }
}

#[test]
fn inner_fold() {
    let mut vm = create_avm(echo_number_call_service(), "A");
    let mut set_variable_vm = create_avm(set_variable_call_service(r#"["1","2","3","4","5"]"#), "set_variable");

    let script = String::from(
        r#"
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
            )"#,
    );

    let res = call_vm!(set_variable_vm, "", script.clone(), "[]", "[]");
    let res = call_vm!(vm, "", script, "[]", res.data);
    let actual_trace: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be valid executed trace");

    assert_eq!(actual_trace.len(), 27);

    let expected_state = executed_state::scalar_string_array(vec!["1", "2", "3", "4", "5"]);
    assert_eq!(actual_trace[0], expected_state);
    assert_eq!(actual_trace[1], expected_state);

    for i in 1..=5 {
        for j in 1..=5 {
            let expected_state = executed_state::stream_number(i, "$acc");
            assert_eq!(actual_trace[1 + 5 * (i - 1) + j], expected_state);
        }
    }
}

#[test]
fn several_nexts() {
    let mut vm = create_avm(echo_number_call_service(), "A");
    let mut set_variable_vm = create_avm(set_variable_call_service(r#"["1","2"]"#), "set_variable");

    let script = String::from(
        r#"
            (seq
                (call "set_variable" ("" "") [] Iterable1)
                (fold Iterable1 i
                    (seq
                        (seq
                            (seq
                                (call "A" ("" "") [i "A"] $acc)
                                (next i)
                            )
                            (seq
                                (call "A" ("" "") [i "B"] $acc)
                                (next i)
                            )
                        )
                        (seq
                            (call "A" ("" "") [i "C"] $acc)
                            (next i)
                        )
                    )
                )
            )"#,
    );

    let res = call_vm!(set_variable_vm, "", &script, "", "");
    let res = call_vm!(vm, "", script, "", res.data);
    let actual_trace: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be valid executed trace");

    for state in actual_trace {
        match state {
            air_test_utils::ExecutedState::Call(air_test_utils::CallResult::Executed(value)) => {
                println!("{}", value)
            }
            _ => {}
        }
    }
}

#[test]
fn inner_fold_with_same_iterator() {
    let mut vm = create_avm(set_variable_call_service(r#"["1","2","3","4","5"]"#), "set_variable");

    let script = String::from(
        r#"
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
            )"#,
    );

    let res = call_vm!(vm, "", script, "[]", "[]");

    assert_eq!(res.ret_code, 1012);
}

#[test]
fn empty_fold() {
    let mut vm = create_avm(echo_number_call_service(), "A");
    let mut set_variable_vm = create_avm(set_variable_call_service(r#"[]"#), "set_variable");

    let empty_fold = String::from(
        r#"
            (seq
                (call "set_variable" ("" "") [] Iterable)
                (fold Iterable i
                    (seq
                        (call "A" ("" "") [i] $acc)
                        (next i)
                    )
                )
            )"#,
    );

    let res = call_vm!(set_variable_vm, "", empty_fold.clone(), "[]", "[]");
    let res = call_vm!(vm, "", empty_fold, "[]", res.data);
    let actual_trace: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be valid executed trace");
    let expected_state = executed_state::scalar_jvalue(json!([]));

    assert_eq!(actual_trace.len(), 1);
    assert_eq!(actual_trace[0], expected_state);
}

#[test]
fn empty_fold_json_path() {
    let mut vm = create_avm(echo_number_call_service(), "A");
    let mut set_variable_vm = create_avm(set_variable_call_service(r#"{ "messages": [] }"#), "set_variable");

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

    let res = call_vm!(set_variable_vm, "", empty_fold, "", "");
    let res = call_vm!(vm, "", empty_fold, "", res.data);
    let res: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be valid executed trace");

    assert_eq!(res.len(), 1);
    assert_eq!(res[0], executed_state::scalar_jvalue(json!({ "messages": [] })));
}

// Check that fold works with the join behaviour without hanging up.
#[test]
fn fold_with_join() {
    let mut vm = create_avm(echo_number_call_service(), "A");
    let mut set_variable_vm = create_avm(set_variable_call_service(r#"["1","2"]"#), "set_variable");

    let fold_with_join = String::from(
        r#"
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
            )"#,
    );

    let res = call_vm!(set_variable_vm, "", &fold_with_join, "", "");
    let res = call_vm!(vm, "", fold_with_join, "", res.data);
    let res: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be valid executed trace");

    assert_eq!(res.len(), 3);
}

#[test]
fn json_path() {
    let mut vm = create_avm(echo_number_call_service(), "A");
    let mut set_variable_vm = create_avm(
        set_variable_call_service(r#"{ "array": ["1","2","3","4","5"] }"#),
        "set_variable",
    );

    let lfold = String::from(
        r#"
            (seq
                (call "set_variable" ("" "") [] iterable)
                (fold iterable.$.array! i
                    (seq
                        (call "A" ("" "") [i] $acc)
                        (next i)
                    )
                )
            )"#,
    );

    let res = call_vm!(set_variable_vm, "", lfold.clone(), "[]", "[]");
    let res = call_vm!(vm, "", lfold, "[]", res.data);
    let actual_trace: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be valid executed trace");
    let expected_state = executed_state::scalar_jvalue(json!({ "array": ["1", "2", "3", "4", "5"] }));

    assert_eq!(actual_trace.len(), 6);
    assert_eq!(actual_trace[0], expected_state);

    for i in 1..=5 {
        let expected_state = executed_state::stream_number(i, "$acc");
        assert_eq!(actual_trace[i], expected_state);
    }
}

#[test]
fn shadowing() {
    use executed_state::*;

    let mut set_variables_vm = create_avm(set_variable_call_service(r#"["1","2"]"#), "set_variable");
    let mut vm_a = create_avm(echo_string_call_service(), "A");
    let mut vm_b = create_avm(echo_string_call_service(), "B");

    let script = String::from(
        r#"
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
            )"#,
    );

    let res = call_vm!(set_variables_vm, "", script.clone(), "[]", "[]");
    let res = call_vm!(vm_a, "", script.clone(), "[]", res.data);
    let res = call_vm!(vm_b, "", script.clone(), "[]", res.data);
    let res = call_vm!(vm_a, "", script.clone(), "[]", res.data);
    let res = call_vm!(vm_b, "", script.clone(), "[]", res.data);
    let res = call_vm!(vm_a, "", script.clone(), "[]", res.data);
    let res = call_vm!(vm_b, "", script, "[]", res.data);

    let actual_trace: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be valid executed trace");
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

    fn execute_script(script: String) -> Result<InterpreterOutcome, AVMError> {
        let mut set_variables_vm = create_avm(set_variable_call_service(r#"["1","2"]"#), "set_variable");
        let mut vm_a = create_avm(echo_string_call_service(), "A");
        let mut vm_b = create_avm(echo_string_call_service(), "B");

        let res = call_vm!(set_variables_vm, "", script.clone(), "[]", "[]");
        let res = call_vm!(vm_a, "", script.clone(), "[]", res.data);
        let res = call_vm!(vm_b, "", script.clone(), "[]", res.data);
        let res = call_vm!(vm_a, "", script.clone(), "[]", res.data);
        let res = call_vm!(vm_b, "", script.clone(), "[]", res.data);

        vm_a.call_with_prev_data("", script, "[]", res.data)
    }

    let variable_shadowing_script = String::from(
        r#"
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
            )"#,
    );

    let res = execute_script(variable_shadowing_script).unwrap();
    let actual_trace: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be valid executed trace");
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
