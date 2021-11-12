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

use std::rc::Rc;

#[test]
fn parse_seq() {
    let source_code = r#"
        (par
            (call "" ("" "") [])
            (call "" ("" "") [])
        )
        "#;

    let instruction = parse(source_code);
    let expected = par(
        call(
            CallInstrValue::Literal(""),
            CallInstrValue::Literal(""),
            CallInstrValue::Literal(""),
            Rc::new(vec![]),
            CallOutputValue::None,
        ),
        call(
            CallInstrValue::Literal(""),
            CallInstrValue::Literal(""),
            CallInstrValue::Literal(""),
            Rc::new(vec![]),
            CallOutputValue::None,
        ),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn parse_par_par() {
    let source_code = r#"
        (par
            (par
                (call "" ("" "") [])
                (call ("" "") ("" "") [])
            )
            (call "" ("" "") [])
        )
        "#;
    let instruction = parse(source_code);
    let expected = par(
        par(
            call(
                CallInstrValue::Literal(""),
                CallInstrValue::Literal(""),
                CallInstrValue::Literal(""),
                Rc::new(vec![]),
                CallOutputValue::None,
            ),
            call(
                CallInstrValue::Literal(""),
                CallInstrValue::Literal(""),
                CallInstrValue::Literal(""),
                Rc::new(vec![]),
                CallOutputValue::None,
            ),
        ),
        call(
            CallInstrValue::Literal(""),
            CallInstrValue::Literal(""),
            CallInstrValue::Literal(""),
            Rc::new(vec![]),
            CallOutputValue::None,
        ),
    );
    assert_eq!(instruction, expected);
}
