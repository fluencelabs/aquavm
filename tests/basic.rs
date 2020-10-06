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

use aquamarine_vm::AquamarineVM;
use aquamarine_vm::AquamarineVMConfig;
use aquamarine_vm::Ctx;
use aquamarine_vm::HostImportDescriptor;
use aquamarine_vm::IType;
use aquamarine_vm::IValue;

use serde_json::json;

use std::path::PathBuf;

#[test]
fn call() {
    let call_service = move |_ctx: &Ctx, _args: Vec<IValue>| -> Option<IValue> { None };

    let call_service_descriptor = HostImportDescriptor {
        host_exported_func: Box::new(call_service),
        argument_types: vec![IType::String, IType],
        output_type: None,
        error_handler: None,
    };

    let config = AquamarineVMConfig {
        aquamarine_wasm_path: PathBuf::from("./target/wasm32-wasi/debug/aquamarine.wasm"),
        call_service: call_service_descriptor,
        current_peer_id: String::from("some_peer_id"),
    };

    let mut vm = AquamarineVM::new(config).expect("vm should be created");
    let script = String::from("(seq ((call (%current% (local_service_id local_fn_name) () result_name)) (call (remote_peer_id (service_id fn_name) () g))))");

    vm.call(json!([String::from("asd"), script, String::from("{}"),]));
}
