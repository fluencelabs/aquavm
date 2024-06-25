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

use air::CatchableError;
use air::LambdaError;
use air_test_utils::prelude::*;

#[tokio::test]
async fn dont_wait_on_lens() {
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
    let mut set_variable_vm = create_avm(set_variables_call_service, set_variable_peer_id).await;

    let local_peer_id = "local_peer_id";
    let mut local_vm = create_avm(unit_call_service(), local_peer_id).await;

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

#[tokio::test]
async fn dont_wait_on_lsns_on_scalars() {
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
    let mut set_variable_vm = create_avm(set_variables_call_service, set_variable_peer_id).await;

    let array_consumer_peer_id = "array_consumer_peer_id";
    let mut array_consumer = create_avm(unit_call_service(), array_consumer_peer_id).await;

    let object_consumer_peer_id = "object_consumer_peer_id";
    let mut object_consumer = create_avm(unit_call_service(), object_consumer_peer_id).await;

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

#[tokio::test]
async fn ap_scalar_with_join_behaviour() {
    let peer_1_id = "peer_1_id";
    let peer_2_id = "peer_2_id";

    let mut peer_1 = create_avm(unit_call_service(), peer_1_id).await;

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

#[tokio::test]
async fn ap_stream_with_join_behaviour() {
    let peer_1_id = "peer_1_id";
    let peer_2_id = "peer_2_id";

    let mut peer_1 = create_avm(unit_call_service(), peer_1_id).await;

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

#[tokio::test]
async fn match_with_join_behaviour() {
    let peer_1_id = "peer_1_id";
    let peer_2_id = "peer_2_id";

    let mut peer_1 = create_avm(unit_call_service(), peer_1_id).await;

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

#[tokio::test]
async fn mismatch_with_join_behaviour() {
    let peer_1_id = "peer_1_id";
    let peer_2_id = "peer_2_id";

    let mut peer_1 = create_avm(unit_call_service(), peer_1_id).await;

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

#[tokio::test]
async fn fold_with_join_behaviour() {
    let peer_1_id = "peer_1_id";
    let peer_2_id = "peer_2_id";

    let mut peer_1 = create_avm(unit_call_service(), peer_1_id).await;

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

#[tokio::test]
async fn canon_with_empty_behaviour() {
    let peer_id = "peer_id";

    let mut peer_2 = create_avm(unit_call_service(), peer_id).await;

    let script = format!(
        r#"
        (canon "{peer_id}" $stream #canon_stream)
    "#
    );

    let result = checked_call_vm!(peer_2, <_>::default(), script, "", "");
    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![executed_state::canon(
        json!({"tetraplet": {"function_name": "", "lens": "", "peer_pk": "peer_id", "service_id": ""}, "values": []}),
    )];

    assert_eq!(actual_trace, expected_trace);
}
