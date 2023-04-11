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
    #[error("map {variable_name} key can not be float")]
    FloatMapKeyIsUnsupported { variable_name: String },

    #[error("unsupported type for {variable_name} map's key")]
    UnsupportedMapKeyType { variable_name: String },

    #[error("there must be a key to add a value into {variable_name} map")]
    MapKeyIsAbsent { variable_name: String },
}
