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

use air_lambda_ast::{LambdaAST, ValueAccessor};

use super::parse;
use crate::ast::Embed;
use crate::ast::EmbedOutputValue;
use crate::ast::ImmutableValue;
use crate::ast::ImmutableVariable;
use crate::ast::Scalar;

#[test]
fn embed_with_var() {
    let embed_script = r#"
def sum(x):
   n = 0
   for i in range(x):
      n += i
   return n

sum(get_value(0) + get_value(1))
              "#;
    let source_code = format!(
        "
        (embed [x %last_error%.$.message!] (#{}#)
              var)
    ",
        embed_script
    );

    let actual = parse(&source_code);
    let expected = crate::ast::Instruction::Embed(
        Embed {
            args: vec![
                ImmutableValue::Variable(ImmutableVariable::Scalar(Scalar::new("x", 17.into()))),
                ImmutableValue::LastError(Some(
                    LambdaAST::try_from_accessors(vec![ValueAccessor::FieldAccessByName {
                        field_name: "message",
                    }])
                    .unwrap(),
                )),
            ]
            .into(),
            script: embed_script,
            output: EmbedOutputValue::Scalar(Scalar::new("var", 180.into())),
        }
        .into(),
    );

    assert_eq!(actual, expected);
}

#[test]
fn embed_no_var() {
    let embed_script = r#" get_tetraplet(0).peer_pk + ": " + get_value(1) "#;
    let source_code = format!(
        r#"
        (embed [x %last_error%.$.message!] (#{}#))
    "#,
        embed_script
    );

    let actual = parse(&source_code);
    let expected = crate::ast::Instruction::Embed(
        Embed {
            args: vec![
                ImmutableValue::Variable(ImmutableVariable::Scalar(Scalar::new("x", 17.into()))),
                ImmutableValue::LastError(Some(
                    LambdaAST::try_from_accessors(vec![ValueAccessor::FieldAccessByName {
                        field_name: "message",
                    }])
                    .unwrap(),
                )),
            ]
            .into(),
            script: embed_script,
            output: EmbedOutputValue::None,
        }
        .into(),
    );

    assert_eq!(actual, expected);
}

#[test]
fn embed_with_hash_symbol_string() {
    let source_code = r##"
        (embed [x %last_error%.$.message!] (#"the hash inside the string: \x23"#) var)
    "##;

    let actual = parse(source_code);
    let expected = crate::ast::Instruction::Embed(
        Embed {
            args: vec![
                ImmutableValue::Variable(ImmutableVariable::Scalar(Scalar::new("x", 17.into()))),
                ImmutableValue::LastError(Some(
                    LambdaAST::try_from_accessors(vec![ValueAccessor::FieldAccessByName {
                        field_name: "message",
                    }])
                    .unwrap(),
                )),
            ]
            .into(),
            script: r#""the hash inside the string: \x23""#,
            output: EmbedOutputValue::Scalar(Scalar::new("var", 83.into())),
        }
        .into(),
    );

    assert_eq!(actual, expected);
}
