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

use std::io::Write;

use crate::Format;

use unsigned_varint::decode as varint_decode;
use unsigned_varint::encode as varint_encode;

pub type SerializationCodec = u32;

const ENCODING_BUFFER_CAPACITY: usize = 1024;

#[derive(thiserror::Error, Debug)]
pub enum DecodeError<FormatError> {
    #[error(transparent)]
    Format(FormatError),
    #[error("unsupported multiformat codec: {0}")]
    Codec(SerializationCodec),
    #[error("failed to parse multiformat: {0}")]
    VarInt(#[from] varint_decode::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum EncodeError<FormatError> {
    #[error(transparent)]
    Format(FormatError),
    #[error("failed to write: {0}")]
    Io(#[from] std::io::Error),
}

pub fn parse_multiformat_bytes(
    data: &[u8],
) -> Result<(SerializationCodec, &[u8]), varint_decode::Error> {
    varint_decode::u32(data)
}

pub fn encode_multiformat<Value, Fmt: Format<Value>>(
    data: &Value,
    codec: SerializationCodec,
    format: &Fmt,
) -> Result<Vec<u8>, EncodeError<<Fmt as Format<Value>>::WriteError>> {
    let mut output = Vec::with_capacity(ENCODING_BUFFER_CAPACITY);

    write_multiformat(data, codec, format, &mut output)?;

    Ok(output)
}

pub fn write_multiformat<Value, Fmt: Format<Value>, W: Write>(
    data: &Value,
    codec: SerializationCodec,
    format: &Fmt,
    output: &mut W,
) -> Result<(), EncodeError<<Fmt as Format<Value>>::WriteError>> {
    // looks weird, but that's how the API is
    let mut buf = varint_encode::u32_buffer();
    let codec_bytes = varint_encode::u32(codec, &mut buf);
    output.write_all(codec_bytes)?;
    format
        .to_writer(data, output)
        .map_err(EncodeError::Format)?;
    Ok(())
}

pub fn decode_multiformat<Value, Fmt: Format<Value>>(
    multiformat_data: &[u8],
    expected_codec: SerializationCodec,
    format: &Fmt,
) -> Result<Value, DecodeError<<Fmt as Format<Value>>::DeserializationError>> {
    let (data_codec, data) = parse_multiformat_bytes(multiformat_data)?;

    if data_codec != expected_codec {
        // TODO we may be more permissive, having kind of registry for the possible incoming formats, akin to
        // CID algorithms; but it may be *really* tricky to organize it
        return Err(DecodeError::Codec(data_codec));
    }

    format.from_slice(data).map_err(DecodeError::Format)
}
