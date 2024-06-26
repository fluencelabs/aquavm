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

use crate::ast::ImmutableValue;

use air_lambda_ast::LambdaAST;
use air_lambda_ast::ValueAccessor;

#[test]
// https://github.com/fluencelabs/aquavm/issues/263
fn issue_263() {
    let val = ImmutableValue::LastError(Some(
        LambdaAST::try_from_accessors(vec![ValueAccessor::FieldAccessByName {
            field_name: "message",
        }])
        .unwrap(),
    ));
    assert_eq!(val.to_string(), "%last_error%.$.message");
}
