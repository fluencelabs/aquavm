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

use air::CatchableError;
use air::LambdaError;
use air::StreamMapError::FloatMapKeyIsUnsupported;
use air::StreamMapError::UnsupportedMapKeyType;

use air_test_utils::prelude::*;

#[test]
fn local_service_error() {
    let client_peer_id = "some_peer_id";
    let script = f!(r#"
        (call "{client_peer_id}" ("m" "f") ["arg1"] other)
    "#);

    let mut client_vm = create_avm(echo_call_service(), client_peer_id);
    let result = client_vm
        .runner
        .call(&script, "", "", client_peer_id, 0, 0, None, <_>::default())
        .unwrap();

    let err_msg = "some error".to_string();
    let call_service_result = air_test_utils::CallServiceResult::err(10000, json!(err_msg));
    let call_results_4_call = maplit::hashmap!(
        1 => call_service_result,
    );
    let result = client_vm
        .runner
        .call(script, "", result.data, client_peer_id, 0, 0, None, call_results_4_call)
        .unwrap();

    let err_msg = std::rc::Rc::new(f!("\"{err_msg}\""));
    let expected_error = CatchableError::LocalServiceError(10000, err_msg);
    assert!(check_error(&result, expected_error));
}

#[test]
fn variable_not_found_ap_scalar() {
    let vm_2_peer_id = "vm_2_peer_id";
    let var_name = "scalar_1".to_string();
    let script = f!(r#"
        (par
            (match 0 1
                (call "vm_1_peer_id" ("m" "f") ["scalar_1_result"] {var_name})
            )
            (ap {var_name} scalar_2)
        )
        "#);
    let mut vm_2 = create_avm(echo_call_service(), vm_2_peer_id);
    let result = vm_2
        .runner
        .call(&script, "", "", vm_2_peer_id, 0, 0, None, <_>::default())
        .unwrap();
    let expected_error = CatchableError::VariableNotFound(var_name);
    assert!(check_error(&result, expected_error));
}

#[test]
fn variable_not_found_ap_canon_stream() {
    let vm_2_peer_id = "vm_2_peer_id";
    let var_name = "#canon".to_string();
    let script = f!(r#"
        (par
            (match 0 1
                (seq
                    (call "vm_1_peer_id" ("m" "f") ["scalar_1_result"] $stream)
                    (canon "vm_1_peer_id" $stream {var_name})
                )
            )
            (ap {var_name} scalar_2)
        )
        "#);
    let mut vm_2 = create_avm(echo_call_service(), vm_2_peer_id);
    let result = vm_2
        .runner
        .call(&script, "", "", vm_2_peer_id, 0, 0, None, <_>::default())
        .unwrap();
    let expected_error = CatchableError::VariableNotFound(var_name);
    assert!(check_error(&result, expected_error));
}

#[test]
fn fold_iterates_over_non_array_scalar_iterable() {
    let vm_2_peer_id = "vm_2_peer_id";
    let var_name = "outcome".to_string();
    let script = f!(r#"
        (seq
            (call "{vm_2_peer_id}" ("m" "f") [] {var_name})
            (fold {var_name} i
                (seq
                    (null)
                    (next i)
                )
            )
        )
        "#);
    let unsupported_jvalue = json!({"attr": 42, });
    let mut vm_2 = create_avm(set_variable_call_service(unsupported_jvalue.clone()), vm_2_peer_id);
    let result = vm_2.call(&script, "", "", <_>::default()).unwrap();
    let expected_error = CatchableError::FoldIteratesOverNonArray(unsupported_jvalue, var_name);
    assert!(check_error(&result, expected_error));
}

#[test]
fn fold_iterates_over_non_array_scalar_lambda_iterable() {
    let vm_2_peer_id = "vm_2_peer_id";
    let var_name = "outcome".to_string();
    let scalar_int = 42;
    let script = f!(r#"
        (seq
            (call "{vm_2_peer_id}" ("m" "f") [] {var_name})
            (fold {var_name}.$.int i
                (seq
                    (null)
                    (next i)
                )
            )
        )
        "#);
    let unsupported_jvalue = json!({"int": scalar_int, });
    let mut vm_2 = create_avm(set_variable_call_service(unsupported_jvalue.clone()), vm_2_peer_id);
    let result = vm_2.call(&script, "", "", <_>::default()).unwrap();
    let expected_error = CatchableError::FoldIteratesOverNonArray(json!(scalar_int), ".$.int".to_string());
    assert!(check_error(&result, expected_error));
}

#[test]
fn non_string_value_in_triplet_resolution() {
    let vm_2_peer_id = "vm_2_peer_id";
    let scalar_int = 42;
    let var_name = "var_name".to_string();
    let script = f!(r#"
        (seq
            (ap {scalar_int} {var_name})
            (call {var_name} ("" "") [] outcome)
        )
        "#);
    let mut vm_2 = create_avm(unit_call_service(), vm_2_peer_id);
    let result = vm_2.call(&script, "", "", <_>::default()).unwrap();
    let expected_error = CatchableError::NonStringValueInTripletResolution {
        variable_name: var_name.clone(),
        actual_value: json!(scalar_int),
    };
    assert!(check_error(&result, expected_error));

    let script = f!(r#"
        (seq
            (seq
                (call "{vm_2_peer_id}" ("m" "f") [] {var_name})
                (null)
            )
            (call {var_name}.$.int ("" "") [] outcome)
        )
        "#);
    let mut vm_2 = create_avm(set_variable_call_service(json!({"int": scalar_int, })), vm_2_peer_id);
    let result = vm_2.call(&script, "", "", <_>::default()).unwrap();
    let expected_error = CatchableError::NonStringValueInTripletResolution {
        variable_name: var_name,
        actual_value: json!(scalar_int),
    };
    assert!(check_error(&result, expected_error));

    let var_name = "#canon".to_string();
    let script = f!(r#"
        (seq
            (seq
                (ap {scalar_int} $stream)
                (canon "{vm_2_peer_id}" $stream {var_name})
            )
            (call #canon.$.[0] ("" "") [] outcome)
        )
        "#);
    let mut vm_2 = create_avm(echo_call_service(), vm_2_peer_id);
    let result = vm_2.call(&script, "", "", <_>::default()).unwrap();
    let expected_error = CatchableError::NonStringValueInTripletResolution {
        variable_name: var_name.clone(),
        actual_value: json!(scalar_int),
    };
    assert!(check_error(&result, expected_error));
}

#[test]
fn non_string_value_in_triplet_resolution_module_name() {
    let vm_2_peer_id = "vm_2_peer_id";
    let scalar_int = 42;
    let var_name = "var_name".to_string();
    let script = f!(r#"
        (seq
            (ap {scalar_int} {var_name})
            (call "{vm_2_peer_id}" ({var_name} "") [] outcome)
        )
        "#);
    let mut vm_2 = create_avm(unit_call_service(), vm_2_peer_id);
    let result = vm_2.call(&script, "", "", <_>::default()).unwrap();
    let expected_error = CatchableError::NonStringValueInTripletResolution {
        variable_name: var_name.clone(),
        actual_value: json!(scalar_int),
    };

    assert!(check_error(&result, expected_error));
    let script = f!(r#"
        (seq
            (seq
                (call "{vm_2_peer_id}" ("m" "f") [] {var_name})
                (null)
            )
            (call "{vm_2_peer_id}" ({var_name}.$.int "") [] outcome)
        )
        "#);
    let mut vm_2 = create_avm(set_variable_call_service(json!({"int": scalar_int, })), vm_2_peer_id);
    let result = vm_2.call(&script, "", "", <_>::default()).unwrap();
    let expected_error = CatchableError::NonStringValueInTripletResolution {
        variable_name: var_name,
        actual_value: json!(scalar_int),
    };
    assert!(check_error(&result, expected_error));

    let var_name = "#canon".to_string();
    let script = f!(r#"
        (seq
            (seq
                (ap {scalar_int} $stream)
                (canon "{vm_2_peer_id}" $stream {var_name})
            )
            (call "{vm_2_peer_id}" (#canon.$.[0] "") [] outcome)
        )
        "#);
    let mut vm_2 = create_avm(echo_call_service(), vm_2_peer_id);
    let result = vm_2.call(&script, "", "", <_>::default()).unwrap();
    let expected_error = CatchableError::NonStringValueInTripletResolution {
        variable_name: var_name.clone(),
        actual_value: json!(scalar_int),
    };
    assert!(check_error(&result, expected_error));
}

#[test]
fn non_string_value_in_triplet_resolution_function_name() {
    let vm_2_peer_id = "vm_2_peer_id";
    let scalar_int = 42;
    let var_name = "var_name".to_string();
    let script = f!(r#"
        (seq
            (ap {scalar_int} {var_name})
            (call "{vm_2_peer_id}" ("" {var_name}) [] outcome)
        )
        "#);
    let mut vm_2 = create_avm(unit_call_service(), vm_2_peer_id);
    let result = vm_2.call(&script, "", "", <_>::default()).unwrap();
    let expected_error = CatchableError::NonStringValueInTripletResolution {
        variable_name: var_name.clone(),
        actual_value: json!(scalar_int),
    };

    assert!(check_error(&result, expected_error));
    let script = f!(r#"
        (seq
            (seq
                (call "{vm_2_peer_id}" ("m" "f") [] {var_name})
                (null)
            )
            (call "{vm_2_peer_id}" ("" {var_name}.$.int) [] outcome)
        )
        "#);
    let mut vm_2 = create_avm(set_variable_call_service(json!({"int": scalar_int, })), vm_2_peer_id);
    let result = vm_2.call(&script, "", "", <_>::default()).unwrap();
    let expected_error = CatchableError::NonStringValueInTripletResolution {
        variable_name: var_name,
        actual_value: json!(scalar_int),
    };
    assert!(check_error(&result, expected_error));

    let var_name = "#canon".to_string();
    let script = f!(r#"
        (seq
            (seq
                (ap {scalar_int} $stream)
                (canon "{vm_2_peer_id}" $stream {var_name})
            )
            (call "{vm_2_peer_id}" ("" #canon.$.[0]) [] outcome)
        )
        "#);
    let mut vm_2 = create_avm(echo_call_service(), vm_2_peer_id);
    let result = vm_2.call(&script, "", "", <_>::default()).unwrap();
    let expected_error = CatchableError::NonStringValueInTripletResolution {
        variable_name: var_name.clone(),
        actual_value: json!(scalar_int),
    };
    assert!(check_error(&result, expected_error));
}

#[test]
fn value_not_contain_such_array_idx_ap() {
    let vm_1_peer_id = "vm_1_peer_id";
    let idx = 42;
    let script = f!(r#"
        (seq
            (call "{vm_1_peer_id}" ("m" "f") [] result)
            (ap result.$.[{idx}].atrib outcome)
        )
        "#);
    let wrong_jvalue = json!(["a", "b"]);
    let mut vm_2 = create_avm(set_variable_call_service(wrong_jvalue.clone()), vm_1_peer_id);
    let result = vm_2.call(&script, "", "", <_>::default()).unwrap();

    let expected_error = CatchableError::LambdaApplierError(LambdaError::ValueNotContainSuchArrayIdx {
        value: wrong_jvalue,
        idx: idx,
    });
    assert!(check_error(&result, expected_error));
}

#[test]
fn field_accessor_applied_to_stream() {
    let vm_peer_id_1 = "vm_peer_id_1";

    let arg = json!({"a": 1,});
    let mut peer_vm_1 = create_avm(set_variable_call_service(arg), vm_peer_id_1);
    let field_name = "int".to_string();
    let script = f!(r#"
        (seq
            (canon "{vm_peer_id_1}" $stream #canon_stream)
            (call "{vm_peer_id_1}" ("m" "f") [#canon_stream.$.{field_name}] output)
        )
        "#);

    let result = peer_vm_1.call(script.clone(), "", "", <_>::default()).unwrap();
    let expected_error = air::CatchableError::LambdaApplierError(air::LambdaError::FieldAccessorAppliedToStream {
        field_name: field_name,
    });
    assert!(check_error(&result, expected_error));
}

#[test]
fn array_accessor_not_match_value() {
    let vm_peer_id_1 = "vm_peer_id_1";

    let arg = json!({"a": 1,});
    let mut peer_vm_1 = create_avm(set_variable_call_service(arg.clone()), vm_peer_id_1);
    let idx = 42;
    let script = f!(r#"
        (seq
            (call "{vm_peer_id_1}" ("m" "f") [] nonarray)
            (call "{vm_peer_id_1}" ("m" "f") [nonarray.$.[{idx}]] result)
        )
        "#);

    let result = peer_vm_1.call(script.clone(), "", "", <_>::default()).unwrap();
    let expected_error =
        air::CatchableError::LambdaApplierError(air::LambdaError::ArrayAccessorNotMatchValue { value: arg, idx: idx });
    assert!(check_error(&result, expected_error));
}

#[test]
fn value_not_contain_such_array_idx_call_arg_lambda() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let arg = json!([0, 1]);
    let mut peer_vm_1 = create_avm(set_variable_call_service(arg.clone()), vm_peer_id_1);
    let idx = 42;
    let script = f!(r#"
        (seq
            (call "{vm_peer_id_1}" ("m" "f") [] array)
            (call "{vm_peer_id_1}" ("m" "f") [array.$.[{idx}]] result)
        )
        "#);

    let result = peer_vm_1.call(script.clone(), "", "", <_>::default()).unwrap();
    let expected_error =
        air::CatchableError::LambdaApplierError(air::LambdaError::ValueNotContainSuchArrayIdx { value: arg, idx });
    assert!(check_error(&result, expected_error));
}

#[test]
fn value_not_contain_such_field_call_arg_lambda() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let arg = json!({"a": 1,});
    let mut peer_vm_1 = create_avm(set_variable_call_service(arg.clone()), vm_peer_id_1);
    let field_name = "b".to_string();
    let script = f!(r#"
        (seq
            (call "{vm_peer_id_1}" ("m" "f") [] obj)
            (call "{vm_peer_id_1}" ("m" "f") [obj.$.{field_name}] output)
        )
        "#);

    let result = peer_vm_1.call(script.clone(), "", "", <_>::default()).unwrap();
    let expected_error =
        air::CatchableError::LambdaApplierError(air::LambdaError::ValueNotContainSuchField { value: arg, field_name });
    assert!(check_error(&result, expected_error));
}

#[test]
fn field_accesssor_not_match_value_call_arg_lambda() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let arg = json!([0, 1]);
    let mut peer_vm_1 = create_avm(set_variable_call_service(arg.clone()), vm_peer_id_1);

    let script = f!(r#"
        (seq
            (call "{vm_peer_id_1}" ("m" "f") [] obj)
            (call "{vm_peer_id_1}" ("m" "f") [obj.$.b] output)
        )
        "#);
    let result = peer_vm_1.call(script.clone(), "", "", <_>::default()).unwrap();
    let expected_error = air::CatchableError::LambdaApplierError(air::LambdaError::FieldAccessorNotMatchValue {
        value: arg,
        field_name: "b".to_string(),
    });
    assert!(check_error(&result, expected_error));
}

#[test]
fn index_access_not_u32_i64() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let vm_peer_id_2 = "vm_peer_id_2";
    let number = serde_json::Number::from(8589934592 as i64);
    let arg = json!(number);
    let mut peer_vm_1 = create_avm(set_variable_call_service(arg.clone()), vm_peer_id_1);
    let arg = json!([0, 1]);
    let mut peer_vm_2 = create_avm(set_variable_call_service(arg.clone()), vm_peer_id_2);

    let script = f!(r#"
        (seq
            (call "{vm_peer_id_1}" ("m" "f") [] idx)
            (seq
                (call "{vm_peer_id_2}" ("m" "f") [] array)
                (call "{vm_peer_id_2}" ("m" "f") [array.$.[idx]] result)
            )
        )
        "#);

    let result = peer_vm_1.call(script.clone(), "", "", <_>::default()).unwrap();
    let result = peer_vm_2.call(script.clone(), "", result.data, <_>::default()).unwrap();
    let expected_error =
        air::CatchableError::LambdaApplierError(air::LambdaError::IndexAccessNotU32 { accessor: number });
    assert!(check_error(&result, expected_error));
}

#[test]
fn scalar_accessor_has_invalid_type_ap() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let obj_arg = json!({"a": 1,});
    let mut peer_vm_1 = create_avm(set_variable_call_service(obj_arg.clone()), vm_peer_id_1);

    let script = f!(r#"
        (seq
            (call "{vm_peer_id_1}" ("m" "f") [] array_obj)
            (seq
                (call "{vm_peer_id_1}" ("m" "f") [] array)
                (ap array.$.[array_obj] outcome)
            )
        )
        "#);

    let result = peer_vm_1.call(script.clone(), "", "", <_>::default()).unwrap();
    let expected_error = air::CatchableError::LambdaApplierError(air::LambdaError::ScalarAccessorHasInvalidType {
        scalar_accessor: obj_arg,
    });
    assert!(check_error(&result, expected_error));

    let obj_arg = json!([{"a": 1,}]);
    let mut peer_vm_1 = create_avm(set_variable_call_service(obj_arg.clone()), vm_peer_id_1);
    let result = peer_vm_1.call(script.clone(), "", "", <_>::default()).unwrap();
    let expected_error = air::CatchableError::LambdaApplierError(air::LambdaError::ScalarAccessorHasInvalidType {
        scalar_accessor: obj_arg,
    });
    assert!(check_error(&result, expected_error));
}

#[test]
fn stream_accessor_has_invalid_type() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let obj_arg = json!({"a": 1,});
    let mut peer_vm_1 = create_avm(set_variable_call_service(obj_arg.clone()), vm_peer_id_1);

    let script = f!(r#"
    (seq
        (call "{vm_peer_id_1}" ("m" "f") [] array_obj)
        (seq
            (call "{vm_peer_id_1}" ("m" "f") [] $stream)
            (seq
                (canon "{vm_peer_id_1}" $stream #canon)
                (ap #canon.$.[array_obj] outcome)
            )
        )
    )
    "#);

    let result = peer_vm_1.call(script.clone(), "", "", <_>::default()).unwrap();
    let expected_error = air::CatchableError::LambdaApplierError(air::LambdaError::StreamAccessorHasInvalidType {
        scalar_accessor: obj_arg,
    });
    assert!(check_error(&result, expected_error));

    let obj_arg = json!([{"a": 1,}]);
    let mut peer_vm_1 = create_avm(set_variable_call_service(obj_arg.clone()), vm_peer_id_1);
    let result = peer_vm_1.call(script.clone(), "", "", <_>::default()).unwrap();
    let expected_error = air::CatchableError::LambdaApplierError(air::LambdaError::StreamAccessorHasInvalidType {
        scalar_accessor: obj_arg,
    });
    assert!(check_error(&result, expected_error));
}

#[test]
fn canon_stream_not_have_enough_values_call_arg() {
    let local_peer_id = "local_peer_id";
    let mut local_vm = create_avm(echo_call_service(), local_peer_id);

    let join_stream_script = f!(r#"
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
        (seq
            (canon "{local_peer_id}" $ns #ns)
            (call "{local_peer_id}" ("" "") [#ns.$.[0] #ns.$.[1] #ns])
        )
     )"#);

    let result = local_vm.call(&join_stream_script, "", "", <_>::default()).unwrap();
    let actual_trace = trace_from_result(&result);
    assert_eq!(actual_trace.len(), 2); // only the first call and canon should produce a trace
    let expected_error =
        CatchableError::LambdaApplierError(LambdaError::CanonStreamNotHaveEnoughValues { stream_size: 0, idx: 0 });
    assert!(check_error(&result, expected_error));
}

#[test]
fn float_map_key_is_unsupported() {
    let mut local_vm = create_avm(echo_call_service(), "local_peer_id");

    let map_name = "%map";
    let join_stream_script = f!(r#"
        (ap 0.5 "serv1" %map)
     "#);

    let result = local_vm.call(&join_stream_script, "", "", <_>::default()).unwrap();
    let expected_error = CatchableError::StreamMapError(FloatMapKeyIsUnsupported {
        variable_name: String::from(map_name),
    });
    assert!(check_error(&result, expected_error));
}

#[test]
fn unsupported_map_key_type() {
    let mut local_vm = create_avm(echo_call_service(), "local_peer_id");

    let map_name = "%map";
    let join_stream_script = f!(r#"
    (seq
        (ap "a" some)
        (ap some "serv1" %map)
    )
     "#);

    let result = local_vm.call(&join_stream_script, "", "", <_>::default()).unwrap();
    let expected_error = CatchableError::StreamMapError(UnsupportedMapKeyType {
        variable_name: String::from(map_name),
    });
    assert!(check_error(&result, expected_error));
}
