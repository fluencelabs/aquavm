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

use crate::JValue;

use thiserror::Error as ThisError;

/// Describes errors related to applying lambdas to values.
#[derive(Debug, Clone, ThisError)]
pub enum LambdaError {
    #[error("lambda is applied to a stream that have only '{stream_size}' elements, but '{idx}' requested")]
    CanonStreamNotHaveEnoughValues { stream_size: usize, idx: usize },

    /// An error occurred while trying to apply lambda to an empty stream.
    #[error("lambda is applied to an empty stream")]
    EmptyStream,

    #[error("field accessor (with field name = '{field_name}') can't be applied to a stream")]
    FieldAccessorAppliedToStream { field_name: String },

    #[error("value '{value}' is not an array-type to match array accessor with idx = '{idx}'")]
    ArrayAccessorNotMatchValue { value: JValue, idx: u32 },

    #[error("value '{value}' does not contain element for idx = '{idx}'")]
    ValueNotContainSuchArrayIdx { value: JValue, idx: u32 },

    #[error("value '{value}' does not contain element with field name = '{field_name}'")]
    ValueNotContainSuchField { value: JValue, field_name: String },

    #[error("value '{value}' is not an map-type to match field accessor with field_name = '{field_name}'")]
    FieldAccessorNotMatchValue { value: JValue, field_name: String },

    #[error("index accessor `{accessor} can't be converted to u32`")]
    IndexAccessNotU32 { accessor: serde_json::Number },

    #[error("scalar accessor `{scalar_accessor}` should has number or string type")]
    ScalarAccessorHasInvalidType { scalar_accessor: JValue },

    #[error("stream accessor `{scalar_accessor}` should has number (u32) type")]
    StreamAccessorHasInvalidType { scalar_accessor: JValue },

    #[error("canon stream map accessor `{map_accessor}` should be either string or number")]
    CanonStreamMapAccessorHasInvalidType { map_accessor: JValue },

    #[error("canon stream map accessor must not be iterable")]
    CanonStreamMapAccessorMustNotBeIterable,
}
