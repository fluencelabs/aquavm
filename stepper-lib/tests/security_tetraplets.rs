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

use aqua_test_utils::call_vm;
use aqua_test_utils::create_aqua_vm;
use aqua_test_utils::HostExportedFunc;
use aqua_test_utils::IValue;
use aqua_test_utils::Vec1;
use plets::{ResolvedTriplet, SecurityTetraplet};

use std::cell::RefCell;
use std::rc::Rc;

type ArgTetraplets = Vec<Vec<SecurityTetraplet>>;

fn arg_host_function() -> (HostExportedFunc, Rc<RefCell<ArgTetraplets>>) {
    let arg_tetraplets = Rc::new(RefCell::new(ArgTetraplets::new()));

    let arg_tetraplets_inner = arg_tetraplets.clone();
    let host_function: HostExportedFunc = Box::new(move |_, args| -> Option<IValue> {
        let tetraplets = match &args[3] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };

        let de_tetraplets: ArgTetraplets =
            serde_json::from_str(tetraplets).expect("json deserialization shouldn't fail");
        *arg_tetraplets_inner.borrow_mut() = de_tetraplets;

        Some(IValue::Record(
            Vec1::new(vec![IValue::S32(0), IValue::String(tetraplets.clone())]).unwrap(),
        ))
    });

    (host_function, arg_tetraplets)
}

#[test]
fn simple_fold() {
    let return_numbers_call_service: HostExportedFunc = Box::new(|_, _| -> Option<IValue> {
        Some(IValue::Record(
            Vec1::new(vec![
                IValue::S32(0),
                IValue::String(String::from(
                    "[\"1\", \"2\", \"3\", \"4\", \"5\", \"6\", \"7\", \"8\", \"9\", \"10\"]",
                )),
            ])
            .unwrap(),
        ))
    });

    let set_variable_vm_peer_id = String::from("some_peer_id_1");
    let mut set_variable_vm = create_aqua_vm(return_numbers_call_service, set_variable_vm_peer_id.clone());

    let mut client_vms = Vec::new();
    for i in 1..=10 {
        let (arg_host_func, arg_tetraplets) = arg_host_function();
        let vm = create_aqua_vm(arg_host_func, i.to_string());
        client_vms.push((vm, arg_tetraplets))
    }

    let service_id = String::from("some_service_id");
    let function_name = String::from("some_function_name");
    let script = format!(
        r#"
        (seq
            (call "{}" ("{}" "{}") [] IterableResultPeer1)
            (fold IterableResultPeer1 i
                (par
                    (call i ("local_service_id" "local_fn_name") [i "some_text_literal"] acc[])
                    (next i)
                )
            )
        )
        "#,
        set_variable_vm_peer_id, service_id, function_name
    );

    let init_peer_id = String::from("some_init_peer_id");
    let res = call_vm!(set_variable_vm, init_peer_id.clone(), script.clone(), "", "");
    let mut data = res.data;

    let first_arg_triplet = ResolvedTriplet {
        peer_pk: set_variable_vm_peer_id,
        service_id,
        function_name,
    };
    let first_arg_triplet = Rc::new(first_arg_triplet);
    let first_arg_tetraplet = SecurityTetraplet {
        triplet: first_arg_triplet,
        json_path: String::new(),
    };

    let second_arg_triplet = ResolvedTriplet {
        peer_pk: init_peer_id.clone(),
        service_id: String::new(),
        function_name: String::new(),
    };
    let second_arg_triplet = Rc::new(second_arg_triplet);
    let second_arg_tetraplet = SecurityTetraplet {
        triplet: second_arg_triplet,
        json_path: String::new(),
    };

    let right_tetraplets = vec![vec![first_arg_tetraplet], vec![second_arg_tetraplet]];
    let right_tetraplets = Rc::new(RefCell::new(right_tetraplets));
    for i in 0..10 {
        let res = call_vm!(client_vms[i].0, init_peer_id.clone(), script.clone(), "[]", data);
        data = res.data;

        assert_eq!(client_vms[i].1, right_tetraplets);
    }
}

#[test]
fn fold_json_path() {
    let return_numbers_call_service: HostExportedFunc = Box::new(|_, _| -> Option<IValue> {
        Some(IValue::Record(
            Vec1::new(vec![
                IValue::S32(0),
                IValue::String(String::from(
                    "{\"arg\": [\"1\", \"2\", \"3\", \"4\", \"5\", \"6\", \"7\", \"8\", \"9\", \"10\"]}",
                )),
            ])
            .unwrap(),
        ))
    });

    let set_variable_vm_peer_id = String::from("some_peer_id_1");
    let mut set_variable_vm = create_aqua_vm(return_numbers_call_service, set_variable_vm_peer_id.clone());

    let (arg_host_func, arg_tetraplets) = arg_host_function();
    let client_peer_id = String::from("client_id");
    let mut client_vm = create_aqua_vm(arg_host_func, client_peer_id.clone());

    let service_id = String::from("some_service_id");
    let function_name = String::from("some_function_name");
    let script = format!(
        r#"
        (seq
            (call "{}" ("{}" "{}") [] IterableResultPeer1)
            (fold IterableResultPeer1.$.arg i
                (par
                    (call "{}" ("local_service_id" "local_fn_name") [i "some_text_literal"] acc[])
                    (next i)
                )
            )
        )
        "#,
        set_variable_vm_peer_id, service_id, function_name, client_peer_id
    );

    let init_peer_id = String::from("some_init_peer_id");
    let res = call_vm!(set_variable_vm, init_peer_id.clone(), script.clone(), "", "");

    let first_arg_triplet = ResolvedTriplet {
        peer_pk: set_variable_vm_peer_id,
        service_id,
        function_name,
    };
    let first_arg_triplet = Rc::new(first_arg_triplet);
    let first_arg_tetraplet = SecurityTetraplet {
        triplet: first_arg_triplet,
        json_path: String::from("$.arg"),
    };

    let second_arg_triplet = ResolvedTriplet {
        peer_pk: init_peer_id.clone(),
        service_id: String::new(),
        function_name: String::new(),
    };
    let second_arg_triplet = Rc::new(second_arg_triplet);
    let second_arg_tetraplet = SecurityTetraplet {
        triplet: second_arg_triplet,
        json_path: String::new(),
    };

    let right_tetraplets = vec![vec![first_arg_tetraplet], vec![second_arg_tetraplet]];
    let right_tetraplets = Rc::new(RefCell::new(right_tetraplets));
    call_vm!(client_vm, init_peer_id.clone(), script.clone(), "[]", res.data);
    assert_eq!(arg_tetraplets, right_tetraplets);
}
