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

use aquamarine_vm::AquamarineVM;
use aquamarine_vm::AquamarineVMConfig;
use aquamarine_vm::HostExportedFunc;
use aquamarine_vm::HostImportDescriptor;
use aquamarine_vm::IType;

use std::path::PathBuf;

pub fn create_aqua_vm(call_service: HostExportedFunc) -> AquamarineVM {
    let call_service_descriptor = HostImportDescriptor {
        host_exported_func: call_service,
        argument_types: vec![IType::String, IType::String, IType::String],
        output_type: Some(IType::Record(0)),
        error_handler: None,
    };

    let config = AquamarineVMConfig {
        aquamarine_wasm_path: PathBuf::from("../target/wasm32-wasi/debug/aquamarine.wasm"),
        call_service: call_service_descriptor,
        current_peer_id: String::from("some_peer_id"),
    };

    AquamarineVM::new(config).expect("vm should be created")
}
