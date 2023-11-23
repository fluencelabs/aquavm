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

use std::fmt::Debug;

pub trait Format<Value> {
    type SerializationError: Debug;
    type DeserializationError: Debug;
    type WriteError: Debug;

    fn to_vec(&self, val: &Value) -> Result<Vec<u8>, Self::SerializationError>;

    // todo owned_from_slice
    #[allow(clippy::wrong_self_convention)]
    fn from_slice(&self, slice: &[u8]) -> Result<Value, Self::DeserializationError>;

    fn to_writer<W: std::io::Write>(
        &self,
        value: &Value,
        write: &mut W,
    ) -> Result<(), Self::WriteError>;
}

pub trait BorrowFormat<'data, Value: 'data>: Format<Value> {
    fn borrow_from_slice(&self, slice: &'data [u8]) -> Result<Value, Self::DeserializationError>;
}

pub trait ArchivedFormat<Value>: Format<Value> {
    type Archived;
    type ValidationError;

    fn archived_from_slice<'data>(
        &self,
        slice: &'data [u8],
    ) -> Result<&'data Self::Archived, Self::ValidationError>;
}
