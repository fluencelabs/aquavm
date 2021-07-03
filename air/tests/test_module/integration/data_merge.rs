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

use pretty_assertions::assert_eq;
use serde_json::json;

use air_test_utils::call_vm;
use air_test_utils::create_avm;
use air_test_utils::executed_state;
use air_test_utils::set_variable_call_service;
use air_test_utils::set_variables_call_service;
use air_test_utils::trace_from_result;
use air_test_utils::CallServiceClosure;
use air_test_utils::IValue;
use air_test_utils::NEVec;

type JValue = serde_json::Value;

#[test]
fn data_merge() {
    use executed_state::*;

    let neighborhood_call_service1: CallServiceClosure = Box::new(|_| -> Option<IValue> {
        Some(IValue::Record(
            NEVec::new(vec![IValue::S32(0), IValue::String(String::from("[\"A\", \"B\"]"))]).unwrap(),
        ))
    });

    let neighborhood_call_service2: CallServiceClosure = Box::new(|_| -> Option<IValue> {
        Some(IValue::Record(
            NEVec::new(vec![IValue::S32(0), IValue::String(String::from("[\"A\", \"B\"]"))]).unwrap(),
        ))
    });

    let mut vm1 = create_avm(neighborhood_call_service1, "A");
    let mut vm2 = create_avm(neighborhood_call_service2, "B");

    let script = r#"
        (seq
            (call %init_peer_id% ("neighborhood" "") [] neighborhood)
            (seq
                (seq
                    (fold neighborhood i
                        (par
                            (call i ("add_provider" "") [] $void)
                            (next i)
                        )
                    )
                    (fold neighborhood i
                        (par
                            (call i ("get_providers" "") [] $providers)
                            (next i)
                        )
                    )
                )
                (seq
                    (call "A" ("identity" "") [] $void)
                    (call "B" ("" "") [] none)
                )
            )
        )
        "#;

    // little hack here with init_peer_id to execute the first call from both VMs
    let result_1 = call_vm!(vm1, "A", script, "", "");
    let result_2 = call_vm!(vm2, "B", script, "", "");
    let result_3 = call_vm!(vm1, "asd", script, result_1.data.clone(), result_2.data.clone());
    let result_4 = call_vm!(vm2, "asd", script, result_1.data.clone(), result_2.data.clone());

    let actual_trace_1 = trace_from_result(&result_1);

    let expected_trace_1 = vec![
        scalar_string_array(vec!["A", "B"]),
        par(1, 2),
        stream_string_array(vec!["A", "B"], "$void"),
        par(1, 0),
        request_sent_by("A"),
        par(1, 2),
        stream_string_array(vec!["A", "B"], "$providers"),
        par(1, 0),
        request_sent_by("A"),
        stream_string_array(vec!["A", "B"], "$void"),
        request_sent_by("A"),
    ];

    assert_eq!(actual_trace_1, expected_trace_1);
    assert_eq!(result_1.next_peer_pks, vec![String::from("B")]);

    let actual_trace_2 = trace_from_result(&result_2);

    let expected_trace_2 = vec![
        scalar_string_array(vec!["A", "B"]),
        par(1, 2),
        request_sent_by("B"),
        par(1, 0),
        stream_string_array(vec!["A", "B"], "$void"),
        par(1, 2),
        request_sent_by("B"),
        par(1, 0),
        stream_string_array(vec!["A", "B"], "$providers"),
        request_sent_by("B"),
    ];

    assert_eq!(actual_trace_2, expected_trace_2);
    assert_eq!(result_2.next_peer_pks, vec![String::from("A")]);

    let actual_trace_3 = trace_from_result(&result_3);

    let expected_trace_3 = vec![
        scalar_string_array(vec!["A", "B"]),
        par(1, 2),
        stream_string_array(vec!["A", "B"], "$void"),
        par(1, 0),
        stream_string_array(vec!["A", "B"], "$void"),
        par(1, 2),
        stream_string_array(vec!["A", "B"], "$providers"),
        par(1, 0),
        stream_string_array(vec!["A", "B"], "$providers"),
        stream_string_array(vec!["A", "B"], "$void"),
        request_sent_by("A"),
    ];

    assert_eq!(actual_trace_3, expected_trace_3);
    assert!(result_3.next_peer_pks.is_empty());

    let actual_trace_4 = trace_from_result(&result_4);

    let expected_trace_4 = vec![
        scalar_string_array(vec!["A", "B"]),
        par(1, 2),
        stream_string_array(vec!["A", "B"], "$void"),
        par(1, 0),
        stream_string_array(vec!["A", "B"], "$void"),
        par(1, 2),
        stream_string_array(vec!["A", "B"], "$providers"),
        par(1, 0),
        stream_string_array(vec!["A", "B"], "$providers"),
        stream_string_array(vec!["A", "B"], "$void"),
        scalar_string_array(vec!["A", "B"]),
    ];

    assert_eq!(actual_trace_4, expected_trace_4);
    assert!(result_4.next_peer_pks.is_empty());
}

