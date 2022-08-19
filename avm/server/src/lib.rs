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

mod avm;
mod config;
mod errors;
mod runner;

pub use avm::AVM;
pub use config::AVMConfig;
pub use errors::AVMError;
pub use runner::AVMMemoryStats;

pub use avm_interface::*;

pub mod avm_runner {
    pub use crate::runner::AVMRunner;
    pub use avm_interface::raw_outcome::RawAVMOutcome;
}

// Re-exports
pub use marine::ne_vec;
pub use marine::Ctx;
pub use marine::HostExportedFunc;
pub use marine::HostImportDescriptor;
pub use marine::HostImportError;
pub use marine::IType;
pub use marine::IValue;

pub use polyplets::SecurityTetraplet;

pub use avm_data_store::AnomalyData;
pub use avm_data_store::DataStore;

pub type AVMDataStore<E> = Box<dyn DataStore<Error = E> + Send + Sync + 'static>;

pub type AVMResult<T, E> = std::result::Result<T, AVMError<E>>;

pub(crate) use errors::RunnerError;
pub(crate) type RunnerResult<T> = std::result::Result<T, RunnerError>;
