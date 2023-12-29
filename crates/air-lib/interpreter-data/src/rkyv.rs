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

use rkyv::{
    de::deserializers::{SharedDeserializeMap, SharedDeserializeMapError},
    ser::{
        serializers::{
            AlignedSerializer, AllocScratch, AllocScratchError, CompositeSerializer,
            CompositeSerializerError, FallbackScratch, HeapScratch, SharedSerializeMap,
            SharedSerializeMapError, WriteSerializer,
        },
        Serializer,
    },
    validation::validators::DefaultValidator,
    AlignedVec,
};

const DEFAULT_VALIDATION_CAPACITY: usize = 1024;
const DEFAULT_DESERIALIZE_CAPACITY: usize = 1024;

#[derive(Debug, thiserror::Error)]
pub enum RkyvDeserializeError {
    #[error(transparent)]
    // rkyv validation types is tricky and involves lifetime, though final type is 'static
    Validation(Box<dyn std::error::Error>),

    #[error(transparent)]
    Deserialize(SharedDeserializeMapError),
}

pub type RkyvSerializeError =
    CompositeSerializerError<std::convert::Infallible, AllocScratchError, SharedSerializeMapError>;

pub(crate) fn to_vec<Value>(value: &Value) -> Result<Vec<u8>, RkyvSerializeError>
where
    Value: rkyv::Serialize<
        CompositeSerializer<
            AlignedSerializer<AlignedVec>,
            FallbackScratch<HeapScratch<4096>, AllocScratch>,
            SharedSerializeMap,
        >,
    >,
{
    let mut ser = rkyv::ser::serializers::AllocSerializer::<4096>::default();
    ser.serialize_value(value)?;
    Ok(ser.into_serializer().into_inner().to_vec())
}

pub(crate) fn from_aligned_slice<'a, Value>(slice: &'a [u8]) -> Result<Value, RkyvDeserializeError>
where
    Value: rkyv::Archive,
    <Value as rkyv::Archive>::Archived:
        rkyv::CheckBytes<DefaultValidator<'a>> + rkyv::Deserialize<Value, SharedDeserializeMap>,
{
    let mut validator = rkyv::validation::validators::DefaultValidator::with_capacity(
        slice,
        DEFAULT_VALIDATION_CAPACITY,
    );
    let archived_data = rkyv::check_archived_root_with_context::<Value, _>(slice, &mut validator)
        .map_err(|e| RkyvDeserializeError::Validation(Box::new(e)))?;

    let mut shared = SharedDeserializeMap::with_capacity(DEFAULT_DESERIALIZE_CAPACITY);
    rkyv::Deserialize::<Value, _>::deserialize(archived_data, &mut shared)
        .map_err(RkyvDeserializeError::Deserialize)
}

#[allow(dead_code)]
pub(crate) fn to_writer<'a, Value, W: std::io::Write>(
    write: &'a mut W,
    value: &Value,
) -> Result<(), std::io::Error>
where
    Value: rkyv::Serialize<WriteSerializer<&'a mut W>>,
{
    let mut ser = rkyv::ser::serializers::WriteSerializer::new(write);
    ser.serialize_value(value)?;
    Ok(())
}
