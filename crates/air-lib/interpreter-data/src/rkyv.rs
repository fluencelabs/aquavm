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

use rkyv::de::deserializers::SharedDeserializeMap;
use rkyv::de::deserializers::SharedDeserializeMapError;
use rkyv::ser::serializers::AlignedSerializer;
use rkyv::ser::serializers::AllocScratch;
use rkyv::ser::serializers::AllocScratchError;
use rkyv::ser::serializers::CompositeSerializer;
use rkyv::ser::serializers::CompositeSerializerError;
use rkyv::ser::serializers::FallbackScratch;
use rkyv::ser::serializers::HeapScratch;
use rkyv::ser::serializers::SharedSerializeMap;
use rkyv::ser::serializers::SharedSerializeMapError;
use rkyv::ser::serializers::WriteSerializer;
use rkyv::ser::Serializer;
use rkyv::validation::validators::DefaultValidator;
use rkyv::AlignedVec;

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
