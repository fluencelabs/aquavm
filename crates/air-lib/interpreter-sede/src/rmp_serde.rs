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

use crate::format::BorrowFormat;
use crate::multiformat::SerializationCodec;
use crate::Format;

// https://github.com/multiformats/multicodec/blob/master/table.csv
const MULTIFORMAT_MSGPCK: SerializationCodec = 0x0201;

// rmp_serde has config with human-readable representation too, but I'm not sure it worth it
#[derive(Copy, Clone, Default)]
pub struct RmpSerdeFormat;

impl<Value> Format<Value> for RmpSerdeFormat
where
    Value: serde::Serialize + serde::de::DeserializeOwned,
{
    type SerializationError = rmp_serde::encode::Error;
    type DeserializationError = rmp_serde::decode::Error;
    type WriteError = rmp_serde::encode::Error;

    #[inline]
    fn to_vec(&self, val: &Value) -> Result<Vec<u8>, Self::SerializationError> {
        // named representation (i.e. structs are serialized as maps, not tuples) is important
        // for JS interop and data compatibility detection
        rmp_serde::to_vec_named(val)
    }

    #[inline]
    fn from_slice(&self, slice: &[u8]) -> Result<Value, Self::DeserializationError> {
        rmp_serde::from_slice(slice)
    }

    #[inline]
    fn to_writer<W: std::io::Write>(
        &self,
        value: &Value,
        write: &mut W,
    ) -> Result<(), Self::WriteError> {
        // named representation (i.e. structs are serialized as maps, not tuples) is important
        // for JS interop and data compatibility detection
        rmp_serde::encode::write_named(write, value)
    }
}

impl<'data, Value: 'data> BorrowFormat<'data, Value> for RmpSerdeFormat
where
    Value: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    #[inline]
    fn borrow_from_slice(&self, slice: &'data [u8]) -> Result<Value, Self::DeserializationError> {
        rmp_serde::from_slice(slice)
    }
}

#[derive(Copy, Clone, Default)]
pub struct RmpSerdeMultiformat;

impl<Value> Format<Value> for RmpSerdeMultiformat
where
    Value: serde::Serialize + serde::de::DeserializeOwned,
{
    type SerializationError = crate::multiformat::EncodeError<rmp_serde::encode::Error>;
    type DeserializationError = crate::multiformat::DecodeError<rmp_serde::decode::Error>;
    type WriteError = crate::multiformat::EncodeError<rmp_serde::encode::Error>;

    #[inline]
    fn to_vec(&self, value: &Value) -> Result<Vec<u8>, Self::SerializationError> {
        crate::multiformat::encode_multiformat(value, MULTIFORMAT_MSGPCK, &RmpSerdeFormat)
    }

    #[inline]
    fn from_slice(&self, slice: &[u8]) -> Result<Value, Self::DeserializationError> {
        crate::multiformat::decode_multiformat(slice, MULTIFORMAT_MSGPCK, &RmpSerdeFormat)
    }

    #[inline]
    fn to_writer<W: std::io::Write>(
        &self,
        value: &Value,
        write: &mut W,
    ) -> Result<(), Self::WriteError> {
        crate::multiformat::write_multiformat(value, MULTIFORMAT_MSGPCK, &RmpSerdeFormat, write)
    }
}
