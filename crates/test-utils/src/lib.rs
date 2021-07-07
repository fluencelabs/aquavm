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

mod call_services;
pub mod executed_state;

pub use avm_server::ne_vec::NEVec;
pub use avm_server::AVMConfig;
pub use avm_server::AVMError;
pub use avm_server::CallServiceClosure;
pub use avm_server::IType;
pub use avm_server::IValue;
pub use avm_server::InterpreterOutcome;
pub use avm_server::ParticleParameters;
pub use avm_server::AVM;
pub use call_services::*;

pub use air::interpreter_data::*;

use std::path::PathBuf;

pub(self) type JValue = serde_json::Value;

pub fn create_avm(call_service: CallServiceClosure, current_peer_id: impl Into<String>) -> AVM {
    let tmp_dir = std::env::temp_dir();

    let config = AVMConfig {
        air_wasm_path: PathBuf::from("../target/wasm32-wasi/debug/air_interpreter_server.wasm"),
        call_service,
        current_peer_id: current_peer_id.into(),
        vault_dir: tmp_dir.join("vault"),
        particle_data_store: tmp_dir,
        logging_mask: i32::MAX,
    };

    AVM::new(config).expect("vm should be created")
}

#[macro_export]
macro_rules! call_vm {
    ($vm:expr, $init_peer_id:expr, $script:expr, $prev_data:expr, $data:expr) => {
        match $vm.call_with_prev_data($init_peer_id, $script, $prev_data, $data) {
            Ok(v) if v.ret_code != 0 => {
                panic!("VM returns a error: {} {}", v.ret_code, v.error_message)
            }
            Ok(v) => v,
            Err(err) => panic!("VM call failed: {}", err),
        }
    };
}

pub fn trace_from_result(result: &InterpreterOutcome) -> ExecutionTrace {
    let data = data_from_result(result);
    data.trace
}

pub fn data_from_result(result: &InterpreterOutcome) -> InterpreterData {
    serde_json::from_slice(&result.data).expect("default serializer shouldn't fail")
}

pub fn raw_data_from_trace(trace: ExecutionTrace) -> Vec<u8> {
    let data = InterpreterData::from_execution_result(trace, <_>::default());
    serde_json::to_vec(&data).expect("default serializer shouldn't fail")
}

pub fn print_trace(result: &InterpreterOutcome, trace_name: &str) {
    let trace = trace_from_result(result);

    println!("trace {} (states_count: {}): [", trace_name, trace.len());
    for state in trace.iter() {
        println!("  {}", state);
    }
    println!("]");
}
