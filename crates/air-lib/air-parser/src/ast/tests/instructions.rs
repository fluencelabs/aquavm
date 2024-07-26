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

#[test]
fn display_fail_scalar() {
    let ast = crate::parse("(fail x)").unwrap();
    assert_eq!(ast.to_string(), "fail x");
}

#[test]
fn display_fail_literal() {
    let ast = crate::parse(r#"(fail 123 "string")"#).unwrap();
    assert_eq!(ast.to_string(), r#"fail 123 "string""#);
}

#[test]
fn display_fail_last_error() {
    let ast = crate::parse("(fail %last_error%)").unwrap();
    assert_eq!(ast.to_string(), "fail %last_error%");
}

#[test]
fn display_embed() {
    let ast = crate::parse("(embed [var1 var2.$.length] (#get_argument(0)+get_argument(1)#) x)").unwrap();
    assert_eq!(ast.to_string(), "embed [var1 var2.$.length] (#get_argument(0)+get_argument(1)#) x");
}
