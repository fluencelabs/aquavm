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

use crate::Functor;
use crate::LambdaAST;
use crate::ValueAccessor;

pub use non_empty_vec::EmptyError;
use non_empty_vec::NonEmpty;

impl<'input> LambdaAST<'input> {
    pub fn try_from_accessors(accessors: Vec<ValueAccessor<'input>>) -> Result<Self, EmptyError> {
        let value_path = NonEmpty::try_from(accessors)?;
        let lambda_ast = Self::ValuePath(value_path);

        Ok(lambda_ast)
    }

    pub fn from_functor(functor: Functor) -> Self {
        Self::Functor(functor)
    }
}
