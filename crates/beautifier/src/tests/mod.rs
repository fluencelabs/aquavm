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

#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]

mod beautifier;

use crate::{beautify, beautify_to_string, BeautifyError};

#[test]
fn beautify_valid() {
    let air_script = "(seq (null) (null))";
    let mut buffer = vec![];
    let res = beautify(air_script, &mut buffer, false);
    assert!(res.is_ok());
    assert_eq!(std::str::from_utf8(&buffer).unwrap(), "null\nnull\n");
}

#[test]
fn beautify_valid_with_patterns() {
    let air_script = "(seq (null) (null))";
    let mut buffer = vec![];
    let res = beautify(air_script, &mut buffer, true);
    assert!(res.is_ok());
    assert_eq!(std::str::from_utf8(&buffer).unwrap(), "null\nnull\n");
}

#[test]
fn beautify_invalid() {
    let air_script = "(seq (null))";
    let mut buffer = vec![];
    let res = beautify(air_script, &mut buffer, false);
    assert!(matches!(res, Err(BeautifyError::Parse(_))));
}

#[test]
fn beautify_to_string_valid() {
    let air_script = "(seq (null) (null))";
    let res = beautify_to_string(air_script).unwrap();
    assert_eq!(res, "null\nnull\n");
}

#[test]
fn beautify_to_string_invalid() {
    let air_script = "(seq (null))";
    let res = beautify_to_string(air_script);
    assert!(res.is_err());
}