#[test]
fn acc_merge() {
    env_logger::init();

    let neighborhood_call_service: CallServiceClosure = Box::new(|args| -> Option<IValue> {
        let args_count = match &args.function_args[1] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };

        let args_count = (args_count.as_bytes()[0] - b'0') as usize;

        let args_json = match &args.function_args[2] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };

        let args: Vec<JValue> = serde_json::from_str(args_json).expect("valid json");
        let args = match &args[0] {
            JValue::Array(args) => args,
            _ => unreachable!(),
        };

        assert_eq!(args.len(), args_count);

        Some(IValue::Record(
            NEVec::new(vec![IValue::S32(0), IValue::String(json!(args).to_string())]).unwrap(),
        ))
    });

    let mut vm1 = create_avm(set_variable_call_service(r#""peer_id""#), "A");
    let mut vm2 = create_avm(neighborhood_call_service, "B");

    let script = String::from(
        r#"
        (seq 
            (call "A" ("add_provider" "") [] $void)
            (seq 
                (call "A" ("add_provider" "") [] $void)
                (seq 
                    (call "A" ("get_providers" "") [] $providers)
                    (seq 
                        (call "A" ("get_providers" "") [] $providers)
                        (seq 
                            (call "B" ("" "2") [$providers] $void)
                            (call "B" ("" "3") [$void] $void)
                        )
                    )
                )
            )
        )
        "#,
    );

    let result = call_vm!(vm1, "asd", script.clone(), "", "");
    call_vm!(vm2, "asd", script, "", result.data);
}

#[test]
#[ignore]
fn fold_merge() {
    let set_variable_vm_id = "set_variable";
    let local_vm_id = "local_vm";

    let variables = maplit::hashmap! {
        "stream1".to_string() => r#"["s1", "s2", "s3"]"#.to_string(),
        "stream2".to_string() => r#"["s4", "s5", "s6"]"#.to_string(),
    };

    let local_vm_service_call: CallServiceClosure = Box::new(|args| -> Option<IValue> {
        let args = match &args.function_args[2] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };
        println!("args {:?}", args);

        Some(IValue::Record(
            NEVec::new(vec![IValue::S32(0), IValue::String(format!("{}", args))]).unwrap(),
        ))
    });

    let mut set_variable_vm = create_avm(set_variables_call_service(variables), set_variable_vm_id);
    let mut local_vm = create_avm(local_vm_service_call, local_vm_id);

    let script = format!(
        include_str!("./scripts/inner_folds_v1.clj"),
        set_variable_vm_id, local_vm_id
    );

    let result = call_vm!(set_variable_vm, "", &script, "", "");
    let result = call_vm!(local_vm, "", script, "", result.data);

    let _actual_trace = trace_from_result(&result);

    // println!("res is {:?}", actual_trace);
}
