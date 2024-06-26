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

use air_lambda_ast::LambdaAST;
use polyplets::SecurityTetraplet;

pub(crate) fn populate_tetraplet_with_lambda(
    mut tetraplet: SecurityTetraplet,
    lambda: &LambdaAST<'_>,
) -> SecurityTetraplet {
    match lambda {
        LambdaAST::ValuePath(_) => {
            tetraplet.add_lens(&lambda.to_string());
            tetraplet
        }
        LambdaAST::Functor(_) => SecurityTetraplet::new("", "", "", lambda.to_string()),
    }
}
