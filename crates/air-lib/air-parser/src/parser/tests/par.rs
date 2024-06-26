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
            ResolvableToPeerIdVariable::Literal(""),
            ResolvableToStringVariable::Literal(""),
            ResolvableToStringVariable::Literal(""),
            Rc::new(vec![]),
            CallOutputValue::None,
        ),
        call(
            ResolvableToPeerIdVariable::Literal(""),
            ResolvableToStringVariable::Literal(""),
            ResolvableToStringVariable::Literal(""),
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
                (call "" ("" "") [])
            )
            (call "" ("" "") [])
        )
        "#;
    let instruction = parse(source_code);
    let expected = par(
        par(
            call(
                ResolvableToPeerIdVariable::Literal(""),
                ResolvableToStringVariable::Literal(""),
                ResolvableToStringVariable::Literal(""),
                Rc::new(vec![]),
                CallOutputValue::None,
            ),
            call(
                ResolvableToPeerIdVariable::Literal(""),
                ResolvableToStringVariable::Literal(""),
                ResolvableToStringVariable::Literal(""),
                Rc::new(vec![]),
                CallOutputValue::None,
            ),
        ),
        call(
            ResolvableToPeerIdVariable::Literal(""),
            ResolvableToStringVariable::Literal(""),
            ResolvableToStringVariable::Literal(""),
            Rc::new(vec![]),
            CallOutputValue::None,
        ),
    );
    assert_eq!(instruction, expected);
}
