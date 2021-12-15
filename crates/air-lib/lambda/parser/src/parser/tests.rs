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

use crate::parser::LambdaParser;
use crate::ValueAccessor;

thread_local!(static TEST_PARSER: LambdaParser = LambdaParser::new());

fn parse(source_code: &str) -> Vec<ValueAccessor<'_>> {
    TEST_PARSER.with(|parser| {
        let mut errors = Vec::new();
        let lexer = crate::parser::AccessorsLexer::new(source_code);
        parser
            .parse(source_code, &mut errors, lexer)
            .expect("parsing should be successful")
    })
}

#[test]
fn field_access() {
    let field_name = "some_field_name";
    let lambda = format!(".{}", field_name);

    let actual = parse(&lambda);
    let expected = vec![ValueAccessor::FieldAccessByName { field_name }];
    assert_eq!(actual, expected);
}

#[test]
fn field_access_with_flattening() {
    let field_name = "some_field_name";
    let lambda = format!(".{}!", field_name);

    let actual = parse(&lambda);
    let expected = vec![ValueAccessor::FieldAccessByName { field_name }];
    assert_eq!(actual, expected);
}

#[test]
fn array_access() {
    let idx = 0;
    let lambda = format!(".[{}]", idx);

    let actual = parse(&lambda);
    let expected = vec![ValueAccessor::ArrayAccess { idx }];
    assert_eq!(actual, expected);
}

#[test]
fn array_access_with_flattening() {
    let idx = 0;
    let lambda = format!(".[{}]!", idx);

    let actual = parse(&lambda);
    let expected = vec![ValueAccessor::ArrayAccess { idx }];
    assert_eq!(actual, expected);
}

#[test]
fn scalar_access() {
    let scalar_name = "some_field_name";
    let lambda = format!(".[{}]", scalar_name);

    let actual = parse(&lambda);
    let expected = vec![ValueAccessor::FieldAccessByScalar { scalar_name }];
    assert_eq!(actual, expected);
}

#[test]
fn scalar_access_with_flattening() {
    let scalar_name = "some_scalar_name";
    let lambda = format!(".[{}]!", scalar_name);

    let actual = parse(&lambda);
    let expected = vec![ValueAccessor::FieldAccessByScalar { scalar_name }];
    assert_eq!(actual, expected);
}

#[test]
fn field_array_access() {
    let field_name = "some_field_name";
    let idx = 1;
    let lambda = format!(".{}.[{}]", field_name, idx);

    let actual = parse(&lambda);
    let expected = vec![
        ValueAccessor::FieldAccessByName { field_name },
        ValueAccessor::ArrayAccess { idx },
    ];
    assert_eq!(actual, expected);
}

#[test]
fn field_scalar_access() {
    let field_name = "some_field_name";
    let scalar_name = "some_scalar_name";
    let lambda = format!(".{}.[{}]", field_name, scalar_name);

    let actual = parse(&lambda);
    let expected = vec![
        ValueAccessor::FieldAccessByName { field_name },
        ValueAccessor::FieldAccessByScalar { scalar_name },
    ];
    assert_eq!(actual, expected);
}

#[test]
fn scalar_array_access() {
    let scalar_name = "some_scalar_name";
    let idx = 1;
    let lambda = format!(".[{}].[{}]", scalar_name, idx);

    let actual = parse(&lambda);
    let expected = vec![
        ValueAccessor::FieldAccessByScalar { scalar_name },
        ValueAccessor::ArrayAccess { idx },
    ];
    assert_eq!(actual, expected);
}

#[test]
fn field_array_access_without_dot() {
    let field_name = "some_field_name";
    let idx = 1;
    let lambda = format!(".{}[{}]", field_name, idx);

    let actual = parse(&lambda);
    let expected = vec![
        ValueAccessor::FieldAccessByName { field_name },
        ValueAccessor::ArrayAccess { idx },
    ];
    assert_eq!(actual, expected);
}

#[test]
fn array_field_access() {
    let field_name = "some_field_name";
    let idx = 1;
    let lambda = format!(".[{}].{}", idx, field_name);

    let actual = parse(&lambda);
    let expected = vec![
        ValueAccessor::ArrayAccess { idx },
        ValueAccessor::FieldAccessByName { field_name },
    ];
    assert_eq!(actual, expected);
}

#[test]
fn array_scalar_access() {
    let scalar_name = "some_scalar_name";
    let idx = 1;
    let lambda = format!(".[{}].[{}]", idx, scalar_name);

    let actual = parse(&lambda);
    let expected = vec![
        ValueAccessor::ArrayAccess { idx },
        ValueAccessor::FieldAccessByScalar { scalar_name },
    ];
    assert_eq!(actual, expected);
}

#[test]
fn many_array_field_access() {
    let field_name_1 = "some_field_name_1";
    let field_name_2 = "some_field_name_2";
    let idx_1 = 1;
    let idx_2 = u32::MAX;
    let lambda = format!(".[{}].{}.[{}].{}", idx_1, field_name_1, idx_2, field_name_2);

    let actual = parse(&lambda);
    let expected = vec![
        ValueAccessor::ArrayAccess { idx: idx_1 },
        ValueAccessor::FieldAccessByName {
            field_name: field_name_1,
        },
        ValueAccessor::ArrayAccess { idx: idx_2 },
        ValueAccessor::FieldAccessByName {
            field_name: field_name_2,
        },
    ];
    assert_eq!(actual, expected);
}

#[test]
fn many_array_field_scalar_access() {
    let field_name_1 = "some_field_name_1";
    let field_name_2 = "some_field_name_2";
    let idx_1 = 1;
    let idx_2 = u32::MAX;
    let scalar_name_1 = "some_scalar_name_1";
    let scalar_name_2 = "some_scalar_name_2";
    let lambda = format!(
        ".[{}].[{}].{}.[{}].[{}].{}",
        idx_1, scalar_name_1, field_name_1, idx_2, scalar_name_2, field_name_2
    );

    let actual = parse(&lambda);
    let expected = vec![
        ValueAccessor::ArrayAccess { idx: idx_1 },
        ValueAccessor::FieldAccessByScalar {
            scalar_name: scalar_name_1,
        },
        ValueAccessor::FieldAccessByName {
            field_name: field_name_1,
        },
        ValueAccessor::ArrayAccess { idx: idx_2 },
        ValueAccessor::FieldAccessByScalar {
            scalar_name: scalar_name_2,
        },
        ValueAccessor::FieldAccessByName {
            field_name: field_name_2,
        },
    ];
    assert_eq!(actual, expected);
}
