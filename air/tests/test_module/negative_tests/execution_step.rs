/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use air::no_error_object;
use air::unsupported_map_key_type;
use air::CatchableError;
use air::LambdaError;

use air_test_framework::AirScriptExecutor;
use air_test_utils::prelude::*;

#[tokio::test]
async fn local_service_error() {
    let client_peer_id = "some_peer_id";
    let script = format!(
        r#"
        (call "{client_peer_id}" ("m" "f") ["arg1"] other)
    "#
    );

    let mut client_vm = create_avm(echo_call_service(), client_peer_id).await;
    let result = client_vm
        .call_single(&script, "", "", client_peer_id, 0, 0, None, <_>::default(), "")
        .await
        .unwrap();

    let err_msg = "some error".to_string();
    let call_service_result = air_test_utils::CallServiceResult::err(10000, json!(err_msg).into());
    let call_results_4_call = maplit::hashmap!(
        1 => call_service_result,
    );
    let result = client_vm
        .call_single(
            script,
            "",
            result.data,
            client_peer_id,
            0,
            0,
            None,
            call_results_4_call,
            "",
        )
        .await
        .unwrap();

    let err_msg = std::rc::Rc::new(format!("\"{err_msg}\""));
    let expected_error = CatchableError::LocalServiceError(10000, err_msg);
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn fold_iterates_over_non_array_scalar_iterable() {
    let vm_2_peer_id = "vm_2_peer_id";
    let var_name = "outcome".to_string();
    let script = format!(
        r#"
        (seq
            (call "{vm_2_peer_id}" ("m" "f") [] {var_name})
            (fold {var_name} i
                (seq
                    (null)
                    (next i)
                )
            )
        )
        "#
    );
    let unsupported_jvalue = json!({"attr": 42, });
    let mut vm_2 = create_avm(set_variable_call_service(unsupported_jvalue.clone()), vm_2_peer_id).await;
    let result = vm_2.call(&script, "", "", <_>::default()).await.unwrap();
    let expected_error = CatchableError::FoldIteratesOverNonArray(unsupported_jvalue.into(), var_name);
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn fold_iterates_over_non_array_scalar_lambda_iterable() {
    let vm_2_peer_id = "vm_2_peer_id";
    let var_name = "outcome".to_string();
    let scalar_int = 42;
    let script = format!(
        r#"
        (seq
            (call "{vm_2_peer_id}" ("m" "f") [] {var_name})
            (fold {var_name}.$.int i
                (seq
                    (null)
                    (next i)
                )
            )
        )
        "#
    );
    let unsupported_jvalue = json!({"int": scalar_int, });
    let mut vm_2 = create_avm(set_variable_call_service(unsupported_jvalue.clone()), vm_2_peer_id).await;
    let result = vm_2.call(&script, "", "", <_>::default()).await.unwrap();
    let expected_error = CatchableError::FoldIteratesOverNonArray(scalar_int.into(), ".$.int".to_string());
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn non_string_value_in_triplet_resolution() {
    let vm_2_peer_id = "vm_2_peer_id";
    let scalar_int = 42;
    let var_name = "var_name".to_string();
    let script = format!(
        r#"
        (seq
            (ap {scalar_int} {var_name})
            (call {var_name} ("" "") [] outcome)
        )
        "#
    );
    let mut vm_2 = create_avm(unit_call_service(), vm_2_peer_id).await;
    let result = vm_2.call(&script, "", "", <_>::default()).await.unwrap();
    let expected_error = CatchableError::NonStringValueInTripletResolution {
        variable_name: var_name.clone(),
        actual_value: scalar_int.into(),
    };
    assert!(check_error(&result, expected_error));

    let script = format!(
        r#"
        (seq
            (seq
                (call "{vm_2_peer_id}" ("m" "f") [] {var_name})
                (null)
            )
            (call {var_name}.$.int ("" "") [] outcome)
        )
        "#
    );
    let mut vm_2 = create_avm(set_variable_call_service(json!({"int": scalar_int, })), vm_2_peer_id).await;
    let result = vm_2.call(&script, "", "", <_>::default()).await.unwrap();
    let expected_error = CatchableError::NonStringValueInTripletResolution {
        variable_name: var_name,
        actual_value: scalar_int.into(),
    };
    assert!(check_error(&result, expected_error));

    let var_name = "#canon".to_string();
    let script = format!(
        r#"
        (seq
            (seq
                (ap {scalar_int} $stream)
                (canon "{vm_2_peer_id}" $stream {var_name})
            )
            (call #canon.$.[0] ("" "") [] outcome)
        )
        "#
    );
    let mut vm_2 = create_avm(echo_call_service(), vm_2_peer_id).await;
    let result = vm_2.call(&script, "", "", <_>::default()).await.unwrap();
    let expected_error = CatchableError::NonStringValueInTripletResolution {
        variable_name: var_name.clone(),
        actual_value: scalar_int.into(),
    };
    assert_error_eq!(&result, expected_error);
}

#[tokio::test]
async fn non_string_value_in_triplet_resolution_module_name() {
    let vm_2_peer_id = "vm_2_peer_id";
    let scalar_int = 42;
    let var_name = "var_name".to_string();
    let script = format!(
        r#"
        (seq
            (ap {scalar_int} {var_name})
            (call "{vm_2_peer_id}" ({var_name} "") [] outcome)
        )
        "#
    );
    let mut vm_2 = create_avm(unit_call_service(), vm_2_peer_id).await;
    let result = vm_2.call(&script, "", "", <_>::default()).await.unwrap();
    let expected_error = CatchableError::NonStringValueInTripletResolution {
        variable_name: var_name.clone(),
        actual_value: scalar_int.into(),
    };

    assert!(check_error(&result, expected_error));
    let script = format!(
        r#"
        (seq
            (seq
                (call "{vm_2_peer_id}" ("m" "f") [] {var_name})
                (null)
            )
            (call "{vm_2_peer_id}" ({var_name}.$.int "") [] outcome)
        )
        "#
    );
    let mut vm_2 = create_avm(set_variable_call_service(json!({"int": scalar_int, })), vm_2_peer_id).await;
    let result = vm_2.call(&script, "", "", <_>::default()).await.unwrap();
    let expected_error = CatchableError::NonStringValueInTripletResolution {
        variable_name: var_name,
        actual_value: scalar_int.into(),
    };
    assert!(check_error(&result, expected_error));

    let var_name = "#canon".to_string();
    let script = format!(
        r#"
        (seq
            (seq
                (ap {scalar_int} $stream)
                (canon "{vm_2_peer_id}" $stream {var_name})
            )
            (call "{vm_2_peer_id}" (#canon.$.[0] "") [] outcome)
        )
        "#
    );
    let mut vm_2 = create_avm(echo_call_service(), vm_2_peer_id).await;
    let result = vm_2.call(&script, "", "", <_>::default()).await.unwrap();
    let expected_error = CatchableError::NonStringValueInTripletResolution {
        variable_name: var_name.clone(),
        actual_value: scalar_int.into(),
    };
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn non_string_value_in_triplet_resolution_function_name() {
    let vm_2_peer_id = "vm_2_peer_id";
    let scalar_int = 42;
    let var_name = "var_name".to_string();
    let script = format!(
        r#"
        (seq
            (ap {scalar_int} {var_name})
            (call "{vm_2_peer_id}" ("" {var_name}) [] outcome)
        )
        "#
    );
    let mut vm_2 = create_avm(unit_call_service(), vm_2_peer_id).await;
    let result = vm_2.call(&script, "", "", <_>::default()).await.unwrap();
    let expected_error = CatchableError::NonStringValueInTripletResolution {
        variable_name: var_name.clone(),
        actual_value: scalar_int.into(),
    };

    assert!(check_error(&result, expected_error));
    let script = format!(
        r#"
        (seq
            (seq
                (call "{vm_2_peer_id}" ("m" "f") [] {var_name})
                (null)
            )
            (call "{vm_2_peer_id}" ("" {var_name}.$.int) [] outcome)
        )
        "#
    );
    let mut vm_2 = create_avm(set_variable_call_service(json!({"int": scalar_int, })), vm_2_peer_id).await;
    let result = vm_2.call(&script, "", "", <_>::default()).await.unwrap();
    let expected_error = CatchableError::NonStringValueInTripletResolution {
        variable_name: var_name,
        actual_value: scalar_int.into(),
    };
    assert!(check_error(&result, expected_error));

    let var_name = "#canon".to_string();
    let script = format!(
        r#"
        (seq
            (seq
                (ap {scalar_int} $stream)
                (canon "{vm_2_peer_id}" $stream {var_name})
            )
            (call "{vm_2_peer_id}" ("" #canon.$.[0]) [] outcome)
        )
        "#
    );
    let mut vm_2 = create_avm(echo_call_service(), vm_2_peer_id).await;
    let result = vm_2.call(&script, "", "", <_>::default()).await.unwrap();
    let expected_error = CatchableError::NonStringValueInTripletResolution {
        variable_name: var_name.clone(),
        actual_value: scalar_int.into(),
    };
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn value_not_contain_such_array_idx_ap() {
    let vm_1_peer_id = "vm_1_peer_id";
    let idx = 42;
    let script = format!(
        r#"
        (seq
            (call "{vm_1_peer_id}" ("m" "f") [] result)
            (ap result.$.[{idx}].atrib outcome)
        )
        "#
    );
    let wrong_jvalue = json!(["a", "b"]);
    let mut vm_2 = create_avm(set_variable_call_service(wrong_jvalue.clone()), vm_1_peer_id).await;
    let result = vm_2.call(&script, "", "", <_>::default()).await.unwrap();

    let expected_error = CatchableError::LambdaApplierError(LambdaError::ValueNotContainSuchArrayIdx {
        value: wrong_jvalue.into(),
        idx,
    });
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn field_accessor_applied_to_stream() {
    let vm_peer_id_1 = "vm_peer_id_1";

    let arg = json!({"a": 1,});
    let mut peer_vm_1 = create_avm(set_variable_call_service(arg), vm_peer_id_1).await;
    let field_name = "int".to_string();
    let script = format!(
        r#"
        (seq
            (canon "{vm_peer_id_1}" $stream #canon_stream)
            (call "{vm_peer_id_1}" ("m" "f") [#canon_stream.$.{field_name}] output)
        )
        "#
    );

    let result = peer_vm_1.call(script.clone(), "", "", <_>::default()).await.unwrap();
    let expected_error =
        air::CatchableError::LambdaApplierError(air::LambdaError::FieldAccessorAppliedToStream { field_name });
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn array_accessor_not_match_value() {
    let vm_peer_id_1 = "vm_peer_id_1";

    let arg = json!({"a": 1,});
    let mut peer_vm_1 = create_avm(set_variable_call_service(arg.clone()), vm_peer_id_1).await;
    let idx = 42;
    let script = format!(
        r#"
        (seq
            (call "{vm_peer_id_1}" ("m" "f") [] nonarray)
            (call "{vm_peer_id_1}" ("m" "f") [nonarray.$.[{idx}]] result)
        )
        "#
    );

    let result = peer_vm_1.call(script.clone(), "", "", <_>::default()).await.unwrap();
    let expected_error = air::CatchableError::LambdaApplierError(air::LambdaError::ArrayAccessorNotMatchValue {
        value: arg.into(),
        idx,
    });
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn value_not_contain_such_array_idx_call_arg_lambda() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let arg = json!([0, 1]);
    let mut peer_vm_1 = create_avm(set_variable_call_service(arg.clone()), vm_peer_id_1).await;
    let idx = 42;
    let script = format!(
        r#"
        (seq
            (call "{vm_peer_id_1}" ("m" "f") [] array)
            (call "{vm_peer_id_1}" ("m" "f") [array.$.[{idx}]] result)
        )
        "#
    );

    let result = peer_vm_1.call(script.clone(), "", "", <_>::default()).await.unwrap();
    let expected_error = air::CatchableError::LambdaApplierError(air::LambdaError::ValueNotContainSuchArrayIdx {
        value: arg.into(),
        idx,
    });
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn value_not_contain_such_field_call_arg_lambda() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let arg = json!({"a": 1,});
    let mut peer_vm_1 = create_avm(set_variable_call_service(arg.clone()), vm_peer_id_1).await;
    let field_name = "b".to_string();
    let script = format!(
        r#"
        (seq
            (call "{vm_peer_id_1}" ("m" "f") [] obj)
            (call "{vm_peer_id_1}" ("m" "f") [obj.$.{field_name}] output)
        )
        "#
    );

    let result = peer_vm_1.call(script.clone(), "", "", <_>::default()).await.unwrap();
    let expected_error = air::CatchableError::LambdaApplierError(air::LambdaError::ValueNotContainSuchField {
        value: arg.into(),
        field_name,
    });
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn field_accesssor_not_match_value_call_arg_lambda() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let arg = json!([0, 1]);
    let mut peer_vm_1 = create_avm(set_variable_call_service(arg.clone()), vm_peer_id_1).await;

    let script = format!(
        r#"
        (seq
            (call "{vm_peer_id_1}" ("m" "f") [] obj)
            (call "{vm_peer_id_1}" ("m" "f") [obj.$.b] output)
        )
        "#
    );
    let result = peer_vm_1.call(script.clone(), "", "", <_>::default()).await.unwrap();
    let expected_error = air::CatchableError::LambdaApplierError(air::LambdaError::FieldAccessorNotMatchValue {
        value: arg.into(),
        field_name: "b".to_string(),
    });
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn index_access_not_u32_i64() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let vm_peer_id_2 = "vm_peer_id_2";
    let number = serde_json::Number::from(8589934592 as i64);
    let arg = json!(number);
    let mut peer_vm_1 = create_avm(set_variable_call_service(arg.clone()), vm_peer_id_1).await;
    let arg = json!([0, 1]);
    let mut peer_vm_2 = create_avm(set_variable_call_service(arg.clone()), vm_peer_id_2).await;

    let script = format!(
        r#"
        (seq
            (call "{vm_peer_id_1}" ("m" "f") [] idx)
            (seq
                (call "{vm_peer_id_2}" ("m" "f") [] array)
                (call "{vm_peer_id_2}" ("m" "f") [array.$.[idx]] result)
            )
        )
        "#
    );

    let result = peer_vm_1.call(script.clone(), "", "", <_>::default()).await.unwrap();
    let result = peer_vm_2
        .call(script.clone(), "", result.data, <_>::default())
        .await
        .unwrap();
    let expected_error =
        air::CatchableError::LambdaApplierError(air::LambdaError::IndexAccessNotU32 { accessor: number });
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn scalar_accessor_has_invalid_type_ap() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let obj_arg = json!({"a": 1,});
    let mut peer_vm_1 = create_avm(set_variable_call_service(obj_arg.clone()), vm_peer_id_1).await;

    let script = format!(
        r#"
        (seq
            (call "{vm_peer_id_1}" ("m" "f") [] array_obj)
            (seq
                (call "{vm_peer_id_1}" ("m" "f") [] array)
                (ap array.$.[array_obj] outcome)
            )
        )
        "#
    );

    let result = peer_vm_1.call(script.clone(), "", "", <_>::default()).await.unwrap();
    let expected_error = air::CatchableError::LambdaApplierError(air::LambdaError::ScalarAccessorHasInvalidType {
        scalar_accessor: obj_arg.into(),
    });
    assert!(check_error(&result, expected_error));

    let obj_arg = json!([{"a": 1,}]);
    let mut peer_vm_1 = create_avm(set_variable_call_service(obj_arg.clone()), vm_peer_id_1).await;
    let result = peer_vm_1.call(script.clone(), "", "", <_>::default()).await.unwrap();
    let expected_error = air::CatchableError::LambdaApplierError(air::LambdaError::ScalarAccessorHasInvalidType {
        scalar_accessor: obj_arg.into(),
    });
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn stream_accessor_has_invalid_type() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let obj_arg = json!({"a": 1,});
    let mut peer_vm_1 = create_avm(set_variable_call_service(obj_arg.clone()), vm_peer_id_1).await;

    let script = format!(
        r#"
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
    "#
    );

    let result = peer_vm_1.call(script.clone(), "", "", <_>::default()).await.unwrap();
    let expected_error = air::CatchableError::LambdaApplierError(air::LambdaError::StreamAccessorHasInvalidType {
        scalar_accessor: obj_arg.into(),
    });
    assert!(check_error(&result, expected_error));

    let obj_arg = json!([{"a": 1,}]);
    let mut peer_vm_1 = create_avm(set_variable_call_service(obj_arg.clone()), vm_peer_id_1).await;
    let result = peer_vm_1.call(script.clone(), "", "", <_>::default()).await.unwrap();
    let expected_error = air::CatchableError::LambdaApplierError(air::LambdaError::StreamAccessorHasInvalidType {
        scalar_accessor: obj_arg.into(),
    });
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn canon_stream_not_have_enough_values_call_arg() {
    let local_peer_id = "local_peer_id";
    let mut local_vm = create_avm(echo_call_service(), local_peer_id).await;

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
        (seq
            (canon "{local_peer_id}" $ns #ns)
            (call "{local_peer_id}" ("" "") [#ns.$.[0] #ns.$.[1] #ns])
        )
     )"#
    );

    let result = local_vm
        .call(&join_stream_script, "", "", <_>::default())
        .await
        .unwrap();
    let actual_trace = trace_from_result(&result);
    assert_eq!(actual_trace.len(), 2); // only the first call and canon should produce a trace
    let expected_error =
        CatchableError::LambdaApplierError(LambdaError::CanonStreamNotHaveEnoughValues { stream_size: 0, idx: 0 });
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn unsupported_map_keytype() {
    let local_peer_id = "local_peer_id";
    let obj_arg = json!({"a": {"b": 1},});
    let mut local_vm = create_avm(set_variable_call_service(obj_arg), local_peer_id).await;

    let map_name = "%map";
    let join_stream_script = format!(
        r#"
    (seq
        (call "{local_peer_id}" ("" "") [] scalar)
        (ap (scalar.$.a "serv1") %map)
    )
     "#
    );

    let result = local_vm
        .call(&join_stream_script, "", "", <_>::default())
        .await
        .unwrap();
    let expected_error = CatchableError::StreamMapError(unsupported_map_key_type(map_name));
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn undefined_last_error_functor() {
    let local_peer_id = "local_peer_id";
    let script = format!(
        r#"
    (xor
        (match 1 2
            (null)
        )
        (call "local_peer_id" ("test" "error_code") [%last_error%.length] scalar)
    )
    "#
    );

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(local_peer_id), &script)
        .await
        .expect("invalid test AIR script");
    let result = executor.execute_all(local_peer_id).await.unwrap();

    let expected_error = CatchableError::LengthFunctorAppliedToNotArray(no_error_object());
    assert!(check_error(&result.last().unwrap(), expected_error));
}

#[tokio::test]
async fn undefined_error_functor() {
    let local_peer_id = "local_peer_id";
    let script = format!(
        r#"
    (xor
        (match 1 2
            (null)
        )
        (call "local_peer_id" ("test" "error_code") [:error:.length] scalar)
    )
    "#
    );

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(local_peer_id), &script)
        .await
        .expect("invalid test AIR script");
    let result = executor.execute_all(local_peer_id).await.unwrap();

    let error_object = json!({"error_code":10001, "instruction":"match 1 2","message":"compared values do not match"});
    let expected_error = CatchableError::LengthFunctorAppliedToNotArray(error_object.into());
    assert!(check_error(&result.last().unwrap(), expected_error));
}

#[tokio::test]
async fn undefined_error_peerid() {
    let local_peer_id = "local_peer_id";
    let script = format!(
        r#"
    (xor
        (match 1 2
            (null)
        )
        (call "local_peer_id" ("test" "error_code") [:error:.$.peerid] scalar)
    )
    "#
    );

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(local_peer_id), &script)
        .await
        .expect("invalid test AIR script");
    let result = executor.execute_all(local_peer_id).await.unwrap();

    let value = json!({
        "error_code":10001,
        "instruction":"match 1 2",
        "message":"compared values do not match",
    })
    .into();
    let field_name = "peerid".into();
    let expected_error =
        air::CatchableError::LambdaApplierError(air::LambdaError::ValueNotContainSuchField { value, field_name });
    assert!(check_error(&result.last().unwrap(), expected_error));
}

#[tokio::test]
async fn undefined_last_error_instruction() {
    let local_peer_id = "local_peer_id";
    let script = format!(
        r#"
    (xor
        (match 1 2
            (null)
        )
        (call "local_peer_id" ("test" "instruction") [%last_error%.$.instruction] scalar)
    )
    "#
    );

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(local_peer_id), &script)
        .await
        .expect("invalid test AIR script");
    let result = executor.execute_all(local_peer_id).await.unwrap();

    let expected_error = CatchableError::LambdaApplierError(LambdaError::ValueNotContainSuchField {
        value: no_error_object(),
        field_name: "instruction".to_string(),
    });
    assert!(check_error(&&result.last().unwrap(), expected_error));
}

#[tokio::test]
async fn undefined_last_error_peer_id() {
    let local_peer_id = "local_peer_id";
    let script = format!(
        r#"
    (xor
        (match 1 2
            (null)
        )
        (call "local_peer_id" ("test" "peer_id") [%last_error%.$.peer_id] scalar)
    )
    "#
    );
    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(local_peer_id), &script)
        .await
        .expect("invalid test AIR script");
    let result = executor.execute_all(local_peer_id).await.unwrap();

    let expected_error = CatchableError::LambdaApplierError(LambdaError::ValueNotContainSuchField {
        value: no_error_object(),
        field_name: "peer_id".to_string(),
    });
    assert!(check_error(&&result.last().unwrap(), expected_error));
}
