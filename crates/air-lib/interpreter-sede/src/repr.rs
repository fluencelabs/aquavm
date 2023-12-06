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

/// A formatter intended for particular type, a base type that defines generic behavior
/// used by particular implementations.
pub trait Representation {
    type SerializeError;
    type DeserializeError;
    type WriteError;
    type Format;
    type SerializedValue: std::ops::Deref<Target = [u8]>;

    fn get_format(&self) -> Self::Format;
}

/// Serialization trait restricted to for particular type.
pub trait ToSerialized<Value>: Representation {
    fn serialize(&self, value: &Value) -> Result<Self::SerializedValue, Self::SerializeError>;
}

/// Owned deserialization trait restricted to for particular type.
pub trait FromSerialized<Value>: Representation {
    fn deserialize(&self, repr: &[u8]) -> Result<Value, Self::DeserializeError>;
}

/// Borrow deserialization trait restricted to for particular type.
pub trait FromSerialiedBorrow<'data, Value: 'data>: Representation {
    fn deserialize_borrow(&self, repr: &'data [u8]) -> Result<Value, Self::DeserializeError>;
}

/// Writing deserialization trait restricted to for particular type.
pub trait ToWriter<Value>: Representation {
    fn to_writer<W: std::io::Write>(
        &self,
        value: &Value,
        writer: &mut W,
    ) -> Result<(), Self::WriteError>;
}

#[macro_export]
macro_rules! define_simple_representation {
    ($repr_type:ident, $value_type:ty, $format_type:ty, $serialized_value:ty) => {
        #[derive(Default)]
        pub struct $repr_type;

        impl $crate::Representation for $repr_type {
            type SerializeError = <$format_type as $crate::Format<$value_type>>::SerializationError;

            type DeserializeError =
                <$format_type as $crate::Format<$value_type>>::DeserializationError;

            type WriteError = <$format_type as $crate::Format<$value_type>>::WriteError;

            type Format = $format_type;

            type SerializedValue = $serialized_value;

            #[inline]
            fn get_format(&self) -> Self::Format {
                <$format_type>::default()
            }
        }

        impl $crate::ToSerialized<$value_type> for $repr_type {
            #[inline]
            fn serialize(
                &self,
                value: &$value_type,
            ) -> Result<$serialized_value, Self::SerializeError> {
                use $crate::Format;
                use $crate::Representation;
                Self::get_format(self).to_vec(value).map(Into::into)
            }
        }

        impl $crate::FromSerialized<$value_type> for $repr_type {
            #[inline]
            fn deserialize(&self, repr: &[u8]) -> Result<$value_type, Self::DeserializeError> {
                use $crate::Format;
                use $crate::Representation;
                Self::get_format(self).from_slice(repr)
            }
        }

        impl $crate::ToWriter<$value_type> for $repr_type {
            #[inline]
            fn to_writer<W: std::io::Write>(
                &self,
                value: &$value_type,
                writer: &mut W,
            ) -> Result<(), Self::WriteError> {
                use $crate::Format;
                use $crate::Representation;
                Self::get_format(self).to_writer(value, writer)
            }
        }
    };
}
