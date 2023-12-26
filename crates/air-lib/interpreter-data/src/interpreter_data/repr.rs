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

use crate::InterpreterDataEnv;
use crate::Versions;

use air_interpreter_sede::Format;
use air_interpreter_sede::FromSerialized;
use air_interpreter_sede::JsonFormat;
use air_interpreter_sede::Representation;
use air_interpreter_sede::ToSerialized;
use air_interpreter_sede::ToWriter;

#[derive(Default, Debug)]
pub struct InterpreterDataEnvRepr;

pub type InterpreterDataEnvFormat = JsonFormat;

impl Representation for InterpreterDataEnvRepr {
    type SerializeError =
        <InterpreterDataEnvFormat as Format<InterpreterDataEnv>>::SerializationError;
    type DeserializeError =
        <InterpreterDataEnvFormat as Format<InterpreterDataEnv>>::DeserializationError;
    type WriteError = <InterpreterDataEnvFormat as Format<InterpreterDataEnv>>::WriteError;
    type Format = InterpreterDataEnvFormat;
    type SerializedValue = Vec<u8>; // TODO a typed wrapper

    fn get_format(&self) -> InterpreterDataEnvFormat {
        InterpreterDataEnvFormat::default()
    }
}

impl ToSerialized<InterpreterDataEnv> for InterpreterDataEnvRepr {
    #[inline]
    fn serialize(&self, value: &InterpreterDataEnv) -> Result<Vec<u8>, Self::SerializeError> {
        Self::get_format(self).to_vec(value)
    }
}

impl FromSerialized<InterpreterDataEnv> for InterpreterDataEnvRepr {
    #[inline]
    fn deserialize(&self, repr: &[u8]) -> Result<InterpreterDataEnv, Self::DeserializeError> {
        Self::get_format(self).from_slice(repr)
    }
}

impl ToWriter<InterpreterDataEnv> for InterpreterDataEnvRepr {
    #[inline]
    fn to_writer<W: std::io::Write>(
        &self,
        value: &InterpreterDataEnv,
        writer: &mut W,
    ) -> Result<(), Self::WriteError> {
        Self::get_format(self).to_writer(value, writer)
    }
}

impl FromSerialized<Versions> for InterpreterDataEnvRepr {
    #[inline]
    fn deserialize(&self, repr: &[u8]) -> Result<Versions, Self::DeserializeError> {
        Self::get_format(self).from_slice(repr)
    }
}
