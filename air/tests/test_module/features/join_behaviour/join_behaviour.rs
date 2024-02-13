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

use air::CatchableError;
use air::LambdaError;
use air_test_utils::prelude::*;

#[test]
fn dont_wait_on_json_path() {
    let status = json!({
        "err_msg": "",
        "is_authenticated": 1,
        "ret_code": 0,
    });

    let msg = json!("some message");

    let variables = maplit::hashmap!(
        "msg".to_string() => msg,
        "status".to_string() => status,
    );

    let set_variables_call_service = set_variables_call_service(variables, VariableOptionSource::Argument(0));

    let set_variable_peer_id = "set_variable";
    let mut set_variable_vm = create_avm(set_variables_call_service, set_variable_peer_id);

    let local_peer_id = "local_peer_id";
    let mut local_vm = create_avm(unit_call_service(), local_peer_id);

    let script = format!(
        r#"
        (seq
            (seq
                (call "{set_variable_peer_id}" ("" "") ["msg"] msg)
                (call "{set_variable_peer_id}" ("" "") ["status"] status)
            )
            (seq
                (call "{local_peer_id}" ("op" "identity") [])
                (seq
                    (call "{local_peer_id}" ("history" "add") [msg status.$.is_authenticated!] auth_result)
                    (call %init_peer_id% ("returnService" "run") [auth_result])
                )
            )
        )
    "#
    );

    let test_params = TestRunParameters::default();
    let result = checked_call_vm!(set_variable_vm, test_params.clone(), &script, "", "");
    let result = checked_call_vm!(local_vm, test_params.clone(), script, "", result.data);

    assert_eq!(result.next_peer_pks, vec![test_params.init_peer_id]);
}

#[test]
fn dont_wait_on_json_path_on_scalars() {
    let array = json!([1u32, 2u32, 3u32, 4u32, 5u32]);

    let object = json!({
        "err_msg": "",
        "is_authenticated": 1i32,
        "ret_code": 0i32,
    });

    let variables = maplit::hashmap!(
        "array".to_string() => array.clone(),
        "object".to_string() => object.clone(),
    );

    let set_variables_call_service = set_variables_call_service(variables, VariableOptionSource::Argument(0));

    let set_variable_peer_id = "set_variable";
    let mut set_variable_vm = create_avm(set_variables_call_service, set_variable_peer_id);

    let array_consumer_peer_id = "array_consumer_peer_id";
    let mut array_consumer = create_avm(unit_call_service(), array_consumer_peer_id);

    let object_consumer_peer_id = "object_consumer_peer_id";
    let mut object_consumer = create_avm(unit_call_service(), object_consumer_peer_id);

    let script = format!(
        r#"
        (seq
            (call "{set_variable_peer_id}" ("" "") ["array"] array)
            (call "{array_consumer_peer_id}" ("" "") [array.$.[5]!] auth_result)
        )
    "#
    );

    let result = call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let array_result = call_vm!(array_consumer, <_>::default(), &script, "", result.data);

    let expected_error = CatchableError::LambdaApplierError(LambdaError::ValueNotContainSuchArrayIdx {
        value: array.into(),
        idx: 5,
    });
    assert!(check_error(&array_result, expected_error));

    let script = format!(
        r#"
        (seq
            (call "{set_variable_peer_id}" ("" "") ["object"] object)
            (call "{object_consumer_peer_id}" ("" "") [object.$.non_exist_path])
        )
    "#
    );

    let result = call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let object_result = call_vm!(object_consumer, <_>::default(), script, "", result.data);

    let expected_error = CatchableError::LambdaApplierError(LambdaError::ValueNotContainSuchField {
        value: object.into(),
        field_name: "non_exist_path".to_string(),
    });

    assert!(check_error(&object_result, expected_error));
}

#[test]
fn ap_scalar_with_join_behaviour() {
    let peer_1_id = "peer_1_id";
    let peer_2_id = "peer_2_id";

    let mut peer_1 = create_avm(unit_call_service(), peer_1_id);

    let script = format!(
        r#"
        (par
            (call "{peer_2_id}" ("" "") [] join_var)
            (xor
                (ap join_var other_var)
                (call "{peer_1_id}" ("" "") []) ;; this call shouldn't be called
            )
        )
    "#
    );

    let result = checked_call_vm!(peer_1, <_>::default(), script, "", "");
    let trace = trace_from_result(&result);
    assert_eq!(trace.len(), 2);
}

#[test]
fn ap_stream_with_join_behaviour() {
    let peer_1_id = "peer_1_id";
    let peer_2_id = "peer_2_id";

    let mut peer_1 = create_avm(unit_call_service(), peer_1_id);

    let script = format!(
        r#"
        (par
            (call "{peer_2_id}" ("" "") [] join_var)
            (xor
                (ap join_var $stream)
                (call "{peer_1_id}" ("" "") []) ;; this call shouldn't be called
            )
        )
    "#
    );

    let result = checked_call_vm!(peer_1, <_>::default(), script, "", "");
    let trace = trace_from_result(&result);
    assert_eq!(trace.len(), 2);
}

#[test]
fn match_with_join_behaviour() {
    let peer_1_id = "peer_1_id";
    let peer_2_id = "peer_2_id";

    let mut peer_1 = create_avm(unit_call_service(), peer_1_id);

    let script = format!(
        r#"
        (par
            (call "{peer_2_id}" ("" "") [] join_var)
            (xor
                (match join_var 1
                    (null)
                )
                (call "{peer_1_id}" ("" "") []) ;; this call shouldn't be called
            )
        )
    "#
    );

    let result = checked_call_vm!(peer_1, <_>::default(), script, "", "");
    let trace = trace_from_result(&result);
    assert_eq!(trace.len(), 2);
}

#[test]
fn mismatch_with_join_behaviour() {
    let peer_1_id = "peer_1_id";
    let peer_2_id = "peer_2_id";

    let mut peer_1 = create_avm(unit_call_service(), peer_1_id);

    let script = format!(
        r#"
        (par
            (call "{peer_2_id}" ("" "") [] join_var)
            (xor
                (mismatch join_var 1
                    (null)
                )
                (call "{peer_1_id}" ("" "") []) ;; this call shouldn't be called
            )
        )
    "#
    );

    let result = checked_call_vm!(peer_1, <_>::default(), script, "", "");
    let trace = trace_from_result(&result);
    assert_eq!(trace.len(), 2);
}

#[test]
fn fold_with_join_behaviour() {
    let peer_1_id = "peer_1_id";
    let peer_2_id = "peer_2_id";

    let mut peer_1 = create_avm(unit_call_service(), peer_1_id);

    let script = format!(
        r#"
        (par
            (call "{peer_2_id}" ("" "") [] join_var)
            (xor
                (fold join_var iterator
                    (null)
                )
                (call "{peer_1_id}" ("" "") []) ;; this call shouldn't be called
            )
        )
    "#
    );

    let result = checked_call_vm!(peer_1, <_>::default(), script, "", "");
    let trace = trace_from_result(&result);
    assert_eq!(trace.len(), 2);
}

#[test]
fn canon_with_empty_behaviour() {
    let peer_id = "peer_id";

    let mut peer_2 = create_avm(unit_call_service(), peer_id);

    let script = format!(
        r#"
        (canon "{peer_id}" $stream #canon_stream)
    "#
    );

    let result = checked_call_vm!(peer_2, <_>::default(), script, "", "");
    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![executed_state::canon(
        json!({"tetraplet": {"function_name": "", "json_path": "", "peer_pk": "peer_id", "service_id": ""}, "values": []}),
    )];

    assert_eq!(actual_trace, expected_trace);
}
