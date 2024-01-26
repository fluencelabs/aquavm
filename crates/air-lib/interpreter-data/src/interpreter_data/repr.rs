/*
 * Copyright 2023 Fluence Labs Limited
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
