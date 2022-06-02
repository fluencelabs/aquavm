/*
 * Copyright 2021 Fluence Labs Limited
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

mod executed_state;
mod interpreter_data;
mod stream_generations;
mod trace_pos;

pub use executed_state::*;
pub use interpreter_data::*;
pub use stream_generations::*;
pub use trace_pos::*;

use once_cell::sync::Lazy;
use std::str::FromStr;

pub static DATA_FORMAT_VERSION: Lazy<semver::Version> = Lazy::new(|| {
    semver::Version::from_str("0.2.2").expect("invalid data format version specified")
});
