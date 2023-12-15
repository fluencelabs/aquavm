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
