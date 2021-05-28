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
use air_test_utils::CallServiceClosure;
use air_test_utils::IValue;
use air_test_utils::NEVec;

use serde_json::Value as JValue;

#[test]
fn empty_stream() {
    fn arg_type_check_closure() -> CallServiceClosure {
        Box::new(move |_, args| -> Option<IValue> {
            let call_args = match &args[2] {
                IValue::String(str) => str,
                _ => unreachable!(),
            };

            let actual_call_args: Vec<Vec<JValue>> =
                serde_json::from_str(call_args).expect("json deserialization shouldn't fail");
            let expected_call_args: Vec<Vec<JValue>> = vec![vec![]];

            assert_eq!(actual_call_args, expected_call_args);

            Some(IValue::Record(
                NEVec::new(vec![IValue::S32(0), IValue::String(r#""""#.to_string())]).unwrap(),
            ))
        })
    }

    let mut vm = create_avm(arg_type_check_closure(), "A");

    let script = r#"
        (seq
            (call "A" ("" "") [$stream] $other_stream)
            (null)
        )"#;

    let _ = call_vm!(vm, "", script, "", "");
}
