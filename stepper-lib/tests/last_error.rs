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

use aqua_test_utils::call_vm;
use aqua_test_utils::create_aqua_vm;
use aqua_test_utils::echo_number_call_service;
use aqua_test_utils::unit_call_service;
use aqua_test_utils::CallServiceClosure;
use aqua_test_utils::IValue;
use aqua_test_utils::NEVec;

use serde_json::json;

use std::rc::Rc;

type JValue = serde_json::Value;

#[test]
fn executed_trace_seq_par_call() {
    use stepper_lib::execution_trace::CallResult::*;
    use stepper_lib::execution_trace::ExecutedState::{self, *};

    let local_peer_id = "local_peer_id";
    let mut vm = create_aqua_vm(unit_call_service(), local_peer_id);

    let script = format!(include_str!("scripts/create_service_with_xor.clj"), local_peer_id);

    let initial_state = json!([
        { "par": [1,1] },
        { "call": {"executed": "test"} },
        { "call": {"executed": "test"} },
    ])
    .to_string();

    let res = call_vm!(vm, "asd", script, "[]", initial_state);
    let actual_trace: Vec<ExecutedState> = serde_json::from_slice(&res.data).expect("stepper should return valid json");

    let test_string = String::from("test");
    let expected_trace = vec![
        Par(1, 1),
        Call(Executed(Rc::new(JValue::String(test_string.clone())))),
        Call(Executed(Rc::new(JValue::String(test_string.clone())))),
        Call(Executed(Rc::new(JValue::String(test_string)))),
    ];

    assert_eq!(actual_trace, expected_trace);
    assert!(res.next_peer_pks.is_empty());
}
