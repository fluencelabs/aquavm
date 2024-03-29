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
