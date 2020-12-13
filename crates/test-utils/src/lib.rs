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

#![warn(rust_2018_idioms)]
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]

use aquamarine_vm::vec1::Vec1;
use aquamarine_vm::AquamarineVM;
use aquamarine_vm::AquamarineVMConfig;
use aquamarine_vm::HostExportedFunc;
use aquamarine_vm::HostImportDescriptor;
use aquamarine_vm::IType;
use aquamarine_vm::IValue;

use std::collections::HashMap;
use std::path::PathBuf;

type JValue = serde_json::Value;

pub fn create_aqua_vm(
    call_service: HostExportedFunc,
    current_peer_id: impl Into<String>,
) -> AquamarineVM {
    let call_service_descriptor = HostImportDescriptor {
        host_exported_func: call_service,
        argument_types: vec![IType::String, IType::String, IType::String, IType::String],
        output_type: Some(IType::Record(0)),
        error_handler: None,
    };

    let tmp_dir = std::env::temp_dir();

    let config = AquamarineVMConfig {
        aquamarine_wasm_path: PathBuf::from("../target/wasm32-wasi/debug/aquamarine.wasm"),
        call_service: call_service_descriptor,
        current_peer_id: current_peer_id.into(),
        particle_data_store: tmp_dir,
        logging_mask: i64::max_value(),
    };

    AquamarineVM::new(config).expect("vm should be created")
}

pub fn unit_call_service() -> HostExportedFunc {
    Box::new(|_, _| -> Option<IValue> {
        Some(IValue::Record(
            Vec1::new(vec![
                IValue::S32(0),
                IValue::String(String::from("\"test\"")),
            ])
            .unwrap(),
        ))
    })
}

pub fn echo_string_call_service() -> HostExportedFunc {
    Box::new(|_, args| -> Option<IValue> {
        let arg = match &args[2] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };

        let arg: Vec<String> = serde_json::from_str(arg).unwrap();

        Some(IValue::Record(
            Vec1::new(vec![
                IValue::S32(0),
                IValue::String(format!("\"{}\"", arg[0])),
            ])
            .unwrap(),
        ))
    })
}

pub fn echo_number_call_service() -> HostExportedFunc {
    Box::new(|_, args| -> Option<IValue> {
        let arg = match &args[2] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };

        let arg: Vec<String> = serde_json::from_str(arg).unwrap();

        Some(IValue::Record(
            Vec1::new(vec![IValue::S32(0), IValue::String(arg[0].clone())]).unwrap(),
        ))
    })
}

pub fn set_variable_call_service(json: impl Into<String>) -> HostExportedFunc {
    let json = json.into();
    Box::new(move |_, _| -> Option<IValue> {
        Some(IValue::Record(
            Vec1::new(vec![IValue::S32(0), IValue::String(json.clone())]).unwrap(),
        ))
    })
}

pub fn set_variables_call_service(ret_mapping: HashMap<String, String>) -> HostExportedFunc {
    Box::new(move |_, args| -> Option<IValue> {
        let arg_name = match &args[2] {
            IValue::String(json_str) => {
                let json = serde_json::from_str(json_str).expect("a valid json");
                match json {
                    JValue::Array(array) => match array.first() {
                        Some(JValue::String(str)) => str.to_string(),
                        _ => String::from("default"),
                    },
                    _ => String::from("default"),
                }
            }
            _ => String::from("default"),
        };

        let result = ret_mapping
            .get(&arg_name)
            .cloned()
            .unwrap_or(String::from(r#""test""#));

        Some(IValue::Record(
            Vec1::new(vec![IValue::S32(0), IValue::String(result.clone())]).unwrap(),
        ))
    })
}

#[macro_export]
macro_rules! call_vm {
    ($vm:expr, $init_peer_id:expr, $script:expr, $prev_data:expr, $data:expr) => {
        match $vm.call_with_prev_data($init_peer_id, $script, $prev_data, $data) {
            Ok(v) => v,
            Err(err) => panic!("VM call failed: {}", err),
        }
    };
}
