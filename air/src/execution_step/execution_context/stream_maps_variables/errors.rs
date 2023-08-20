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

use thiserror::Error as ThisError;

/// Describes errors related to applying lambdas to values.
#[derive(Debug, Clone, ThisError)]
pub enum StreamMapError {
    #[error("unsupported type for {variable_name} map's key")]
    UnsupportedMapKeyType { variable_name: String },
}

/// CanonStreamMap related errors.
#[derive(Debug, Clone, ThisError)]
pub enum CanonStreamMapError {
    #[error("there is no such index in the canon stream map")]
    IndexIsAbsentInTheMap,

    #[error("there is an index with no corresponding value")]
    NonexistentMappingIdx,
}

#[derive(Debug, Clone, ThisError)]
pub enum StreamMapKeyError {
    #[error("the value must be an object with key and value fields")]
    NotAnObject,

    #[error("there must be a \"value\" field in kvpair object")]
    ValueFieldIsAbsent,

    #[error("unsupported kvpair object or map key type")]
    UnsupportedKVPairObjectOrMapKeyType,
}

pub fn unsupported_map_key_type(variable_name: &str) -> StreamMapError {
    StreamMapError::UnsupportedMapKeyType {
        variable_name: variable_name.to_string(),
    }
}
