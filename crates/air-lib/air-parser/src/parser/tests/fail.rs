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

use super::dsl::*;
use super::parse;
use crate::ast::*;

#[test]
fn parse_fail_last_error() {
    let source_code = r#"
           (fail %last_error%)
        "#;
    let instruction = parse(source_code);
    let expected = fail_last_error(LastErrorPath::None);
    assert_eq!(instruction, expected)
}

#[test]
fn parse_fail_literals() {
    let source_code = r#"
           (fail 1 "error message")
        "#;
    let instruction = parse(source_code);
    let expected = fail_literals(1, "error message");
    assert_eq!(instruction, expected)
}
