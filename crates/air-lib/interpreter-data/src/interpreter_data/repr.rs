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

use crate::Versions;

use air_interpreter_sede::Format;
use air_interpreter_sede::FromSerialized;
use air_interpreter_sede::MsgPackFormat;
use air_interpreter_sede::Representation;

#[derive(Default, Debug)]
pub struct InterpreterDataEnvelopeRepr;

pub type InterpreterDataEnvelopeFormat = MsgPackFormat;

impl Representation for InterpreterDataEnvelopeRepr {
    type SerializeError = rmp_serde::encode::Error;
    type DeserializeError = rmp_serde::decode::Error;
    type WriteError = rmp_serde::encode::Error;
    type Format = InterpreterDataEnvelopeFormat;
    type SerializedValue = Vec<u8>; // TODO a typed wrapper

    fn get_format(&self) -> InterpreterDataEnvelopeFormat {
        InterpreterDataEnvelopeFormat::default()
    }
}

impl FromSerialized<Versions> for InterpreterDataEnvelopeRepr {
    #[inline]
    fn deserialize(&self, repr: &[u8]) -> Result<Versions, Self::DeserializeError> {
        Self::get_format(self).from_slice(repr)
    }
}
