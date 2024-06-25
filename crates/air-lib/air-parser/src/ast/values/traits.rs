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
use std::fmt;

impl fmt::Display for Scalar<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for ScalarWithLambda<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.name, self.lambda)
    }
}

impl fmt::Display for Stream<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for CanonStream<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for CanonStreamWithLambda<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.name, self.lambda)
    }
}

impl fmt::Display for CanonStreamMap<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for CanonStreamMapWithLambda<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.name, self.lambda)
    }
}

impl fmt::Display for ImmutableVariable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ImmutableVariable::*;

        match self {
            Scalar(scalar) => write!(f, "{scalar}"),
            CanonStream(canon_stream) => write!(f, "{canon_stream}"),
            CanonStreamMap(canon_stream_map) => write!(f, "{canon_stream_map}"),
        }
    }
}

impl fmt::Display for ImmutableVariableWithLambda<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ImmutableVariableWithLambda::*;

        match self {
            Scalar(scalar) => write!(f, "{scalar}"),
            CanonStream(canon_stream) => write!(f, "{canon_stream}"),
            CanonStreamMap(canon_stream_map) => write!(f, "{canon_stream_map}"),
        }
    }
}

impl fmt::Display for StreamMap<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
