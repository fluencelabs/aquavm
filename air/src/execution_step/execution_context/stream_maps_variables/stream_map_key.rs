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

use crate::execution_step::execution_context::stream_maps_variables::errors::unsupported_map_key_type;
use crate::execution_step::ValueAggregate;
use crate::CatchableError;
use crate::ExecutionError;
use crate::JValue;

use serde::Serialize;
use std::borrow::Cow;

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) enum StreamMapKey<'i> {
    Str(Cow<'i, str>),
    U64(u64),
    I64(i64),
}

impl<'i> StreamMapKey<'i> {
    pub(crate) fn from_value(value: JValue, map_name: &str) -> Result<Self, ExecutionError> {
        match value {
            JValue::String(s) => Ok(StreamMapKey::Str(Cow::Owned(s))),
            JValue::Number(n) if n.is_i64() => Ok(StreamMapKey::I64(n.as_i64().unwrap())),
            JValue::Number(n) if n.is_u64() => Ok(StreamMapKey::U64(n.as_u64().unwrap())),
            _ => Err(CatchableError::StreamMapError(unsupported_map_key_type(map_name)).into()),
        }
    }

    pub(crate) fn from_kvpair(value: &ValueAggregate) -> Option<Self> {
        let object = value.get_result().as_object()?;
        let key = object.get("key")?.to_owned();
        match key {
            JValue::String(s) => Some(StreamMapKey::Str(Cow::Owned(s))),
            JValue::Number(n) if n.is_i64() => Some(StreamMapKey::I64(n.as_i64().unwrap())),
            JValue::Number(n) if n.is_u64() => Some(StreamMapKey::U64(n.as_u64().unwrap())),
            _ => None,
        }
    }
}

impl From<i64> for StreamMapKey<'_> {
    fn from(value: i64) -> Self {
        StreamMapKey::I64(value)
    }
}

impl From<u64> for StreamMapKey<'_> {
    fn from(value: u64) -> Self {
        StreamMapKey::U64(value)
    }
}

impl<'i> From<&'i str> for StreamMapKey<'i> {
    fn from(value: &'i str) -> Self {
        StreamMapKey::Str(Cow::Borrowed(value))
    }
}

impl<'i> Serialize for StreamMapKey<'i> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            StreamMapKey::Str(s) => serializer.serialize_str(s),
            StreamMapKey::U64(n) => serializer.serialize_u64(*n),
            StreamMapKey::I64(n) => serializer.serialize_i64(*n),
        }
    }
}
