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

mod cid_info;
mod cid_store;
mod executed_state;
mod generation_idx;
mod interpreter_data;
mod raw_value;
mod rkyv;
mod trace;
mod trace_pos;

pub use cid_info::*;
pub use cid_store::*;
pub use executed_state::*;
pub use generation_idx::*;
pub use interpreter_data::*;
pub use raw_value::*;
pub use trace::*;
pub use trace_pos::*;

use air_interpreter_value::JValue;
use once_cell::sync::Lazy;

use std::str::FromStr;

/// Interpreter data version, more info in
/// [./docs/update-guide.md]
static INTERPRETER_DATA_VERSION: Lazy<semver::Version> = Lazy::new(|| {
    semver::Version::from_str(env!("CARGO_PKG_VERSION"))
        .expect("invalid data format version specified")
});

pub fn data_version() -> &'static semver::Version {
    Lazy::force(&INTERPRETER_DATA_VERSION)
}
