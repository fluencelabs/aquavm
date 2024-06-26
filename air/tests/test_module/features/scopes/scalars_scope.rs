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
use air::ExecutionError;
use air_test_utils::prelude::*;

use futures::FutureExt;

#[tokio::test]
async fn scalars_scope() {
    let peer_1_id = "peer_1_id";
    let array_1_content = json!(["1", "2"]);
    let mut peer_1_vm = create_avm(set_variable_call_service(array_1_content.clone()), peer_1_id).await;

    let some_peer_id = "some_peer_id";
    let mut some_peer_vm = create_avm(unit_call_service(), some_peer_id).await;

    let set_array_0_peer_id = "set_array_0_peer_id";
    let peer_2_id = "peer_2_id";
    let peers = json!([peer_1_id, peer_2_id]);
    let mut set_array_0_vm = create_avm(set_variable_call_service(peers.clone()), set_array_0_peer_id).await;

    let script = format!(
        r#"
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
        )"#
    );

    let result = checked_call_vm!(set_array_0_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(peer_1_vm, <_>::default(), &script, "", result.data);
    let result = checked_call_vm!(some_peer_vm, <_>::default(), &script, "", result.data);
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        scalar!(peers, peer = set_array_0_peer_id),
        executed_state::par(1, 0),
        scalar!(array_1_content, peer = peer_1_id),
        unused!("result from unit_call_service", peer = some_peer_id),
        unused!("result from unit_call_service", peer = some_peer_id),
        executed_state::par(1, 0),
        executed_state::request_sent_by(some_peer_id),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[tokio::test]
async fn before_after_of_next() {
    let set_array_0_peer_id = "set_array_0_peer_id";
    let array_0_content = json!([1, 2, 3]);
    let mut set_array_0_vm = create_avm(set_variable_call_service(array_0_content.clone()), set_array_0_peer_id).await;

    let vm_peer_0_id = "vm_peer_0_id";
    let counter = std::cell::Cell::new(0);
    let vm_peer_0_call_service: CallServiceClosure = Box::new(move |_params| {
        {
            let uncelled_request_id = counter.get();
            counter.set(uncelled_request_id + 1);
            async move { CallServiceResult::ok(json!(uncelled_request_id)) }
        }
        .boxed_local()
    });
    let mut peer_0_vm = create_avm(vm_peer_0_call_service, vm_peer_0_id).await;

    let vm_peer_1_id = "vm_peer_1_id";
    let mut peer_1_vm = create_avm(echo_call_service(), vm_peer_1_id).await;

    let script = format!(
        r#"
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
        )"#
    );

    let result = checked_call_vm!(set_array_0_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(peer_0_vm, <_>::default(), &script, "", result.data);
    let result = checked_call_vm!(peer_1_vm, <_>::default(), &script, "", result.data);
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        scalar!(array_0_content, peer = set_array_0_peer_id),
        scalar!(0, peer = vm_peer_0_id),
        scalar!(1, peer = vm_peer_0_id),
        scalar!(2, peer = vm_peer_0_id),
        unused!(2, peer = vm_peer_1_id, args = vec![2]),
        unused!(1, peer = vm_peer_1_id, args = vec![1]),
        unused!(0, peer = vm_peer_1_id, args = vec![0]),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[tokio::test]
async fn local_and_global_scalars() {
    let set_variable_peer_id = "set_variable_peer_id";
    let iterable_content = json!([1i64, 2]);
    let mut set_variable_vm = create_avm(
        set_variable_call_service(iterable_content.clone()),
        set_variable_peer_id,
    )
    .await;

    let local_setter_peer_id = "local_setter_peer_id";
    let counter = std::cell::Cell::new(0);
    let local_setter_call_service: CallServiceClosure = Box::new(move |_params| {
        let uncelled_request_id = counter.get();
        counter.set(uncelled_request_id + 1);
        async move { CallServiceResult::ok(json!(uncelled_request_id)) }.boxed_local()
    });
    let mut local_setter_vm = create_avm(local_setter_call_service, local_setter_peer_id).await;

    let local_consumer_peer_id = "local_consumer_peer_id";
    let mut local_consumer_vm = create_avm(echo_call_service(), local_consumer_peer_id).await;

    let script = format!(
        r#"
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
        )"#
    );

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(local_setter_vm, <_>::default(), &script, "", result.data);
    let result = checked_call_vm!(local_consumer_vm, <_>::default(), &script, "", result.data);
    let result = checked_call_vm!(local_setter_vm, <_>::default(), &script, "", result.data);
    let result = checked_call_vm!(local_consumer_vm, <_>::default(), &script, "", result.data);
    let result = checked_call_vm!(local_setter_vm, <_>::default(), &script, "", result.data);
    let result = checked_call_vm!(local_consumer_vm, <_>::default(), &script, "", result.data);
    let result = checked_call_vm!(local_setter_vm, <_>::default(), &script, "", result.data);
    let result = checked_call_vm!(local_consumer_vm, <_>::default(), &script, "", result.data);
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        scalar!(iterable_content.clone(), peer = set_variable_peer_id),
        scalar!(iterable_content, peer = set_variable_peer_id),
        scalar!(0, peer = local_setter_peer_id),
        scalar!(1, peer = local_setter_peer_id),
        scalar!(2, peer = local_setter_peer_id),
        unused!(2, peer = local_consumer_peer_id, args = vec![2]),
        scalar!(3, peer = local_setter_peer_id),
        unused!(3, peer = local_consumer_peer_id, args = vec![3]),
        unused!(1, peer = local_consumer_peer_id, args = vec![1]),
        scalar!(4, peer = local_setter_peer_id),
        scalar!(5, peer = local_setter_peer_id),
        unused!(5, peer = local_consumer_peer_id, args = vec![5]),
        scalar!(6, peer = local_setter_peer_id),
        unused!(6, peer = local_consumer_peer_id, args = vec![6]),
        unused!(4, peer = local_consumer_peer_id, args = vec![4]),
        unused!(0, peer = local_consumer_peer_id, args = vec![0]),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[tokio::test]
async fn new_with_randomly_set_scalars_in_fold_1() {
    let test_peer_id_1 = "test_peer_id_1";
    let mut test_vm_1 = create_avm(set_variable_call_service(json!([1, 2, 3])), test_peer_id_1).await;

    let test_peer_id_2 = "test_peer_id_2";
    let script = format!(
        r#"
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
    )"#
    );

    let result = call_vm!(test_vm_1, <_>::default(), &script, "", "");
    assert_eq!(result.ret_code, 0)
}

#[tokio::test]
async fn new_with_randomly_set_scalars_in_fold_2() {
    let test_peer_id_1 = "test_peer_id_1";
    let mut test_vm_1 = create_avm(set_variable_call_service(json!([1, 2, 3])), test_peer_id_1).await;

    let test_peer_id_2 = "test_peer_id_2";
    let variable_name = "scalar";
    let script = format!(
        r#"
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
    )"#
    );

    let result = call_vm!(test_vm_1, <_>::default(), &script, "", "");
    let expected_error = ExecutionError::Catchable(rc!(CatchableError::VariableWasNotInitializedAfterNew(
        variable_name.to_string()
    )));
    assert!(check_error(&result, expected_error));
}
