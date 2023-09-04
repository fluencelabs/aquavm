/*
 * Copyright 2022 Fluence Labs Limited
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

use once_cell::sync::Lazy;

use std::str::FromStr;

/// Minimal supported interpreter version, should be updated according to
/// [./docs/update-guide.md]
static MINIMAL_INTERPRETER_VERSION: Lazy<semver::Version> =
    Lazy::new(|| semver::Version::from_str("0.45.0").expect("valid minimal supported version specified"));

/// Current interpreter version, more info in
/// [./docs/update-guide.md]
static INTERPRETER_VERSION: Lazy<semver::Version> =
    Lazy::new(|| semver::Version::from_str(env!("CARGO_PKG_VERSION")).expect("invalid data format version specified"));

// This local is intended to check that set version is correct at the AquaVM start for graceful error message.
thread_local!(static _MINIMAL_INTERPRETER_VERSION_CHECK: &'static semver::Version = Lazy::force(&MINIMAL_INTERPRETER_VERSION));

/// Returns a minimal support version by this interpreter.
pub fn min_supported_version() -> &'static semver::Version {
    Lazy::force(&MINIMAL_INTERPRETER_VERSION)
}

/// Returns a current interpreter version.
pub fn interpreter_version() -> &'static semver::Version {
    Lazy::force(&INTERPRETER_VERSION)
}
