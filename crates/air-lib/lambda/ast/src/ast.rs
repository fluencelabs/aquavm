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

mod impls;
mod traits;

use non_empty_vec::NonEmpty;
use serde::Deserialize;
use serde::Serialize;

// TODO: rename lambda to smth more appropriate
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum LambdaAST<'input> {
    /// Various functors that could applied to a value.
    Functor(Functor),
    /// Each value in AIR could be represented as a tree and
    /// this variant acts as a path in such trees.
    #[serde(borrow)]
    ValuePath(NonEmpty<ValueAccessor<'input>>),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum ValueAccessor<'input> {
    // (.)?[$idx]
    ArrayAccess { idx: u32 },

    // .field
    FieldAccessByName { field_name: &'input str },

    // (.)?[field]
    FieldAccessByScalar { scalar_name: &'input str },

    // needed to allow parser catch all errors from a lambda expression without stopping
    // on the very first one. Although, this variant is guaranteed not to be present in a lambda.
    Error,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Functor {
    /// Returns a length of a value if this value has array type (json array or canon stream)
    /// or a error if not.
    Length,
}
