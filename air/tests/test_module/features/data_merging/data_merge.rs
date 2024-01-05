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

use air::ExecutionCidState;
use air_interpreter_data::ExecutionTrace;
use air_test_utils::prelude::*;

use pretty_assertions::assert_eq;

use std::collections::HashMap;
use std::ops::Deref;

#[test]
fn merge_streams_in_two_fold() {
    use executed_state::*;

    let set_variable_peer_id = "set_variable_peer_id";
    let vm_1_peer_id = "vm_1_peer_id";
    let vm_2_peer_id = "vm_2_peer_id";

    let mut set_variable = create_avm(
        set_variable_call_service(json!([vm_1_peer_id, vm_2_peer_id])),
        set_variable_peer_id,
    );
    let mut vm1 = create_avm(return_string_call_service(vm_1_peer_id), vm_1_peer_id);
    let mut vm2 = create_avm(return_string_call_service(vm_2_peer_id), vm_2_peer_id);

    let script = format!(
        r#"
        (seq
            (call "{set_variable_peer_id}" ("neighborhood" "") [] neighborhood)
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
                    (call "{vm_1_peer_id}" ("identity" "") [] $void)
                    (call "{vm_2_peer_id}" ("" "") [] none)
                )
            )
        )
        "#
    );

    let result_0 = checked_call_vm!(set_variable, <_>::default(), &script, "", "");
    let result_1 = checked_call_vm!(vm1, <_>::default(), &script, "", result_0.data.clone());
    let result_2 = checked_call_vm!(vm2, <_>::default(), &script, "", result_0.data);
    let result_3 = checked_call_vm!(
        vm1,
        <_>::default(),
        &script,
        result_1.data.clone(),
        result_2.data.clone()
    );
    let result_4 = checked_call_vm!(
        vm2,
        <_>::default(),
        script,
        result_1.data.clone(),
        result_2.data.clone()
    );

    let actual_trace_1 = trace_from_result(&result_1);

    let expected_trace_1 = ExecutionTrace::from(vec![
        scalar!(
            json!([vm_1_peer_id, vm_2_peer_id]),
            peer = set_variable_peer_id,
            service = "neighborhood"
        ),
        par(1, 2),
        stream!(vm_1_peer_id, 0, peer = vm_1_peer_id, service = "add_provider"),
        par(1, 0),
        request_sent_by(set_variable_peer_id),
        par(1, 2),
        stream!(vm_1_peer_id, 0, peer = vm_1_peer_id, service = "get_providers"),
        par(1, 0),
        request_sent_by(vm_1_peer_id),
        stream!(vm_1_peer_id, 1, peer = vm_1_peer_id, service = "identity"),
        request_sent_by(vm_1_peer_id),
    ]);

    assert_eq!(actual_trace_1, expected_trace_1);
    assert_eq!(result_1.next_peer_pks, vec![vm_2_peer_id.to_string()]);

    let actual_trace_2 = trace_from_result(&result_2);

    let expected_trace_2 = ExecutionTrace::from(vec![
        scalar!(
            json!([vm_1_peer_id, vm_2_peer_id]),
            peer = set_variable_peer_id,
            service = "neighborhood"
        ),
        par(1, 2),
        request_sent_by(set_variable_peer_id),
        par(1, 0),
        stream!(vm_2_peer_id, 0, peer = vm_2_peer_id, service = "add_provider"),
        par(1, 2),
        request_sent_by(vm_2_peer_id),
        par(1, 0),
        stream!(vm_2_peer_id, 0, peer = vm_2_peer_id, service = "get_providers"),
        request_sent_by(vm_2_peer_id),
    ]);

    assert_eq!(actual_trace_2, expected_trace_2);
    assert_eq!(result_2.next_peer_pks, vec![vm_1_peer_id.to_string()]);

    let actual_trace_3 = trace_from_result(&result_3);

    let expected_trace_3 = vec![
        scalar!(
            json!([vm_1_peer_id, vm_2_peer_id]),
            peer = set_variable_peer_id,
            service = "neighborhood"
        ),
        par(1, 2),
        stream!(vm_1_peer_id, 0, peer = vm_1_peer_id, service = "add_provider"),
        par(1, 0),
        stream!(vm_2_peer_id, 2, peer = vm_2_peer_id, service = "add_provider"),
        par(1, 2),
        stream!(vm_1_peer_id, 0, peer = vm_1_peer_id, service = "get_providers"),
        par(1, 0),
        stream!(vm_2_peer_id, 1, peer = vm_2_peer_id, service = "get_providers"),
        stream!(vm_1_peer_id, 1, peer = vm_1_peer_id, service = "identity"),
        request_sent_by(vm_1_peer_id),
    ];

    assert_eq!(actual_trace_3.deref(), expected_trace_3);
    assert!(result_3.next_peer_pks.is_empty());

    let actual_trace_4 = trace_from_result(&result_4);

    let expected_trace_4 = ExecutionTrace::from(vec![
        scalar!(
            json!([vm_1_peer_id, vm_2_peer_id]),
            peer = set_variable_peer_id,
            service = "neighborhood"
        ),
        par(1, 2),
        stream!(vm_1_peer_id, 0, peer = vm_1_peer_id, service = "add_provider"),
        par(1, 0),
        stream!(vm_2_peer_id, 2, peer = vm_2_peer_id, service = "add_provider"),
        par(1, 2),
        stream!(vm_1_peer_id, 0, peer = vm_1_peer_id, service = "get_providers"),
        par(1, 0),
        stream!(vm_2_peer_id, 1, peer = vm_2_peer_id, service = "get_providers"),
        stream!(vm_1_peer_id, 1, peer = vm_1_peer_id, service = "identity"),
        scalar!(vm_2_peer_id, peer = vm_2_peer_id),
    ]);

    assert_eq!(actual_trace_4, expected_trace_4);
    assert!(result_4.next_peer_pks.is_empty());
}

