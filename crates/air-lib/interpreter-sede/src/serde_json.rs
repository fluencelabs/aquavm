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

use crate::multiformat::SerializationCodec;
use crate::Format;

// https://github.com/multiformats/multicodec/blob/master/table.csv
const MULTIFORMAT_JSON: SerializationCodec = 0x0200;

// TODO a human-readability flag?
#[derive(Copy, Clone, Default)]
pub struct SerdeJsonFormat;

impl<Value> Format<Value> for SerdeJsonFormat
where
    Value: serde::Serialize + serde::de::DeserializeOwned,
{
    type SerializationError = serde_json::Error;
    type DeserializationError = serde_json::Error;
    type WriteError = serde_json::Error;

    #[inline]
    fn to_vec(&self, value: &Value) -> Result<Vec<u8>, Self::SerializationError> {
        serde_json::to_vec(value)
    }

    #[inline]
    fn from_slice(&self, slice: &[u8]) -> Result<Value, Self::DeserializationError> {
        serde_json::from_slice(slice)
    }

    #[inline]
    fn to_writer<W: std::io::Write>(
        &self,
        value: &Value,
        write: &mut W,
    ) -> Result<(), Self::WriteError> {
        serde_json::to_writer(write, value)
    }
}

#[derive(Copy, Clone, Default)]
pub struct SerdeJsonMultiformat;

impl<Value> Format<Value> for SerdeJsonMultiformat
where
    Value: serde::Serialize + serde::de::DeserializeOwned,
{
    type SerializationError = crate::multiformat::EncodeError<serde_json::Error>;
    type DeserializationError = crate::multiformat::DecodeError<serde_json::Error>;
    type WriteError = crate::multiformat::EncodeError<serde_json::Error>;

    #[inline]
    fn to_vec(&self, value: &Value) -> Result<Vec<u8>, Self::SerializationError> {
        crate::multiformat::encode_multiformat(value, MULTIFORMAT_JSON, &SerdeJsonFormat)
    }

    #[inline]
    fn from_slice(&self, slice: &[u8]) -> Result<Value, Self::DeserializationError> {
        crate::multiformat::decode_multiformat(slice, MULTIFORMAT_JSON, &SerdeJsonFormat)
    }

    #[inline]
    fn to_writer<W: std::io::Write>(
        &self,
        value: &Value,
        write: &mut W,
    ) -> Result<(), Self::WriteError> {
        crate::multiformat::write_multiformat(value, MULTIFORMAT_JSON, &SerdeJsonFormat, write)
    }
}
