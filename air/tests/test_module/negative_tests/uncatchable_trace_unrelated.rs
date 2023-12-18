/*
 * Copyright 2023 Fluence Labs Limited
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

use air::interpreter_data::ExecutedState;
use air::ExecutionCidState;
use air::UncatchableError::*;
use air_interpreter_data::ValueRef;
use air_interpreter_data::VmValue;
use air_test_framework::AirScriptExecutor;
use air_test_utils::prelude::*;

#[test]
fn fold_state_not_found() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let arg = json!([1, 2,]);
    let mut peer_vm_1 = create_avm(set_variable_call_service(arg), vm_peer_id_1);

    let script = format!(
        r#"
         (seq
             (seq
                 (call "vm_peer_id_1" ("" "") [] some)
                 (fold some i
                     (next i)
                 )
             )
             (next i)
         )
     "#
    );

    let result = peer_vm_1.call(script, "", "", <_>::default()).unwrap();
    let expected_error = FoldStateNotFound(String::from("i"));
    assert!(check_error(&result, expected_error));
}

#[test]
fn iterable_shadowing() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let arg = json!([1, 2,]);
    let mut peer_vm_1 = create_avm(set_variable_call_service(arg), vm_peer_id_1);

    let script = format!(
        r#"
         (seq
             (call "vm_peer_id_1" ("" "") [] some)
             (fold some i
                 (call "vm_peer_id_1" ("" "") [] i)
             )
         )
     "#
    );

    let result = peer_vm_1.call(script, "", "", <_>::default()).unwrap();
    let expected_error = IterableShadowing(String::from("i"));
    assert!(check_error(&result, expected_error));
}

#[test]
fn call_result_not_correspond_to_instr() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let arg = json!([1, 2,]);
    let mut peer_vm_1 = create_avm(set_variable_call_service(arg.clone()), vm_peer_id_1);

    let script = format!(
        r#"
         (call "vm_peer_id_1" ("" "") [] $some)
         "#
    );

    let scalar_value = 42;
    let wrong_trace = vec![scalar!(scalar_value)];
    let cid = extract_service_result_cid(&wrong_trace[0]);
    let data = raw_data_from_trace(wrong_trace, <_>::default());

    let result = peer_vm_1.call(script, "", data, <_>::default()).unwrap();
    let value_ref = ValueRef::Scalar(cid);
    let expected_error = CallResultNotCorrespondToInstr(value_ref);
    assert!(check_error(&result, expected_error), "{:?}", result);
}

#[test]
fn shadowing_is_not_allowed() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let mut peer_vm_1 = create_avm(unit_call_service(), vm_peer_id_1);
    let var_name = String::from("some");
    let script = format!(
        r#"
     (seq
         (ap 42 {var_name})
         (ap 42 {var_name})
     )
     "#
    );

    let result = peer_vm_1.call(script, "", "", <_>::default()).unwrap();
    let expected_error = ShadowingIsNotAllowed(var_name);
    assert!(check_error(&result, expected_error));
}

#[test]
fn value_for_cid_not_found() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let arg = json!([1, 2,]);
    let mut peer_vm_1 = create_avm(set_variable_call_service(arg), vm_peer_id_1);

    let script = format!(
        r#"
         (call "vm_peer_id_1" ("" "") [] some)
     "#
    );

    let wrong_trace = vec![scalar!(42)];
    let cid = extract_service_result_cid(&wrong_trace[0]);
    let data = raw_data_from_trace(wrong_trace, <_>::default());
    let result = peer_vm_1.call(script, "", data, <_>::default()).unwrap();

    let missing_cid = cid.get_inner();
    let expected_error = ValueForCidNotFound("service result aggregate", missing_cid);
    assert!(check_error(&result, expected_error));
}

#[test]
fn malformed_call_service_failed() {
    let peer_id = "init_peer_id";
    let mut cid_state = ExecutionCidState::new();

    // Craft an artificial incorrect error result
    let value = json!("error");
    let value_cid = cid_state
        .value_tracker
        .track_raw_value(VmValue::from_value(value.clone()));
    let tetraplet = SecurityTetraplet::literal_tetraplet(peer_id);
    let tetraplet_cid = cid_state.tetraplet_tracker.track_value(tetraplet).unwrap();
    let service_result_agg = ServiceResultCidAggregate {
        value_cid,
        argument_hash: "bagaaihra2u6rrqrsclvhwyyalff3rg6omaqy63x7foowfc4myqwt46n32wvq".into(),
        tetraplet_cid,
    };
    let service_result_agg_cid = cid_state
        .service_result_agg_tracker
        .track_value(service_result_agg)
        .unwrap();

    let trace = ExecutionTrace::from(vec![ExecutedState::Call(CallResult::Failed(service_result_agg_cid))]);
    let data = raw_data_from_trace(trace, cid_state);

    let mut vm = create_avm(unit_call_service(), peer_id);
    let air = format!(r#"(call "{peer_id}" ("" "") [] var)"#);
    let result = vm.call(&air, vec![], data, TestRunParameters::default()).unwrap();
    let expected_serde_error = serde_json::from_value::<CallServiceFailed>(value).unwrap_err();
    let expected_error = MalformedCallServiceFailed(expected_serde_error);
    assert_error_eq!(&result, expected_error);
}

#[test]
fn recursive_stream_size_limit() {
    let vm_peer_id_1 = "vm_peer_id_1";

    let script = format!(
        r#"
        (seq
            (ap 42 $stream)
            (fold $stream i
                (seq
                    (ap i $stream)
                    (next i)
                )
            )
        )"#
    );

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(vm_peer_id_1), &script)
        .expect("invalid test AIR script");
    let result = executor.execute_all(vm_peer_id_1).unwrap();
    let result = result.last().unwrap();

    let expected_error = StreamSizeLimitExceeded;
    assert!(check_error(&result, expected_error));
}
