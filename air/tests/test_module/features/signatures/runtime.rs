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

use air::UncatchableError;
use air_test_utils::key_utils::derive_dummy_keypair;
use air_test_utils::prelude::*;

/// This test module asserts various runtime safety checks, for example,
/// that actual calls' tetraplets are compared to stored one.

#[tokio::test]
async fn test_runtime_executed_call_argument_hash() {
    // Mallory gets a trace where there are two calls that differ only by argument_hash.
    // Can it swap them successfully?
    let alice_name = "alice";
    let bob_name = "bob";
    let mallory_name = "mallory";

    let (alice_keypair, alice_peer_id) = derive_dummy_keypair(alice_name);
    let (bob_keypair, bob_peer_id) = derive_dummy_keypair(bob_name);
    let (mallory_keypair, mallory_peer_id) = derive_dummy_keypair(mallory_name);

    let test_run_params = TestRunParameters::from_init_peer_id(&alice_peer_id);

    let air_script = format!(
        r#"
        (seq
          (seq
            (call "{alice_peer_id}" ("service" "func") [42] x)
            (call "{alice_peer_id}" ("service" "func") [43] y))
          (seq
            (call "{mallory_peer_id}" ("" "") [42] z)
            (call "{bob_peer_id}" ("service" "secret") [x y z] w)))
        "#
    );

    let mut alice_avm =
        create_avm_with_key::<NativeAirRunner>(alice_keypair, echo_call_service(), <_>::default()).await;
    let mut bob_avm = create_avm_with_key::<NativeAirRunner>(bob_keypair, echo_call_service(), <_>::default()).await;
    let mut mallory_avm =
        create_avm_with_key::<NativeAirRunner>(mallory_keypair, echo_call_service(), <_>::default()).await;

    let alice_res = alice_avm
        .call(&air_script, "", "", test_run_params.clone())
        .await
        .unwrap();
    let mallory_res = mallory_avm
        .call(&air_script, "", alice_res.data, test_run_params.clone())
        .await
        .unwrap();
    let mut mallory_env = env_from_result(&mallory_res);

    let mut mallory_data = InterpreterData::try_from_slice(&mallory_env.inner_data).unwrap();
    let mut mallory_raw_trace: Vec<_> = mallory_data.trace.to_vec();
    mallory_raw_trace.swap(0, 1);
    mallory_data.trace = ExecutionTrace::from(mallory_raw_trace);

    mallory_env.inner_data = mallory_data.serialize().unwrap().into();

    let mallory_data = mallory_env.serialize().unwrap();

    let bob_res = bob_avm
        .call(air_script, "", mallory_data, test_run_params)
        .await
        .unwrap();
    assert_error_eq!(
        &bob_res,
        UncatchableError::InstructionParametersMismatch {
            param: "call argument_hash",
            expected_value: "bagaaihraryhzxrhasfve7jwovrl4rb4j45lljt5prmoci34y3i6qx7joxy2a".to_owned(),
            stored_value: "bagaaihra7w4yil3eqnjimo4d3yp4kr2yra2o6svycab67oymtseafak4la6a".to_owned(),
        }
    );
}

#[tokio::test]
async fn test_runtime_executed_call_tetraplet() {
    // Mallory gets a trace where there are two calls that differ only by argument_hash.
    // Can it swap them successfully?
    let alice_name = "alice";
    let bob_name = "bob";
    let mallory_name = "mallory";

    let (alice_keypair, alice_peer_id) = derive_dummy_keypair(alice_name);
    let (bob_keypair, bob_peer_id) = derive_dummy_keypair(bob_name);
    let (mallory_keypair, mallory_peer_id) = derive_dummy_keypair(mallory_name);

    let test_run_params = TestRunParameters::from_init_peer_id(&alice_peer_id);

    let air_script = format!(
        r#"
        (seq
          (seq
            (call "{alice_peer_id}" ("service1" "func") [42] x)
            (call "{alice_peer_id}" ("service2" "func") [42] y))
          (seq
            (call "{mallory_peer_id}" ("" "") [42] z)
            (call "{bob_peer_id}" ("service" "secret") [x y z] w)))
        "#
    );

    let mut alice_avm =
        create_avm_with_key::<NativeAirRunner>(alice_keypair, echo_call_service(), <_>::default()).await;
    let mut bob_avm = create_avm_with_key::<NativeAirRunner>(bob_keypair, echo_call_service(), <_>::default()).await;
    let mut mallory_avm =
        create_avm_with_key::<NativeAirRunner>(mallory_keypair, echo_call_service(), <_>::default()).await;

    let alice_res = alice_avm
        .call(&air_script, "", "", test_run_params.clone())
        .await
        .unwrap();
    let mallory_res = mallory_avm
        .call(&air_script, "", alice_res.data, test_run_params.clone())
        .await
        .unwrap();
    let mut mallory_env = env_from_result(&mallory_res);

    let mut mallory_data = InterpreterData::try_from_slice(&mallory_env.inner_data).unwrap();
    let mut mallory_raw_trace: Vec<_> = mallory_data.trace.to_vec();
    mallory_raw_trace.swap(0, 1);
    mallory_data.trace = ExecutionTrace::from(mallory_raw_trace);

    mallory_env.inner_data = mallory_data.serialize().unwrap().into();

    let mallory_data = mallory_env.serialize().unwrap();

    let bob_res = bob_avm
        .call(air_script, "", mallory_data, test_run_params)
        .await
        .unwrap();
    let expected_value = format!(
        concat!(
            r#"SecurityTetraplet {{ peer_pk: "{alice_peer_id}","#,
            r#" service_id: "service1", function_name: "func", lens: "" }}"#
        ),
        alice_peer_id = alice_peer_id,
    );
    let stored_value = format!(
        concat!(
            r#"SecurityTetraplet {{ peer_pk: "{alice_peer_id}","#,
            r#" service_id: "service2", function_name: "func", lens: "" }}"#,
        ),
        alice_peer_id = alice_peer_id,
    );
    assert_error_eq!(
        &bob_res,
        UncatchableError::InstructionParametersMismatch {
            param: "call tetraplet",
            // please note that order is important here: if values are swapped, then the error is
            // handled by Executed branch, not Failed branch
            expected_value,
            stored_value,
        }
    );
}

