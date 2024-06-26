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

use air_test_utils::prelude::*;

use futures::FutureExt;

use std::cell::RefCell;
use std::rc::Rc;

type ClosureSettableVar<T> = Rc<RefCell<T>>;

#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct ClosureCallArgs {
    service_id_var: Rc<RefCell<String>>,
    function_name_var: ClosureSettableVar<String>,
    args_var: ClosureSettableVar<Vec<i32>>,
    tetraplets: ClosureSettableVar<Vec<Vec<String>>>,
}

fn create_check_service_closure(closure_call_args: ClosureCallArgs) -> CallServiceClosure<'static> {
    Box::new(move |params| {
        use std::ops::Deref;

        *closure_call_args.service_id_var.deref().borrow_mut() = params.service_id.clone();
        *closure_call_args.function_name_var.deref().borrow_mut() = params.function_name.clone();

        let call_args: Vec<i32> = serde_json::from_value(serde_json::Value::Array(params.arguments))
            .expect("json deserialization shouldn't fail");
        *closure_call_args.args_var.deref().borrow_mut() = call_args;

        let result = CallServiceResult::ok(json!(""));
        async move { result }.boxed_local()
    })
}

#[tokio::test]
async fn flattening_scalar_arrays() {
    let scalar_array = json!({"iterable": [
        {"peer_id" : "local_peer_id", "service_id": "local_service_id", "function_name": "local_function_name", "args": [0, 1]},
        {"peer_id" : "local_peer_id", "service_id": "local_service_id", "function_name": "local_function_name", "args": [2, 3]},
    ]});

    let set_variable_peer_id = "set_variable";
    let mut set_variable_vm = create_avm(set_variable_call_service(scalar_array), set_variable_peer_id).await;

    let closure_call_args = ClosureCallArgs::default();
    let local_peer_id = "local_peer_id";
    let mut local_vm = create_avm(create_check_service_closure(closure_call_args.clone()), local_peer_id).await;

    let script = format!(
        r#"
        (seq
            (call "{set_variable_peer_id}" ("" "") [] scalar_array)
            (fold scalar_array.$.iterable! v
                (seq
                    (call v.$.peer_id! (v.$.service_id! v.$.function_name!) [v.$.args.[0]! v.$.args.[1]!])
                    (next v)
                )
            )
        )
        "#
    );

    let result = checked_call_vm!(set_variable_vm, <_>::default(), script.clone(), "", "");
    let result = call_vm!(local_vm, <_>::default(), script, "", result.data);

    assert!(is_interpreter_succeded(&result));
    assert_eq!(
        closure_call_args.service_id_var,
        Rc::new(RefCell::new("local_service_id".to_string()))
    );
    assert_eq!(
        closure_call_args.function_name_var,
        Rc::new(RefCell::new("local_function_name".to_string()))
    );
    assert_eq!(closure_call_args.args_var, Rc::new(RefCell::new(vec![2, 3])));
}

#[tokio::test]
#[ignore]
async fn flattening_streams() {
    let stream_value = json!(
        {"peer_id" : "local_peer_id", "service_id": "local_service_id", "function_name": "local_function_name", "args": [0, 1]}
    );

    let set_variable_peer_id = "set_variable";
    let mut set_variable_vm = create_avm(set_variable_call_service(stream_value), set_variable_peer_id).await;

    let closure_call_args = ClosureCallArgs::default();
    let local_peer_id = "local_peer_id";
    let mut local_vm = create_avm(create_check_service_closure(closure_call_args.clone()), local_peer_id).await;

    let script = format!(
        r#"
        (seq
            (seq
                (seq
                    (call "{set_variable_peer_id}" ("" "") [] $stream)
                    (call "{set_variable_peer_id}" ("" "") [] $stream)
                )
                (call "{set_variable_peer_id}" ("" "") [] $stream)
            )
            (fold $stream.$.[0,1,2] v
                (seq
                    (call v.$.peer_id! (v.$.service_id! v.$.function_name!) [v.$.args[0]! v.$.args[1]!])
                    (next v)
                )
            )
        )
        "#
    );

    let result = checked_call_vm!(set_variable_vm, <_>::default(), script.clone(), "", "");
    let result = call_vm!(local_vm, <_>::default(), script, "", result.data);

    assert!(is_interpreter_succeded(&result));
    assert_eq!(
        closure_call_args.service_id_var,
        Rc::new(RefCell::new("local_service_id".to_string()))
    );
    assert_eq!(
        closure_call_args.function_name_var,
        Rc::new(RefCell::new("local_function_name".to_string()))
    );
    assert_eq!(closure_call_args.args_var, Rc::new(RefCell::new(vec![0, 1])));
}

#[tokio::test]
#[ignore]
async fn test_handling_non_flattening_values() {
    let stream_value = json!(
        {"peer_id" : "local_peer_id", "service_id": "local_service_id", "function_name": "local_function_name", "args": [0, 1]}
    );

    let set_variable_peer_id = "set_variable";
    let mut set_variable_vm = create_avm(set_variable_call_service(stream_value), set_variable_peer_id).await;

    let closure_call_args = ClosureCallArgs::default();
    let local_peer_id = "local_peer_id";
    let mut local_vm = create_avm(create_check_service_closure(closure_call_args), local_peer_id).await;

    let script = format!(
        r#"
        (seq
            (seq
                (seq
                    (call "{set_variable_peer_id}" ("" "") [] $stream)
                    (call "{set_variable_peer_id}" ("" "") [] $stream)
                )
                (call "{set_variable_peer_id}" ("" "") [] $stream)
            )
            (fold $stream.$.[0,1,2]! v
                (seq
                    (call v.$.peer_id! (v.$.service_id! v.$.function_name!) [v.$.args[0]! v.$.args[1]!])
                    (next v)
                )
            )
        )
        "#
    );

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let result = call_vm!(local_vm, <_>::default(), &script, "", result.data);

    assert_eq!(result.ret_code, 1017);
    assert_eq!(
        result.error_message,
        String::from(
            r#"jvalue '[{"peer_id":"local_peer_id","service_id":"local_service_id","function_name":"local_function_name","args":[0,1]},{"peer_id":"local_peer_id","service_id":"local_service_id","function_name":"local_function_name","args":[0,1]},{"peer_id":"local_peer_id","service_id":"local_service_id","function_name":"local_function_name","args":[0,1]}]' can't be flattened, to be flattened a jvalue should have an array type and consist of zero or one values"#
        )
    );
}
