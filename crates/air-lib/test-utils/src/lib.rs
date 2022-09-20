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

#![forbid(unsafe_code)]
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

pub mod call_services;
pub mod executed_state;
pub mod test_runner;

#[cfg(feature = "test_with_native_code")]
mod native_test_runner;
#[cfg(not(feature = "test_with_native_code"))]
mod wasm_test_runner;

pub use air::interpreter_data::*;
pub use avm_interface::raw_outcome::*;
pub use avm_server::*;

pub mod prelude {
    pub use super::*;
    pub use call_services::*;
    pub use executed_state::*;
    pub use test_runner::*;

    pub use air::interpreter_data::*;
    pub use avm_server::*;

    pub use fstrings::f;
    pub use fstrings::format_args_f;

    pub use serde_json::json;
}

pub type CallServiceClosure = Box<dyn Fn(CallRequestParams) -> CallServiceResult + 'static>;

pub type JValue = serde_json::Value;

#[macro_export]
macro_rules! checked_call_vm {
    ($vm:expr, $test_run_parameters:expr, $script:expr, $prev_data:expr, $data:expr) => {{
        let data = $data;
        match $vm.call($script, $prev_data, data.clone(), $test_run_parameters) {
            Ok(v) if v.ret_code != 0 => {
                let dat = data_from_data(data);
                print_trace1(&dat.trace, "cur_data");
                eprintln!("global streams: {:?}", dat.global_streams);
                eprintln!("restricted streams: {:?}", dat.restricted_streams);
                panic!("VM returns a error: {} {}", v.ret_code, v.error_message)
            }
            Ok(v) => v,
            Err(err) => panic!("VM call failed: {}", err),
        }
    }};
}

#[macro_export]
macro_rules! call_vm {
    ($vm:expr, $test_run_parameters:expr, $script:expr, $prev_data:expr, $data:expr) => {
        match $vm.call($script, $prev_data, $data, $test_run_parameters) {
            Ok(v) => v,
            Err(err) => panic!("VM call failed: {}", err),
        }
    };
}

pub fn trace_from_result(result: &RawAVMOutcome) -> ExecutionTrace {
    let data = data_from_result(result);
    data.trace
}

pub fn data_from_data(data: impl Into<Vec<u8>>) -> InterpreterData {
    serde_json::from_slice(&data.into()).expect("default serializer shouldn't fail")
}

pub fn data_from_result(result: &RawAVMOutcome) -> InterpreterData {
    data_from_data(result.data.clone())
}

pub fn raw_data_from_trace(trace: impl Into<ExecutionTrace>) -> Vec<u8> {
    let data =
        InterpreterData::from_execution_result(trace.into(), <_>::default(), <_>::default(), 0);
    serde_json::to_vec(&data).expect("default serializer shouldn't fail")
}

#[macro_export]
macro_rules! assert_next_pks {
    ($expected:expr, $actual:expr) => {
        let expected: std::collections::HashSet<_> =
            $expected.into_iter().map(|s| s.as_str()).collect();
        let actual: std::collections::HashSet<_> = $actual.into_iter().map(|s| *s).collect();

        assert_eq!(expected, actual)
    };
}

pub fn print_trace1(trace: &ExecutionTrace, trace_name: &str) {
    println!("trace {} (states_count: {}): [", trace_name, trace.len());
    for (id, state) in trace.iter().enumerate() {
        println!("  {}: {}", id, state);
    }
    println!("]");
}

pub fn print_trace(result: &RawAVMOutcome, trace_name: &str) {
    let trace = trace_from_result(result);

    println!("trace {} (states_count: {}): [", trace_name, trace.len());
    for (id, state) in trace.iter().enumerate() {
        println!("  {}: {}", id, state);
    }
    println!("]");
}

#[macro_export]
macro_rules! rc {
    ($expr:expr) => {
        std::rc::Rc::new($expr)
    };
}

use air::ToErrorCode;
use air_interpreter_interface::INTERPRETER_SUCCESS;

pub fn is_interpreter_succeded(result: &RawAVMOutcome) -> bool {
    result.ret_code == INTERPRETER_SUCCESS
}

pub fn check_error(result: &RawAVMOutcome, error: impl ToErrorCode + ToString) -> bool {
    result.ret_code == error.to_error_code() && result.error_message == error.to_string()
}
