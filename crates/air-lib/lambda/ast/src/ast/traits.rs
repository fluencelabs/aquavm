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

use super::*;
use itertools::Itertools;

use std::fmt;

impl fmt::Display for LambdaAST<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LambdaAST::*;

        match self {
            Functor(functor) => write!(f, ".{}", functor),
            ValuePath(value_path) => write!(f, ".{}", value_path.iter().join(".")),
            Error => write!(f, "a parser error occurred while parsing lambda expression"),
        }
    }
}

impl fmt::Display for ValueAccessor<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ValueAccessor::*;

        match self {
            ArrayAccess { idx } => write!(f, "[{}]", idx),
            FieldAccessByName { field_name } => write!(f, "{}", field_name),
            FieldAccessByScalar { scalar_name } => write!(f, "[{}]", scalar_name),
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
