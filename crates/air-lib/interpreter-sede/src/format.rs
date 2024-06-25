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
    #[allow(dead_code)]
    fn borrow_from_slice(&self, slice: &'data [u8]) -> Result<Value, Self::DeserializationError>;
}

#[allow(dead_code)]
pub trait ArchivedFormat<Value>: Format<Value> {
    type Archived;
    type ValidationError;

    fn archived_from_slice<'data>(
        &self,
        slice: &'data [u8],
    ) -> Result<&'data Self::Archived, Self::ValidationError>;
}
