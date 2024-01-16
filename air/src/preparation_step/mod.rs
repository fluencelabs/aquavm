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

mod errors;
mod interpreter_versions;
mod preparation;

pub use errors::PreparationError;
pub use interpreter_versions::interpreter_version;
pub use interpreter_versions::min_supported_version;

pub(crate) use preparation::check_version_compatibility;
pub(crate) use preparation::parse_data;
pub(crate) use preparation::prepare;
pub(crate) use preparation::ParsedDataPair;
pub(crate) use preparation::PreparationDescriptor;
