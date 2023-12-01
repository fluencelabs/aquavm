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

use crate::Format;
use crate::FromSerialized;
use crate::ToWriter;
use crate::TypedFormat;

use unsigned_varint::decode as varint_decode;
use unsigned_varint::encode as varint_encode;

pub type SerializationCodec = u32;

pub const MIN_PRIVATE_MULTIFORMAT_CODE: SerializationCodec = 0x300000;

const ENCODING_BUFFER_CAPACITY: usize = 1024;

#[derive(thiserror::Error, Debug)]
pub enum DecodeError<FormatError> {
    #[error(transparent)]
    FormatError(FormatError),
    #[error("unsupported multiformat codec: {0}")]
    CodecError(SerializationCodec),
    #[error("failed to parse multiformat: {0}")]
    VarInt(#[from] varint_decode::Error),
}

fn parse_multiformat_bytes(
    data: &[u8],
) -> Result<(SerializationCodec, &[u8]), varint_decode::Error> {
    varint_decode::u32(data)
}

pub fn encode_multiformat<Value, Fmt: TypedFormat + ToWriter<Value>>(
    data: &Value,
    format: impl AsRef<Fmt>,
) -> Result<Vec<u8>, <Fmt as TypedFormat>::WriteError>
where
    <Fmt as TypedFormat>::Format: Format<Value>,
{
    let format = format.as_ref();
    let mut output = Vec::with_capacity(ENCODING_BUFFER_CAPACITY);

    // write codec
    let codec: SerializationCodec = format.get_format().get_codec();
    {
        // looks weird, but that's how the API is
        let mut buf = varint_encode::u32_buffer();
        let codec_bytes = varint_encode::u32(codec, &mut buf);
        output.extend_from_slice(codec_bytes);
    }

    // write data
    format.to_writer(data, &mut output)?;

    Ok(output)
}

pub fn decode_multiformat<Value, Fmt: TypedFormat + FromSerialized<Value>>(
    multiformat_data: &[u8],
    format: impl AsRef<Fmt>,
) -> Result<Value, DecodeError<<Fmt as TypedFormat>::DeserializeError>>
where
    <Fmt as TypedFormat>::Format: Format<Value>,
{
    let format = format.as_ref();
    let (codec, data) = parse_multiformat_bytes(multiformat_data)?;

    if codec != format.get_format().get_codec() {
        // TODO we may be more permissive, having kind of registry for the possible incoming formats, akin to
        // CID algorithms; but it may be *really* tricky to organize it
        return Err(DecodeError::CodecError(codec));
    }

    format.deserialize(data).map_err(DecodeError::FormatError)
}
