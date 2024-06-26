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
mod avm_runtime_limits;
mod config;
mod errors;
mod runner;

pub use avm::AVM;
pub use config::AVMConfig;
pub use errors::AVMError;
pub use runner::AVMMemoryStats;
pub use runner::AVMRuntimeLimits;
pub use runner::AquaVMRuntimeLimits;

pub use avm_interface::*;

pub mod avm_runner {
    pub use crate::runner::AVMRunner;
    pub use avm_interface::raw_outcome::RawAVMOutcome;
}

// Re-exports
pub use marine::ne_vec;
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

pub use errors::RunnerError;
pub type RunnerResult<T> = std::result::Result<T, RunnerError>;
