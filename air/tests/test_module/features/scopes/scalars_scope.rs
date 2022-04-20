/*
 * Copyright 2022 Fluence Labs Limited
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
use air::ExecutionError;
use air_test_utils::prelude::*;

use fstrings::f;
use fstrings::format_args_f;

#[test]
fn scalars_scope() {
    let peer_1_id = "peer_1_id";
    let array_1_content = json!(["1", "2"]);
    let mut peer_1_vm = create_avm(set_variable_call_service(array_1_content.clone()), peer_1_id);

    let some_peer_id = "some_peer_id";
    let mut some_peer_vm = create_avm(unit_call_service(), some_peer_id);

    let set_array_0_peer_id = "set_array_0_peer_id";
    let peer_2_id = "peer_2_id";
    let peers = json!([peer_1_id, peer_2_id]);
    let mut set_array_0_vm = create_avm(set_variable_call_service(peers.clone()), set_array_0_peer_id);

    let script = f!(r#"
        (seq
            (call "{set_array_0_peer_id}" ("" "") [] array-0)
            (fold array-0 array-0-iterator
                (seq
                    (par
                        (call array-0-iterator ("" "") [] array-1)
                        (null)
                    )
                    (seq
                        (fold array-1 array-1-iterator
                            (seq
                                (call "{some_peer_id}" ("" "") [])
                                (next array-1-iterator)
                            )
                        )
                        (next array-0-iterator)
                    )
                )
            )
        )"#);

    let init_peer_id = "";
    let result = checked_call_vm!(set_array_0_vm, init_peer_id, &script, "", "");
    let result = checked_call_vm!(peer_1_vm, init_peer_id, &script, "", result.data);
    let result = checked_call_vm!(some_peer_vm, init_peer_id, &script, "", result.data);
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        executed_state::scalar(peers),
        executed_state::par(1, 0),
        executed_state::scalar(array_1_content),
        executed_state::scalar_string("result from unit_call_service"),
        executed_state::scalar_string("result from unit_call_service"),
        executed_state::par(1, 0),
        executed_state::request_sent_by(some_peer_id),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn before_after_of_next() {
    let set_array_0_peer_id = "set_array_0_peer_id";
    let array_0_content = json!([1, 2, 3]);
    let mut set_array_0_vm = create_avm(set_variable_call_service(array_0_content.clone()), set_array_0_peer_id);

    let vm_peer_0_id = "vm_peer_0_id";
    let counter = std::cell::Cell::new(0);
    let vm_peer_0_call_service: CallServiceClosure = Box::new(move |_params| {
        let uncelled_request_id = counter.get();
        counter.set(uncelled_request_id + 1);
        CallServiceResult::ok(json!(uncelled_request_id))
    });
    let mut peer_0_vm = create_avm(vm_peer_0_call_service, vm_peer_0_id);

    let vm_peer_1_id = "vm_peer_1_id";
    let mut peer_1_vm = create_avm(echo_call_service(), vm_peer_1_id);

    let script = f!(r#"
        (seq
            (call "{set_array_0_peer_id}" ("" "") [] array-0)
            (fold array-0 array-0-iterator
                (seq
                    (call "{vm_peer_0_id}" ("" "") [] local)
                    (seq
                        (next array-0-iterator)
                        (call "{vm_peer_1_id}" ("" "") [local])
                    )
                )
            )
        )"#);

    let init_peer_id = "";
    let result = checked_call_vm!(set_array_0_vm, init_peer_id, &script, "", "");
    let result = checked_call_vm!(peer_0_vm, init_peer_id, &script, "", result.data);
    let result = checked_call_vm!(peer_1_vm, init_peer_id, &script, "", result.data);
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        executed_state::scalar(array_0_content),
        executed_state::scalar_number(0),
        executed_state::scalar_number(1),
        executed_state::scalar_number(2),
        executed_state::scalar_number(2),
        executed_state::scalar_number(1),
        executed_state::scalar_number(0),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn local_and_global_scalars() {
    let set_variable_peer_id = "set_variable_peer_id";
    let iterable_content = json!([1i64, 2]);
    let mut set_variable_vm = create_avm(
        set_variable_call_service(iterable_content.clone()),
        set_variable_peer_id,
    );

    let local_setter_peer_id = "local_setter_peer_id";
    let counter = std::cell::Cell::new(0);
    let local_setter_call_service: CallServiceClosure = Box::new(move |_params| {
        let uncelled_request_id = counter.get();
        counter.set(uncelled_request_id + 1);
        CallServiceResult::ok(json!(uncelled_request_id))
    });
    let mut local_setter_vm = create_avm(local_setter_call_service, local_setter_peer_id);

    let local_consumer_peer_id = "local_consumer_peer_id";
    let mut local_consumer_vm = create_avm(echo_call_service(), local_consumer_peer_id);

    let script = f!(r#"
        (seq
           (seq
               (seq
                (call "{set_variable_peer_id}" ("" "") [] iterable_1)
                (call "{set_variable_peer_id}" ("" "") [] iterable_2)
               )
               (seq
                 	(call "{local_setter_peer_id}" ("" "") [] local) ;; (1)
                 	(fold iterable_1 iterator_1
                   		(seq
                     		(seq
                       			(seq
				                    (call "{local_setter_peer_id}" ("" "") [] local) ;; (2)
				                    (fold iterable_2 iterator_2
				                        (seq
				                        	(seq
				                            	(call "{local_setter_peer_id}" ("" "") [] local) ;; (3)
				                            	(call "{local_consumer_peer_id}" ("" "") [local]) ;; local set by (3) will be used
				                            )
				                          	(next iterator_2)
				                        )
				                    )
				                )
                       			(call "{local_consumer_peer_id}" ("" "") [local]) ;; local set by (2) will be used
                     		)
                     		(next iterator_1)
                   		)
                    )
                )
            )
            (call "{local_consumer_peer_id}" ("" "") [local]) ;; local set by (1) will be used
        )"#);

    let init_peer_id = "";
    let result = checked_call_vm!(set_variable_vm, init_peer_id, &script, "", "");
    let result = checked_call_vm!(local_setter_vm, init_peer_id, &script, "", result.data);
    let result = checked_call_vm!(local_consumer_vm, init_peer_id, &script, "", result.data);
    let result = checked_call_vm!(local_setter_vm, init_peer_id, &script, "", result.data);
    let result = checked_call_vm!(local_consumer_vm, init_peer_id, &script, "", result.data);
    let result = checked_call_vm!(local_setter_vm, init_peer_id, &script, "", result.data);
    let result = checked_call_vm!(local_consumer_vm, init_peer_id, &script, "", result.data);
    let result = checked_call_vm!(local_setter_vm, init_peer_id, &script, "", result.data);
    let result = checked_call_vm!(local_consumer_vm, init_peer_id, &script, "", result.data);
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        executed_state::scalar(iterable_content.clone()),
        executed_state::scalar(iterable_content.clone()),
        executed_state::scalar_number(0),
        executed_state::scalar_number(1),
        executed_state::scalar_number(2),
        executed_state::scalar_number(2),
        executed_state::scalar_number(3),
        executed_state::scalar_number(3),
        executed_state::scalar_number(1),
        executed_state::scalar_number(4),
        executed_state::scalar_number(5),
        executed_state::scalar_number(5),
        executed_state::scalar_number(6),
        executed_state::scalar_number(6),
        executed_state::scalar_number(4),
        executed_state::scalar_number(0),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn new_with_randomly_set_scalars_in_fold_1() {
    let test_peer_id_1 = "test_peer_id_1";
    let mut test_vm_1 = create_avm(set_variable_call_service(json!([1, 2, 3])), test_peer_id_1);

    let test_peer_id_2 = "test_peer_id_2";
    let script = f!(r#"
    (seq
        (call "{test_peer_id_1}" ("" "") [] iterable)
        (fold iterable iterator
            (seq
                (seq
                    (call "{test_peer_id_1}" ("" "") [] scalar)
                    (new scalar
                        (seq
                            (seq
                                (par
                                    (call "{test_peer_id_2}" ("" "") [] scalar)
                                    (null)
                                )
                                (next iterator)
                            )
                            (par
                                (call "{test_peer_id_2}" ("" "") [scalar])
                                (null)
                            )
                        )
                    )
                )
                (call "{test_peer_id_1}" ("" "") [scalar])
            )
        )
    )"#);

    let result = call_vm!(test_vm_1, "", &script, "", "");
    assert_eq!(result.ret_code, 0)
}

#[test]
fn new_with_randomly_set_scalars_in_fold_2() {
    let test_peer_id_1 = "test_peer_id_1";
    let mut test_vm_1 = create_avm(set_variable_call_service(json!([1, 2, 3])), test_peer_id_1);

    let test_peer_id_2 = "test_peer_id_2";
    let variable_name = "scalar";
    let script = f!(r#"
    (seq
        (call "{test_peer_id_1}" ("" "") [] iterable)
        (fold iterable iterator
            (seq
                (seq
                    (call "{test_peer_id_1}" ("" "") [] {variable_name})
                    (new {variable_name}
                        (seq
                            (seq
                                (par
                                    (call "{test_peer_id_2}" ("" "") [] {variable_name})
                                    (null)
                                )
                                (next iterator)
                            )
                            (call "{test_peer_id_1}" ("" "") [{variable_name}])
                        )
                    )
                )
                (call "{test_peer_id_1}" ("" "") [{variable_name}])
            )
        )
    )"#);

    let result = call_vm!(test_vm_1, "", &script, "", "");
    let expected_error = ExecutionError::Catchable(rc!(CatchableError::VariableWasNotInitializedAfterNew(
        variable_name.to_string()
    )));
    assert!(check_error(&result, expected_error));
}
