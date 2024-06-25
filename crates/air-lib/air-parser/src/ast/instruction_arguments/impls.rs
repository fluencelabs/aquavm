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

use super::ApResult;
use super::CallOutputValue;
use super::NewArgument;
use super::Scalar;
use super::Stream;
use crate::parser::lexer::AirPos;

impl<'i> NewArgument<'i> {
    pub fn name(&self) -> &'i str {
        match self {
            Self::Scalar(scalar) => scalar.name,
            Self::Stream(stream) => stream.name,
            Self::CanonStream(canon_stream) => canon_stream.name,
            Self::StreamMap(stream_map) => stream_map.name,
            Self::CanonStreamMap(canon_stream_map) => canon_stream_map.name,
        }
    }
}

impl<'i> ApResult<'i> {
    pub fn scalar(name: &'i str, position: AirPos) -> Self {
        Self::Scalar(Scalar { name, position })
    }

    pub fn stream(name: &'i str, position: AirPos) -> Self {
        Self::Stream(Stream { name, position })
    }

    pub fn name(&self) -> &'i str {
        match self {
            Self::Scalar(scalar) => scalar.name,
            Self::Stream(stream) => stream.name,
        }
    }
}

impl<'i> CallOutputValue<'i> {
    pub fn scalar(name: &'i str, position: AirPos) -> Self {
        Self::Scalar(Scalar { name, position })
    }

    pub fn stream(name: &'i str, position: AirPos) -> Self {
        Self::Stream(Stream { name, position })
    }
}
