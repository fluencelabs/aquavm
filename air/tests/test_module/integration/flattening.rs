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

use air_test_utils::call_vm;
use air_test_utils::create_avm;
use air_test_utils::set_variable_call_service;
use air_test_utils::CallServiceClosure;
use air_test_utils::IValue;
use air_test_utils::NEVec;

use serde_json::json;

use std::cell::RefCell;
use std::rc::Rc;

type ClosureSettableVar<T> = Rc<RefCell<T>>;

#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct ClosureCallArgs {
    pub(self) service_id_var: Rc<RefCell<String>>,
    pub(self) function_name_var: ClosureSettableVar<String>,
    pub(self) args_var: ClosureSettableVar<Vec<i32>>,
    pub(self) tetraplets: ClosureSettableVar<Vec<Vec<String>>>,
}

fn create_check_service_closure(closure_call_args: ClosureCallArgs) -> CallServiceClosure {
    Box::new(move |_, args| -> Option<IValue> {
        use std::ops::Deref;

        let service_id = match &args[0] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };
        *closure_call_args.service_id_var.deref().borrow_mut() = service_id.clone();

        let function_name = match &args[1] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };
        *closure_call_args.function_name_var.deref().borrow_mut() = function_name.clone();

        let call_args = match &args[2] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };

        let call_args: Vec<i32> = serde_json::from_str(call_args).expect("json deserialization shouldn't fail");
        *closure_call_args.args_var.deref().borrow_mut() = call_args;

        Some(IValue::Record(
            NEVec::new(vec![IValue::S32(0), IValue::String(r#""""#.to_string())]).unwrap(),
        ))
    })
}

#[test]
fn flattening_scalar_arrays() {
    let scalar_array = json!({"iterable": [
        {"peer_id" : "local_peer_id", "service_id": "local_service_id", "function_name": "local_function_name", "args": [0, 1]},
        {"peer_id" : "local_peer_id", "service_id": "local_service_id", "function_name": "local_function_name", "args": [2, 3]},
    ]});

    let scalar_array = serde_json::to_string(&scalar_array).expect("the default serializer shouldn't fail");
    let set_variable_peer_id = "set_variable";
    let mut set_variable_vm = create_avm(set_variable_call_service(scalar_array), set_variable_peer_id);

    let closure_call_args = ClosureCallArgs::default();
    let local_peer_id = "local_peer_id";
    let mut local_vm = create_avm(create_check_service_closure(closure_call_args.clone()), local_peer_id);

    let script = format!(
        r#"
        (seq
            (call "{0}" ("" "") [] scalar_array)
            (fold scalar_array.$.iterable! v
                (seq
                    (call v.$.peer_id! (v.$.service_id! v.$.function_name!) [v.$.args[0]! v.$.args[1]!])
                    (next v)
                )
            )
        )
        "#,
        set_variable_peer_id
    );

    let res = call_vm!(set_variable_vm, "asd", script.clone(), "", "");
    let res = call_vm!(local_vm, "asd", script.clone(), "", res.data);

    assert_eq!(res.ret_code, 0);
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

#[test]
fn flattening_streams() {
    let stream_value = json!(
        {"peer_id" : "local_peer_id", "service_id": "local_service_id", "function_name": "local_function_name", "args": [0, 1]}
    );

    let stream_value = serde_json::to_string(&stream_value).expect("the default serializer shouldn't fail");
    let set_variable_peer_id = "set_variable";
    let mut set_variable_vm = create_avm(set_variable_call_service(stream_value), set_variable_peer_id);

    let closure_call_args = ClosureCallArgs::default();
    let local_peer_id = "local_peer_id";
    let mut local_vm = create_avm(create_check_service_closure(closure_call_args.clone()), local_peer_id);

    let script = format!(
        r#"
        (seq
            (seq
                (seq
                    (call "{0}" ("" "") [] $stream)
                    (call "{0}" ("" "") [] $stream)
                )
                (call "{0}" ("" "") [] $stream)
            )
            (fold $stream.$.[0,1,2] v
                (seq
                    (call v.$.peer_id! (v.$.service_id! v.$.function_name!) [v.$.args[0]! v.$.args[1]!])
                    (next v)
                )
            )
        )
        "#,
        set_variable_peer_id
    );

    let res = call_vm!(set_variable_vm, "asd", script.clone(), "", "");
    let res = call_vm!(local_vm, "asd", script.clone(), "", res.data);

    assert_eq!(res.ret_code, 0);
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

#[test]
fn flattening_empty_values() {
    let stream_value = json!(
        {"args": []}
    );

    let stream_value = serde_json::to_string(&stream_value).expect("the default serializer shouldn't fail");
    let set_variable_peer_id = "set_variable";
    let mut set_variable_vm = create_avm(set_variable_call_service(stream_value), set_variable_peer_id);

    let closure_call_args = ClosureCallArgs::default();
    let local_peer_id = "local_peer_id";
    let mut local_vm = create_avm(create_check_service_closure(closure_call_args.clone()), local_peer_id);

    let script = format!(
        r#"
        (seq
            (call "{0}" ("" "") [] $stream)
            (call "{1}" ("" "") [$stream.$.args!]) ; here $stream.$.args returns an empty array
        )
        "#,
        set_variable_peer_id, local_peer_id
    );

    let res = call_vm!(set_variable_vm, "asd", script.clone(), "", "");
    let res = call_vm!(local_vm, "asd", script.clone(), "", res.data);

    assert_eq!(res.ret_code, 0);
    assert_eq!(closure_call_args.args_var, Rc::new(RefCell::new(vec![])));
}

#[test]
fn test_handling_non_flattening_values() {
    let stream_value = json!(
        {"peer_id" : "local_peer_id", "service_id": "local_service_id", "function_name": "local_function_name", "args": [0, 1]}
    );

    let stream_value = serde_json::to_string(&stream_value).expect("the default serializer shouldn't fail");
    let set_variable_peer_id = "set_variable";
    let mut set_variable_vm = create_avm(set_variable_call_service(stream_value), set_variable_peer_id);

    let closure_call_args = ClosureCallArgs::default();
    let local_peer_id = "local_peer_id";
    let mut local_vm = create_avm(create_check_service_closure(closure_call_args.clone()), local_peer_id);

    let script = format!(
        r#"
        (seq
            (seq
                (seq
                    (call "{0}" ("" "") [] $stream)
                    (call "{0}" ("" "") [] $stream)
                )
                (call "{0}" ("" "") [] $stream)
            )
            (fold $stream.$.[0,1,2]! v
                (seq
                    (call v.$.peer_id! (v.$.service_id! v.$.function_name!) [v.$.args[0]! v.$.args[1]!])
                    (next v)
                )
            )
        )
        "#,
        set_variable_peer_id
    );

    let res = call_vm!(set_variable_vm, "asd", script.clone(), "", "");
    let res = call_vm!(local_vm, "asd", script.clone(), "", res.data);

    assert_eq!(res.ret_code, 1017);
    assert_eq!(
        res.error_message,
        String::from(
            r#"jvalue '[{"peer_id":"local_peer_id","service_id":"local_service_id","function_name":"local_function_name","args":[0,1]},{"peer_id":"local_peer_id","service_id":"local_service_id","function_name":"local_function_name","args":[0,1]},{"peer_id":"local_peer_id","service_id":"local_service_id","function_name":"local_function_name","args":[0,1]}]' can't be flattened, to be flattened a jvalue should have an array type and consist of zero or one values"#
        )
    );
}
