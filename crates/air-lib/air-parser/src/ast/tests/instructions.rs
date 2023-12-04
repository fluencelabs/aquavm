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

#[test]
fn display_fail_scalar() {
    let arena = crate::Arena::new();
    let ast = crate::parse("(fail x)", &arena).unwrap();
    assert_eq!(ast.to_string(), "fail x");
}

#[test]
fn display_fail_literal() {
    let arena = crate::Arena::new();
    let ast = crate::parse(r#"(fail 123 "string")"#, &arena).unwrap();
    assert_eq!(ast.to_string(), r#"fail 123 "string""#);
}

#[test]
fn display_fail_last_error() {
    let arena = crate::Arena::new();
    let ast = crate::parse("(fail %last_error%)", &arena).unwrap();
    assert_eq!(ast.to_string(), "fail %last_error%");
}
