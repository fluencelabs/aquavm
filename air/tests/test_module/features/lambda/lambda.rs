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

#[tokio::test]
async fn lambda_not_allowed_for_non_objects_and_arrays() {
    let set_variable_peer_id = "set_variable";
    let mut set_variable_vm = create_avm(echo_call_service(), set_variable_peer_id).await;

    let local_peer_id = "local_peer_id";

    let some_string = "some_string";
    let script = format!(
        r#"
        (seq
            (call "{set_variable_peer_id}" ("" "") ["{some_string}"] string_variable)
            (call "{local_peer_id}" ("" "") [string_variable.$.some_lambda])
        )
        "#
    );

    let result = call_vm!(set_variable_vm, <_>::default(), &script, "", "");

    let expected_error = CatchableError::LambdaApplierError(LambdaError::FieldAccessorNotMatchValue {
        value: json!(some_string),
        field_name: "some_lambda".to_string(),
    });
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn lambda_with_string_scalar() {
    let set_variable_peer_id = "set_variable";
    let variables = maplit::hashmap! {
        "string_accessor".to_string() => json!("some_field_name"),
        "value".to_string() => json!({"other_name_1": 0, "some_field_name": 1, "other_name_2": 0})
    };
    let mut set_variable_vm = create_avm(
        set_variables_call_service(variables, VariableOptionSource::FunctionName),
        set_variable_peer_id,
    )
    .await;

    let local_peer_id = "local_peer_id";
    let mut local_vm = create_avm(echo_call_service(), local_peer_id).await;

    let script = format!(
        r#"
        (seq
            (seq
                (call "{set_variable_peer_id}" ("" "string_accessor") [] string_accessor)
                (call "{set_variable_peer_id}" ("" "value") [] value)
            )
            (call "{local_peer_id}" ("" "") [value.$.[string_accessor]])
        )
        "#
    );

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(local_vm, <_>::default(), script, "", result.data);
    let trace = trace_from_result(&result);

    assert_eq!(&trace[2.into()], &unused!(1u32, peer = local_peer_id, args = vec![1]));
}

#[tokio::test]
async fn lambda_with_number_scalar() {
    let set_variable_peer_id = "set_variable";
    let variables = maplit::hashmap! {
        "number_accessor".to_string() => json!(1u32),
        "value".to_string() => json!([0, 1, 2])
    };
    let mut set_variable_vm = create_avm(
        set_variables_call_service(variables, VariableOptionSource::FunctionName),
        set_variable_peer_id,
    )
    .await;

    let local_peer_id = "local_peer_id";
    let mut local_vm = create_avm(echo_call_service(), local_peer_id).await;

    let script = format!(
        r#"
        (seq
            (seq
                (call "{set_variable_peer_id}" ("" "number_accessor") [] number_accessor)
                (call "{set_variable_peer_id}" ("" "value") [] value)
            )
            (call "{local_peer_id}" ("" "") [value.$.[number_accessor]])
        )
        "#
    );

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(local_vm, <_>::default(), script, "", result.data);
    let trace = trace_from_result(&result);

    assert_eq!(&trace[2.into()], &unused!(1u32, peer = local_peer_id, args = vec![1]));
}

#[tokio::test]
async fn lambda_with_number_stream() {
    let set_variable_peer_id = "set_variable";
    let variables = maplit::hashmap! {
        "number_accessor".to_string() => json!(1),
        "iterable".to_string() => json!([1,2,3]),
    };
    let mut set_variable_vm = create_avm(
        set_variables_call_service(variables, VariableOptionSource::FunctionName),
        set_variable_peer_id,
    )
    .await;

    let local_peer_id = "local_peer_id";
    let mut local_vm = create_avm(echo_call_service(), local_peer_id).await;

    let script = format!(
        r#"
        (seq
            (seq
                (call "{set_variable_peer_id}" ("" "number_accessor") [] number_accessor)
                (seq
                    (call "{set_variable_peer_id}" ("" "iterable") [] iterable)
                    (fold iterable iterator
                        (seq
                            (call "{local_peer_id}" ("" "") [iterator] $stream)
                            (next iterator)
                        )
                    )
                )
            )
            (seq
                (canon "{local_peer_id}" $stream #canon_stream)
                (call "{local_peer_id}" ("" "") [#canon_stream.$.[number_accessor]])
            )
        )
        "#
    );

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(local_vm, <_>::default(), script, "", result.data);
    let actual_trace = trace_from_result(&result);

    assert_eq!(
        &actual_trace[6.into()],
        &unused!(2, peer = local_peer_id, args = vec![2])
    );
}

#[tokio::test]
async fn lambda_with_number_stream_and_followed_scalar() {
    let set_variable_peer_id = "set_variable";
    let checkable_value = 1337;
    let variables = maplit::hashmap! {
        "number_accessor".to_string() => json!(1),
        "iterable".to_string() => json!([1,2,3]),
        "value".to_string() => json!({"field_1": checkable_value, "field_2": 31337}),
    };
    let mut set_variable_vm = create_avm(
        set_variables_call_service(variables, VariableOptionSource::FunctionName),
        set_variable_peer_id,
    )
    .await;

    let local_peer_id = "local_peer_id";
    let mut local_vm = create_avm(echo_call_service(), local_peer_id).await;

    let script = format!(
        r#"
        (seq
            (seq
                (seq
                    (call "{set_variable_peer_id}" ("" "number_accessor") [] number_accessor)
                    (call "{set_variable_peer_id}" ("" "value") [] value)
                )
                (seq
                    (call "{set_variable_peer_id}" ("" "iterable") [] iterable)
                    (fold iterable iterator
                        (seq
                            (call "{local_peer_id}" ("" "") [value] $stream) ;; place 3 complex values in a stream
                            (next iterator)
                        )
                    )
                )
            )
            (seq
                (canon "{local_peer_id}" $stream #canon_stream)
                (call "{local_peer_id}" ("" "") [#canon_stream.$.[number_accessor].field_1]) ;; get the 2nd value and then access its field
            )
        )
        "#
    );

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(local_vm, <_>::default(), script, "", result.data);
    let actual_trace = trace_from_result(&result);

    assert_eq!(
        &actual_trace[7.into()],
        &unused!(checkable_value, peer = local_peer_id, args = vec![checkable_value])
    );
}

#[tokio::test]
async fn lambda_with_scalar_join() {
    let set_variable_peer_id = "set_variable";
    let variables = maplit::hashmap! {
        "string_accessor".to_string() => json!("some_field_name"),
        "value".to_string() => json!({"other_name_1": 0, "some_field_name": 1, "other_name_2": 0})
    };
    let mut set_variable_vm = create_avm(
        set_variables_call_service(variables, VariableOptionSource::FunctionName),
        set_variable_peer_id,
    )
    .await;

    let local_peer_id = "local_peer_id";
    let mut local_vm = create_avm(echo_call_service(), local_peer_id).await;

    let script = format!(
        r#"
        (seq
            (par
                (call "non_exist_peer_id" ("" "string_accessor") [] string_accessor)
                (call "{set_variable_peer_id}" ("" "value") [] value)
            )
            (call "{local_peer_id}" ("" "") [value.$.[string_accessor]])
        )
        "#
    );

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(local_vm, <_>::default(), script, "", result.data);
    let trace = trace_from_result(&result);

    assert_eq!(&trace[3.into()], &executed_state::request_sent_by("set_variable"));
}

#[ignore]
// after 0.32 version AIR is no longer supports lambdas over stream,
// although this test could be useful in the future for functors
#[tokio::test]
async fn lambda_with_canon_stream_join() {
    let set_variable_peer_id = "set_variable";
    let variables = maplit::hashmap! {
        "number_accessor".to_string() => json!(1),
        "iterable".to_string() => json!([1,2,3]),
    };
    let mut set_variable_vm = create_avm(
        set_variables_call_service(variables, VariableOptionSource::FunctionName),
        set_variable_peer_id,
    )
    .await;

    let local_peer_id = "local_peer_id";
    let mut local_vm = create_avm(echo_call_service(), local_peer_id).await;

    let script = format!(
        r#"
        (seq
            (par
                (call "non_exist_peer_id" ("" "number_accessor") [] number_accessor)
                (seq
                    (call "{set_variable_peer_id}" ("" "iterable") [] iterable)
                    (fold iterable iterator
                        (seq
                            (ap "value" $stream)
                            (next iterator)
                        )
                    )
                )
            )
            (seq
                (canon "{local_peer_id}" $stream #stream)
                (call "{local_peer_id}" ("" "") [#stream.$.[number_accessor]])
            )
        )
        "#
    );

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(local_vm, <_>::default(), script, "", result.data);
    let actual_trace = trace_from_result(&result);

    assert_eq!(
        &actual_trace[7.into()],
        &executed_state::request_sent_by("set_variable"),
    );
}
