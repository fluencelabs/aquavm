/*
 * Copyright 2021 Fluence Labs Limited
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

use crate::JValue;

use thiserror::Error as ThisError;

/// Describes errors related to applying lambdas to values.
#[derive(Debug, Clone, ThisError)]
pub enum LambdaError {
    #[error("lambda is applied to a stream that have only '{stream_size}' elements, but '{idx}' requested")]
    StreamNotHaveEnoughValues { stream_size: usize, idx: u32 },

    /// An error occurred while trying to apply lambda to an empty stream.
    #[error("lambda is applied to an empty stream")]
    EmptyStream,

    #[error("field accessor (with field name = '{field_name}') can't be applied to a stream")]
    FieldAccessorAppliedToStream { field_name: String },

    #[error("value '{value}' is not an array-type to match array accessor with idx = '{idx}'")]
    ArrayAccessorNotMatchValue { value: JValue, idx: u32 },

    #[error("value '{value}' does not contain element for idx = '{idx}'")]
    ValueNotContainSuchArrayIdx { value: JValue, idx: u32 },

    #[error("value '{value}' is not an map-type to match field accessor with field_name = '{field_name}'")]
    FieldAccessorNotMatchValue { value: JValue, field_name: String },

    #[error("value '{value}' does not contain element with field name = '{field_name}'")]
    JValueNotContainSuchField { value: JValue, field_name: String },

    #[error("index accessor `{accessor} can't be converted to u32`")]
    IndexAccessNotU32 { accessor: serde_json::Number },

    #[error("scalar accessor `{scalar_accessor}` should has number or string type")]
    ScalarAccessorHasInvalidType { scalar_accessor: JValue },
}