#[tokio::test]
async fn test_runtime_executed_failed_argument_hash() {
    // Mallory gets a trace where there are two calls that differ only by argument_hash.
    // Can it swap them successfully?
    let alice_name = "alice";
    let bob_name = "bob";
    let mallory_name = "mallory";

    let (alice_keypair, alice_peer_id) = derive_dummy_keypair(alice_name);
    let (bob_keypair, bob_peer_id) = derive_dummy_keypair(bob_name);
    let (mallory_keypair, mallory_peer_id) = derive_dummy_keypair(mallory_name);

    let test_run_params = TestRunParameters::from_init_peer_id(&alice_peer_id);

    let air_script = format!(
        r#"
        (seq
          (seq
            (xor
              (call "{alice_peer_id}" ("service" "func") [42] x)
              (null))
            (call "{alice_peer_id}" ("service" "func") [43] y))
          (seq
            (call "{mallory_peer_id}" ("" "") [42] z)
            (call "{bob_peer_id}" ("service" "secret") [x y z] w)))
        "#
    );

    let mut alice_avm =
        create_avm_with_key::<NativeAirRunner>(alice_keypair, fallible_call_service_by_arg(43), <_>::default()).await;
    let mut bob_avm = create_avm_with_key::<NativeAirRunner>(bob_keypair, echo_call_service(), <_>::default()).await;
    let mut mallory_avm =
        create_avm_with_key::<NativeAirRunner>(mallory_keypair, echo_call_service(), <_>::default()).await;

    let alice_res = alice_avm
        .call(&air_script, "", "", test_run_params.clone())
        .await
        .unwrap();
    let mallory_res = mallory_avm
        .call(&air_script, "", alice_res.data, test_run_params.clone())
        .await
        .unwrap();
    let mut mallory_env = env_from_result(&mallory_res);

    let mut mallory_data = InterpreterData::try_from_slice(&mallory_env.inner_data).unwrap();
    let mut mallory_raw_trace: Vec<_> = mallory_data.trace.to_vec();
    mallory_raw_trace.swap(0, 1);
    mallory_data.trace = ExecutionTrace::from(mallory_raw_trace);

    mallory_env.inner_data = mallory_data.serialize().unwrap().into();

    let mallory_data = mallory_env.serialize().unwrap();

    let bob_res = bob_avm
        .call(air_script, "", mallory_data, test_run_params)
        .await
        .unwrap();
    assert_error_eq!(
        &bob_res,
        UncatchableError::InstructionParametersMismatch {
            param: "call argument_hash",
            // please note that order is important here: if values are swapped, then the error is
            // handled by Executed branch, not Failed branch
            expected_value: "bagaaihraryhzxrhasfve7jwovrl4rb4j45lljt5prmoci34y3i6qx7joxy2a".to_owned(),
            stored_value: "bagaaihra7w4yil3eqnjimo4d3yp4kr2yra2o6svycab67oymtseafak4la6a".to_owned(),
        }
    );
}

