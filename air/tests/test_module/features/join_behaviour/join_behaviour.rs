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

    let script = f!(r#"
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
    "#);

    let init_peer_id = "asd";
    let result = checked_call_vm!(set_variable_vm, init_peer_id, &script, "", "");
    let result = checked_call_vm!(local_vm, init_peer_id, script, "", result.data);

    assert_eq!(result.next_peer_pks, vec![init_peer_id.to_string()]);
}

#[test]
fn wait_on_stream_json_path_by_id() {
    let local_peer_id = "local_peer_id";
    let mut local_vm = create_avm(unit_call_service(), local_peer_id);

    let non_join_stream_script = f!(r#"
    (par
        (call "{local_peer_id}" ("" "") [] $status)
        (call "{local_peer_id}" ("history" "add") [$status.$[0]!])
     )"#);

    let result = checked_call_vm!(local_vm, "", non_join_stream_script, "", "");
    let actual_trace = trace_from_result(&result);

    assert_eq!(actual_trace.len(), 3);

    let join_stream_script = f!(r#"
    (par
        (call "{local_peer_id}" ("" "") [] $status)
        (call "{local_peer_id}" ("history" "add") [$status.$[1]!]) ; $status stream here has only one value
     )"#);

    let result = checked_call_vm!(local_vm, "", join_stream_script, "", "");
    let actual_trace = trace_from_result(&result);

    assert_eq!(actual_trace.len(), 2); // par and the first call emit traces, second call doesn't
}

#[test]
fn wait_on_empty_stream_json_path() {
    let local_peer_id = "local_peer_id";
    let mut local_vm = create_avm(echo_call_service(), local_peer_id);

    let join_stream_script = format!(
        r#"
    (seq
        (seq
            (call "{local_peer_id}" ("" "") [[]] nodes)
            (fold nodes n
                (par
                    (call n ("" "") [n] $ns)
                    (next n)
                )
            )
        )
        (call "{local_peer_id}" ("" "") [$ns.$.[0] $ns.$.[1] $ns])
     )"#
    );

    let result = checked_call_vm!(local_vm, "", join_stream_script, "", "");
    let actual_trace = trace_from_result(&result);

    assert_eq!(actual_trace.len(), 1); // only the first call should produce a trace
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

    let script = f!(r#"
        (seq
            (call "{set_variable_peer_id}" ("" "") ["array"] array)
            (call "{array_consumer_peer_id}" ("" "") [array.$.[5]!] auth_result)
        )
    "#);

    let init_peer_id = "asd";
    let result = call_vm!(set_variable_vm, init_peer_id, &script, "", "");
    let array_result = call_vm!(array_consumer, init_peer_id, &script, "", result.data.clone());

    let expected_error =
        CatchableError::LambdaApplierError(LambdaError::ValueNotContainSuchArrayIdx { value: array, idx: 5 });
    assert!(check_error(&array_result, expected_error));

    let script = f!(r#"
        (seq
            (call "{set_variable_peer_id}" ("" "") ["object"] object)
            (call "{object_consumer_peer_id}" ("" "") [object.$.non_exist_path])
        )
    "#);

    let init_peer_id = "asd";
    let result = call_vm!(set_variable_vm, init_peer_id, &script, "", "");
    let object_result = call_vm!(object_consumer, init_peer_id, script, "", result.data);

    let expected_error = CatchableError::LambdaApplierError(LambdaError::ValueNotContainSuchField {
        value: object,
        field_name: "non_exist_path".to_string(),
    });

    assert!(check_error(&object_result, expected_error));
}

#[test]
fn match_with_join_behaviour() {
    let peer_1_id = "peer_1_id";
    let peer_2_id = "peer_2_id";

    let mut peer_1 = create_avm(unit_call_service(), peer_1_id);

    let script = f!(r#"
        (par
            (call "{peer_2_id}" ("" "") [] join_var)
            (xor
                (match join_var 1
                    (null)
                )
                (call "{peer_1_id}" ("" "") []) ;; this call shouldn't be called
            )
        )
    "#);

    let result = checked_call_vm!(peer_1, "", script, "", "");
    let trace = trace_from_result(&result);
    assert_eq!(trace.len(), 2);
}

#[test]
fn mismatch_with_join_behaviour() {
    let peer_1_id = "peer_1_id";
    let peer_2_id = "peer_2_id";

    let mut peer_1 = create_avm(unit_call_service(), peer_1_id);

    let script = f!(r#"
        (par
            (call "{peer_2_id}" ("" "") [] join_var)
            (xor
                (mismatch join_var 1
                    (null)
                )
                (call "{peer_1_id}" ("" "") []) ;; this call shouldn't be called
            )
        )
    "#);

    let result = checked_call_vm!(peer_1, "", script, "", "");
    let trace = trace_from_result(&result);
    assert_eq!(trace.len(), 2);
}

#[test]
fn fold_with_join_behaviour() {
    let peer_1_id = "peer_1_id";
    let peer_2_id = "peer_2_id";

    let mut peer_1 = create_avm(unit_call_service(), peer_1_id);

    let script = f!(r#"
        (par
            (call "{peer_2_id}" ("" "") [] join_var)
            (xor
                (fold join_var iterator
                    (null)
                )
                (call "{peer_1_id}" ("" "") []) ;; this call shouldn't be called
            )
        )
    "#);

    let result = checked_call_vm!(peer_1, "", script, "", "");
    let trace = trace_from_result(&result);
    assert_eq!(trace.len(), 2);
}
