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

use super::*;
use itertools::Itertools;

use std::fmt;

impl fmt::Display for LambdaAST<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LambdaAST::*;

        match self {
            Functor(functor) => write!(f, ".{functor}"),
            ValuePath(value_path) => write!(f, ".$.{}", value_path.iter().join(".")),
        }
    }
}

impl fmt::Display for ValueAccessor<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ValueAccessor::*;

        match self {
            ArrayAccess { idx } => write!(f, "[{idx}]"),
            FieldAccessByName { field_name } => write!(f, "{field_name}"),
            FieldAccessByScalar { scalar_name } => write!(f, "[{scalar_name}]"),
            Error => write!(f, "a parser error occurred while parsing lambda expression"),
        }
    }
}

impl fmt::Display for Functor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Functor::*;

        match self {
            Length => write!(f, "length"),
        }
    }
}
