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
use air_lambda_parser::LambdaAST;

impl<'i> ScalarWithLambda<'i> {
    pub fn new(name: &'i str, lambda: LambdaAST<'i>, position: AirPos) -> Self {
        Self {
            name,
            lambda,
            position,
        }
    }

    #[cfg(test)]
    pub(crate) fn from_raw_lambda(
        name: &'i str,
        lambda: Vec<air_lambda_parser::ValueAccessor<'i>>,
        position: AirPos,
    ) -> Self {
        let lambda = LambdaAST::try_from_accessors(lambda).unwrap();
        Self {
            name,
            lambda,
            position,
        }
    }
}

impl<'i> CanonStream<'i> {
    pub fn new(name: &'i str, position: AirPos) -> Self {
        Self { name, position }
    }
}

impl<'i> CanonStreamWithLambda<'i> {
    pub fn new(name: &'i str, lambda: LambdaAST<'i>, position: AirPos) -> Self {
        Self {
            name,
            lambda,
            position,
        }
    }
}

impl<'i> CanonStreamMap<'i> {
    pub fn new(name: &'i str, position: AirPos) -> Self {
        Self { name, position }
    }
}

impl<'i> CanonStreamMapWithLambda<'i> {
    pub fn new(name: &'i str, lambda: LambdaAST<'i>, position: AirPos) -> Self {
        Self {
            name,
            lambda,
            position,
        }
    }
}

impl<'i> Scalar<'i> {
    pub fn new(name: &'i str, position: AirPos) -> Self {
        Self { name, position }
    }
}

impl<'i> Stream<'i> {
    pub fn new(name: &'i str, position: AirPos) -> Self {
        Self { name, position }
    }
}

impl<'i> ImmutableVariable<'i> {
    pub fn scalar(name: &'i str, position: AirPos) -> Self {
        Self::Scalar(Scalar::new(name, position))
    }

    pub fn canon_stream(name: &'i str, position: AirPos) -> Self {
        Self::CanonStream(CanonStream::new(name, position))
    }

    pub fn canon_stream_map(name: &'i str, position: AirPos) -> Self {
        Self::CanonStreamMap(CanonStreamMap::new(name, position))
    }

    pub fn name(&self) -> &'i str {
        match self {
            ImmutableVariable::Scalar(scalar) => scalar.name,
            ImmutableVariable::CanonStream(stream) => stream.name,
            ImmutableVariable::CanonStreamMap(canon_stream_map) => canon_stream_map.name,
        }
    }
}

impl<'i> ImmutableVariableWithLambda<'i> {
    pub fn scalar(name: &'i str, lambda: LambdaAST<'i>, position: AirPos) -> Self {
        Self::Scalar(ScalarWithLambda::new(name, lambda, position))
    }

    pub fn canon_stream(name: &'i str, lambda: LambdaAST<'i>, position: AirPos) -> Self {
        Self::CanonStream(CanonStreamWithLambda::new(name, lambda, position))
    }

    pub fn canon_stream_map(name: &'i str, lambda: LambdaAST<'i>, position: AirPos) -> Self {
        Self::CanonStreamMap(CanonStreamMapWithLambda::new(name, lambda, position))
    }

    pub fn name(&self) -> &'i str {
        match self {
            ImmutableVariableWithLambda::Scalar(scalar) => scalar.name,
            ImmutableVariableWithLambda::CanonStream(canon_stream) => canon_stream.name,
            ImmutableVariableWithLambda::CanonStreamMap(canon_stream_map) => canon_stream_map.name,
        }
    }

    pub fn lambda(&self) -> &LambdaAST<'i> {
        match self {
            ImmutableVariableWithLambda::Scalar(scalar) => &scalar.lambda,
            ImmutableVariableWithLambda::CanonStream(canon_stream) => &canon_stream.lambda,
            ImmutableVariableWithLambda::CanonStreamMap(canon_stream_map) => {
                &canon_stream_map.lambda
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn from_raw_value_path(
        name: &'i str,
        lambda: Vec<air_lambda_parser::ValueAccessor<'i>>,
        position: AirPos,
    ) -> Self {
        let scalar = ScalarWithLambda::from_raw_lambda(name, lambda, position);
        Self::Scalar(scalar)
    }
}

impl<'i> StreamMap<'i> {
    pub fn new(name: &'i str, position: AirPos) -> Self {
        Self { name, position }
    }
}

impl<'lens> InstructionErrorAST<'lens> {
    pub fn new(lens: Option<LambdaAST<'lens>>) -> Self {
        Self { lens }
    }
}
