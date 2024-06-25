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

use once_cell::sync::Lazy;

use std::str::FromStr;

/// Minimal supported interpreter version, should be updated according to
/// [./docs/update-guide.md]
static MINIMAL_INTERPRETER_VERSION: Lazy<semver::Version> =
    Lazy::new(|| semver::Version::from_str("0.61.0").expect("valid minimal supported version specified"));

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
