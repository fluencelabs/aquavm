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
    #[error("can not find JValue to produce scalar from")]
    NoDataToProduceScalar,
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