#[test]
fn stream_merge() {
    let neighborhood_call_service: CallServiceClosure = Box::new(|params| -> CallServiceResult {
        let args_count = (params.function_name.as_bytes()[0] - b'0') as usize;
        let args: Vec<Vec<JValue>> = serde_json::from_value(JValue::Array(params.arguments)).expect("valid json");
        assert_eq!(args[0].len(), args_count);

        CallServiceResult::ok(json!(args))
    });

    let mut vm1 = create_avm(set_variable_call_service(json!("peer_id")), "A");
    let mut vm2 = create_avm(neighborhood_call_service, "B");

    let script = r#"
        (seq
            (call "A" ("add_provider" "") [] $void)
            (seq
                (call "A" ("add_provider" "") [] $void)
                (seq
                    (call "A" ("get_providers" "") [] $providers)
                    (seq
                        (call "A" ("get_providers" "") [] $providers)
                        (seq
                            (seq
                                (canon "B" $providers #providers)
                                (call "B" ("" "2") [#providers] $void)
                            )
                            (seq
                                (canon "B" $void #void)
                                (call "B" ("" "3") [#void] $void)
                            )
                        )
                    )
                )
            )
        )
        "#;

    let result = checked_call_vm!(vm1, <_>::default(), script, "", "");
    checked_call_vm!(vm2, <_>::default(), script, "", result.data);
}

#[test]
fn fold_merge() {
    let set_variable_vm_id = "set_variable";
    let local_vm_id = "local_vm";

    let variables = maplit::hashmap! {
        "stream_1".to_string() => json!(["peer_0", "peer_1", "peer_2", "peer_3"]),
        "stream_2".to_string() => json!(["peer_4", "peer_5", "peer_6"]),
    };
    let mut set_variable_vm = create_avm(
        set_variables_call_service(variables, VariableOptionSource::Argument(0)),
        set_variable_vm_id,
    );

    let script = format!(
        include_str!("./scripts/inner_folds_v1.air"),
        set_variable_vm_id, local_vm_id
    );

    let set_variable_result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");

    let mut local_vms = Vec::with_capacity(7);
    let mut local_vms_results = Vec::with_capacity(7);
    for vm_id in 0..7 {
        let peer_id = format!("peer_{vm_id}");
        let mut vm = create_avm(echo_call_service(), peer_id);
        let result = checked_call_vm!(
            vm,
            <_>::default(),
            &script,
            set_variable_result.data.clone(),
            set_variable_result.data.clone()
        );

        local_vms.push(vm);
        local_vms_results.push(result);
    }

    let mut local_vm = create_avm(echo_call_service(), local_vm_id);
    let result_1 = checked_call_vm!(local_vm, <_>::default(), &script, "", local_vms_results[0].data.clone());
    let result_2 = checked_call_vm!(
        local_vm,
        <_>::default(),
        &script,
        result_1.data,
        local_vms_results[3].data.clone()
    );
    let result_3 = checked_call_vm!(
        local_vm,
        <_>::default(),
        &script,
        result_2.data,
        local_vms_results[4].data.clone()
    );
    let result_4 = checked_call_vm!(
        local_vm,
        <_>::default(),
        &script,
        result_3.data,
        local_vms_results[5].data.clone()
    );

    let result_5 = checked_call_vm!(
        local_vm,
        <_>::default(),
        &script,
        result_4.data,
        local_vms_results[1].data.clone()
    );

    let result_6 = checked_call_vm!(
        local_vm,
        <_>::default(),
        &script,
        result_5.data,
        local_vms_results[2].data.clone()
    );

    let result_7 = checked_call_vm!(
        local_vm,
        <_>::default(),
        &script,
        result_6.data,
        local_vms_results[6].data.clone()
    );

    let (_version, data) = InterpreterDataEnvelope::try_from_slice(&result_7.data).expect("data should be well-formed");

    let mut fold_states_count = 0;
    let mut calls_count = HashMap::new();

    for state in data.trace.iter() {
        if matches!(state, ExecutedState::Fold(_)) {
            fold_states_count += 1;
        }

        if let ExecutedState::Fold(fold) = state {
            for subtrace_lore in fold.lore.iter() {
                let value_pos = subtrace_lore.value_pos;
                if let ExecutedState::Call(CallResult::Executed(value)) = &data.trace[value_pos] {
                    let cid = match value {
                        ValueRef::Scalar(service_result_cid) => service_result_cid,
                        ValueRef::Stream {
                            cid: service_result_cid,
                            ..
                        } => service_result_cid,
                        // Cannot resolve
                        ValueRef::Unused(_value_cid) => continue,
                    };

                    let service_result_agg = data.cid_info.service_result_store.get(cid).unwrap();
                    let value = data
                        .cid_info
                        .value_store
                        .get(&service_result_agg.value_cid)
                        .unwrap()
                        .get_value();

                    if let JValue::String(ref var_name) = &*value {
                        let current_count: usize = calls_count.get(var_name).copied().unwrap_or_default();
                        calls_count.insert(var_name.to_owned(), current_count + 1);
                    }
                }
            }
        }
    }

    // $stream_1 contains 4 generation each with 2 elements, it means that inner fold for $stream_2
    // runs 2*4 times and produces 2*4 fold states, and 1 state comes from fold for $stream_1
    assert_eq!(fold_states_count, 2 * 4 + 1);

    for (call_result, call_count) in calls_count {
        if call_result.as_str() < "peer_4" {
            assert_eq!(call_count, 2);
        } else {
            assert_eq!(call_count, 16);
        }
    }
}

#[test]
fn test_merge_scalar_match() {
    let air = r#"(call "peer" ("" "") [] var)"#;

    let mut avm = create_avm(echo_call_service(), "peer");
    let mut cid_store = ExecutionCidState::new();

    let trace = ExecutionTrace::from(vec![scalar_tracked!(42, cid_store, peer = "peer")]);
    let data = raw_data_from_trace(trace, cid_store);
    checked_call_vm!(avm, <_>::default(), air, data.clone(), data);
}

#[test]
fn test_merge_scalar_mismatch() {
    let air = r#"(call "peer" ("" "") [] var)"#;
    let mut avm = create_avm(echo_call_service(), "peer");

    let mut cid_state1 = ExecutionCidState::default();
    let mut cid_state2 = ExecutionCidState::default();
    let trace1 = ExecutionTrace::from(vec![scalar_tracked!(42, cid_state1, peer = "peer")]);
    let trace2 = ExecutionTrace::from(vec![scalar_tracked!(43, cid_state2, peer = "peer")]);
    let cid1 = extract_service_result_cid(&trace1[0.into()]);
    let cid2 = extract_service_result_cid(&trace2[0.into()]);
    let data1 = raw_data_from_trace(trace1, cid_state1);
    let data2 = raw_data_from_trace(trace2, cid_state2);

    let result = avm.call(air, data1, data2, <_>::default()).unwrap();
    assert_eq!(result.ret_code, 20000);
    assert_eq!(
        result.error_message,
        format!(
            concat!(
                r#"on instruction 'call "peer" ("" "") [] var' trace handler encountered an error:"#,
                r#" values in call results are not equal:"#,
                r#" Scalar({:?})"#,
                r#" != Scalar({:?})"#
            ),
            cid1, cid2
        )
    );
}

#[test]
fn test_merge_stream_match() {
    let air = r#"(call "peer" ("" "") [] $var)"#;

    let mut avm = create_avm(echo_call_service(), "peer");
    let mut cid_store = ExecutionCidState::new();

    let trace = ExecutionTrace::from(vec![stream_tracked!(42, 0, cid_store, peer = "peer")]);
    let data = raw_data_from_trace(trace, cid_store);
    checked_call_vm!(avm, <_>::default(), air, data.clone(), data);
}

#[test]
fn test_merge_stream_match_gen() {
    let air = r#"(call "peer" ("" "") [] $var)"#;
    let mut avm = create_avm(echo_call_service(), "peer");

    let mut cid_state1 = ExecutionCidState::default();
    let mut cid_state2 = ExecutionCidState::default();
    let trace1 = ExecutionTrace::from(vec![stream_tracked!(42, 0, cid_state1, peer = "peer")]);
    let trace2 = ExecutionTrace::from(vec![stream_tracked!(42, 1, cid_state2, peer = "peer")]);
    let data1 = raw_data_from_trace(trace1, cid_state1);
    let data2 = raw_data_from_trace(trace2, cid_state2);
    checked_call_vm!(avm, <_>::default(), air, data1, data2);
}

#[test]
fn test_merge_stream_mismatch() {
    let air = r#"(call "peer" ("" "") [] $var)"#;
    let mut avm = create_avm(echo_call_service(), "peer");

    let mut cid_state1 = ExecutionCidState::default();
    let mut cid_state2 = ExecutionCidState::default();
    let trace1 = ExecutionTrace::from(vec![stream_tracked!(42, 0, cid_state1, peer = "peer")]);
    let trace2 = ExecutionTrace::from(vec![stream_tracked!(43, 0, cid_state2, peer = "peer")]);
    let cid1 = extract_service_result_cid(&trace1[0.into()]);
    let cid2 = extract_service_result_cid(&trace2[0.into()]);
    let data1 = raw_data_from_trace(trace1, cid_state1);
    let data2 = raw_data_from_trace(trace2, cid_state2);

    let result = avm.call(air, data1, data2, <_>::default()).unwrap();
    assert_eq!(result.ret_code, 20000);
    assert_eq!(
        result.error_message,
        format!(
            concat!(
                r#"on instruction 'call "peer" ("" "") [] $var' trace handler encountered an error:"#,
                r#" values in call results are not equal:"#,
                r#" Stream {{ cid: {:?}, generation: 0 }}"#,
                r#" != Stream {{ cid: {:?}, generation: 0 }}"#
            ),
            cid1, cid2
        )
    );
}

#[test]
fn test_merge_unused_match() {
    let air = r#"(call "peer" ("" "") [])"#;

    let mut avm = create_avm(echo_call_service(), "peer");

    let trace = ExecutionTrace::from(vec![unused!(42, peer = "peer")]);
    let data = raw_data_from_trace(trace, <_>::default());

    checked_call_vm!(avm, <_>::default(), air, data.clone(), data);
}

#[test]
fn test_merge_unused_mismatch() {
    let air = r#"(call "peer" ("" "") [])"#;
    let mut avm = create_avm(echo_call_service(), "peer");

    let trace1 = ExecutionTrace::from(vec![unused!(42, peer = "peer")]);
    let trace2 = ExecutionTrace::from(vec![unused!(43, peer = "peer")]);
    let data1 = raw_data_from_trace(trace1, <_>::default());
    let data2 = raw_data_from_trace(trace2, <_>::default());

    let result = avm.call(air, data1, data2, <_>::default()).unwrap();
    // TODO rewrite here and above with assert_error_eq
    assert_eq!(result.ret_code, 20000);
    assert_eq!(
        result.error_message,
        concat!(
            r#"on instruction 'call "peer" ("" "") [] ' trace handler encountered an error:"#,
            r#" values in call results are not equal:"#,
            r#" Unused(CID("bagaaihra3ijwi5gxk5odex3qfo32u5prci4giaz4ysel67m4a5hk3l432djq"))"#,
            r#" != Unused(CID("bagaaihrahhyeotni37z6kds47boxa2llqffxlz4vqt7jbt76jeimm6eu7uhq"))"#
        )
    );
}
