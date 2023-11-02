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

use crate::InterpreterData;
use crate::Versions;

use air_interpreter_sede::Format;
use air_interpreter_sede::FromSerialized;
use air_interpreter_sede::RmpSerdeFormat;
use air_interpreter_sede::ToSerialized;
use air_interpreter_sede::ToWriter;

#[derive(Default)]
pub struct InterpreterDataRepr;

// type InterpreterDataFormat = SerdeJsonFormat;
type InterpreterDataFormat = RmpSerdeFormat;

impl InterpreterDataRepr {
    pub fn get_format<V: serde::Serialize + serde::de::DeserializeOwned>() -> InterpreterDataFormat
    {
        InterpreterDataFormat::default()
    }
}

impl ToSerialized<InterpreterData> for InterpreterDataRepr {
    type Error = <InterpreterDataFormat as Format<InterpreterData>>::SerializationError;

    #[inline]
    fn serialize(&self, value: &InterpreterData) -> Result<Vec<u8>, Self::Error> {
        InterpreterDataRepr::get_format::<InterpreterData>().to_vec(value)
    }
}

impl FromSerialized<InterpreterData> for InterpreterDataRepr {
    type Error = <InterpreterDataFormat as Format<InterpreterData>>::DeserializationError;

    #[inline]
    fn deserialize(&self, repr: &[u8]) -> Result<InterpreterData, Self::Error> {
        InterpreterDataRepr::get_format::<InterpreterData>().from_slice(repr)
    }
}

impl ToWriter<InterpreterData> for InterpreterDataRepr {
    type Error = <InterpreterDataFormat as Format<InterpreterData>>::WriteError;

    #[inline]
    fn to_writer<W: std::io::Write>(
        &self,
        value: &InterpreterData,
        writer: &mut W,
    ) -> Result<(), Self::Error> {
        InterpreterDataRepr::get_format::<InterpreterData>().to_writer(value, writer)
    }
}

impl FromSerialized<Versions> for InterpreterDataRepr {
    type Error = <InterpreterDataFormat as Format<InterpreterData>>::DeserializationError;

    #[inline]
    fn deserialize(&self, repr: &[u8]) -> Result<Versions, Self::Error> {
        InterpreterDataRepr::get_format::<InterpreterData>().from_slice(repr)
    }
}
