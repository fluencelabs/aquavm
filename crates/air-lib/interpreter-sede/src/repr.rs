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
pub trait TypedFormat {
    type SerializeError;
    type DeserializeError;
    type WriteError;
    type Format;

    fn get_format(&self) -> Self::Format;
}

/// Serialization trait restricted to for particular type.
pub trait ToSerialized<Value>: TypedFormat {
    fn serialize(&self, value: &Value) -> Result<Vec<u8>, Self::SerializeError>;
}

/// Owned deserialization trait restricted to for particular type.
pub trait FromSerialized<Value>: TypedFormat {
    fn deserialize(&self, repr: &[u8]) -> Result<Value, Self::DeserializeError>;
}

/// Borrow deserialization trait restricted to for particular type.
pub trait FromSerialiedBorrow<'data, Value: 'data>: TypedFormat {
    fn deserialize_borrow(&self, repr: &'data [u8]) -> Result<Value, Self::DeserializeError>;
}

/// Writing deserialization trait restricted to for particular type.
pub trait ToWriter<Value>: TypedFormat {
    fn to_writer<W: std::io::Write>(
        &self,
        value: &Value,
        writer: &mut W,
    ) -> Result<(), Self::WriteError>;
}