#[tokio::test]
async fn test_runtime_failed_call_tetraplet() {
    // Mallory gets a trace where there are two calls that differ only by argument_hash.
    // Can it swap them successfully?
    let alice_name = "alice";
    let bob_name = "bob";
    let mallory_name = "mallory";

    let (alice_keypair, alice_peer_id) = derive_dummy_keypair(alice_name);
    let (bob_keypair, bob_peer_id) = derive_dummy_keypair(bob_name);
    let (mallory_keypair, mallory_peer_id) = derive_dummy_keypair(mallory_name);

    let test_run_params = TestRunParameters::from_init_peer_id(&alice_peer_id);

    let air_script = format!(
        r#"
        (seq
          (seq
            (xor
              (call "{alice_peer_id}" ("service1" "func") [42] x)
              (null))
            (call "{alice_peer_id}" ("service2" "func") [42] y))
          (seq
            (call "{mallory_peer_id}" ("" "") [42] z)
            (call "{bob_peer_id}" ("service" "secret") [x y z] w)))
        "#
    );

    let mut alice_avm =
        create_avm_with_key::<NativeAirRunner>(alice_keypair, fallible_call_service("service1"), <_>::default()).await;
    let mut bob_avm = create_avm_with_key::<NativeAirRunner>(bob_keypair, echo_call_service(), <_>::default()).await;
    let mut mallory_avm =
        create_avm_with_key::<NativeAirRunner>(mallory_keypair, echo_call_service(), <_>::default()).await;

    let alice_res = alice_avm
        .call(&air_script, "", "", test_run_params.clone())
        .await
        .unwrap();
    let mallory_res = mallory_avm
        .call(&air_script, "", alice_res.data, test_run_params.clone())
        .await
        .unwrap();
    let mut mallory_env = env_from_result(&mallory_res);

    let mut mallory_data = InterpreterData::try_from_slice(&mallory_env.inner_data).unwrap();
    let mut mallory_raw_trace: Vec<_> = mallory_data.trace.to_vec();
    mallory_raw_trace.swap(0, 1);
    mallory_data.trace = ExecutionTrace::from(mallory_raw_trace);

    mallory_env.inner_data = mallory_data.serialize().unwrap().into();

    let mallory_data = mallory_env.serialize().unwrap();

    let bob_res = bob_avm
        .call(air_script, "", mallory_data, test_run_params)
        .await
        .unwrap();
    let expected_value = format!(
        concat!(
            r#"SecurityTetraplet {{ peer_pk: "{alice_peer_id}","#,
            r#" service_id: "service1", function_name: "func", lens: "" }}"#
        ),
        alice_peer_id = alice_peer_id,
    );
    let stored_value = format!(
        concat!(
            r#"SecurityTetraplet {{ peer_pk: "{alice_peer_id}","#,
            r#" service_id: "service2", function_name: "func", lens: "" }}"#,
        ),
        alice_peer_id = alice_peer_id,
    );
    assert_error_eq!(
        &bob_res,
        UncatchableError::InstructionParametersMismatch {
            param: "call tetraplet",
            // please note that order is important here: if values are swapped, then the error is
            // handled by Executed branch, not Failed branch
            expected_value,
            stored_value,
        }
    );
}

#[tokio::test]
async fn test_runtime_canon_tetraplet() {
    let alice_name = "alice";
    let bob_name = "bob";
    let mallory_name = "mallory";

    let (alice_keypair, alice_peer_id) = derive_dummy_keypair(alice_name);
    let (bob_keypair, bob_peer_id) = derive_dummy_keypair(bob_name);
    let (mallory_keypair, mallory_peer_id) = derive_dummy_keypair(mallory_name);

    let test_run_params = TestRunParameters::from_init_peer_id(&alice_peer_id);

    let air_script = format!(
        r#"
    (seq
       (seq
          (ap 42 $x)
          (ap 43 $x))
       (seq
          (seq
             (canon "{alice_peer_id}" $x #xa)
             (canon "{mallory_peer_id}" $x #xm))
          (call "{bob_peer_id}" ("" "") [#xa #xm] z)))
    "#
    );

    let mut alice_avm =
        create_avm_with_key::<NativeAirRunner>(alice_keypair, fallible_call_service("service1"), <_>::default()).await;
    let mut bob_avm = create_avm_with_key::<NativeAirRunner>(bob_keypair, echo_call_service(), <_>::default()).await;
    let mut mallory_avm =
        create_avm_with_key::<NativeAirRunner>(mallory_keypair, echo_call_service(), <_>::default()).await;

    let alice_res = alice_avm
        .call(&air_script, "", "", test_run_params.clone())
        .await
        .unwrap();
    let mallory_res = mallory_avm
        .call(&air_script, "", alice_res.data, test_run_params.clone())
        .await
        .unwrap();
    let mut mallory_env = env_from_result(&mallory_res);

    let mut mallory_data = InterpreterData::try_from_slice(&mallory_env.inner_data).unwrap();
    let mut mallory_raw_trace: Vec<_> = mallory_data.trace.to_vec();
    mallory_raw_trace.swap(2, 3);
    mallory_data.trace = ExecutionTrace::from(mallory_raw_trace);

    mallory_env.inner_data = mallory_data.serialize().unwrap().into();

    let mallory_data = mallory_env.serialize().unwrap();

    let bob_res = bob_avm
        .call(air_script, "", mallory_data, test_run_params)
        .await
        .unwrap();
    let expected_value = format!(
        concat!(
            r#"SecurityTetraplet {{ peer_pk: "{alice_peer_id}","#,
            r#" service_id: "", function_name: "", lens: "" }}"#
        ),
        alice_peer_id = alice_peer_id,
    );
    let stored_value = format!(
        concat!(
            r#"SecurityTetraplet {{ peer_pk: "{mallory_peer_id}","#,
            r#" service_id: "", function_name: "", lens: "" }}"#,
        ),
        mallory_peer_id = mallory_peer_id,
    );
    assert_error_eq!(
        &bob_res,
        UncatchableError::InstructionParametersMismatch {
            param: "canon tetraplet",
            expected_value,
            stored_value,
        }
    );
}
