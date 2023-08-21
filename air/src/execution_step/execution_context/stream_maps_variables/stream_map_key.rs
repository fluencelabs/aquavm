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

use crate::execution_step::ValueAggregate;
use crate::JValue;

use serde::Serialize;
use std::borrow::Cow;
use std::fmt::Display;
use std::fmt::Formatter;

pub(crate) static KEY_FIELD: &str = "key";

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub(crate) enum StreamMapKey<'value> {
    Str(Cow<'value, str>),
    U64(u64),
    I64(i64),
}

impl<'value> StreamMapKey<'value> {
    pub fn from_value(value: JValue) -> Option<Self> {
        match value {
            JValue::String(s) => Some(StreamMapKey::Str(Cow::Owned(s))),
            JValue::Number(n) if n.is_i64() => Some(StreamMapKey::I64(n.as_i64().unwrap())),
            JValue::Number(n) if n.is_u64() => Some(StreamMapKey::U64(n.as_u64().unwrap())),
            _ => None,
        }
    }

    pub fn from_value_ref(value: &'value JValue) -> Option<Self> {
        match value {
            JValue::String(s) => Some(StreamMapKey::Str(Cow::Borrowed(s.as_str()))),
            JValue::Number(n) if n.is_i64() => Some(StreamMapKey::I64(n.as_i64().unwrap())),
            JValue::Number(n) if n.is_u64() => Some(StreamMapKey::U64(n.as_u64().unwrap())),
            _ => None,
        }
    }

    pub(crate) fn from_kvpair(value: ValueAggregate) -> Option<Self> {
        let object = value.get_result().as_object()?;
        let key = (object.get(KEY_FIELD)?).clone();
        StreamMapKey::from_value(key)
    }

    pub(crate) fn from_kvpair_ref(value: &'value ValueAggregate) -> Option<Self> {
        let object = value.get_result().as_object()?;
        let key = object.get(KEY_FIELD)?;
        StreamMapKey::from_value_ref(key)
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

// This conversion is used to cast from numeric lambda accessor that leverages u32
// however larpop parser grammar uses i64 for numeric keys inserting into a stream map.
impl From<u32> for StreamMapKey<'_> {
    fn from(value: u32) -> Self {
        StreamMapKey::I64(value.into())
    }
}

impl<'value> From<&'value str> for StreamMapKey<'value> {
    fn from(value: &'value str) -> Self {
        StreamMapKey::Str(Cow::Borrowed(value))
    }
}

impl From<String> for StreamMapKey<'static> {
    fn from(value: String) -> Self {
        StreamMapKey::Str(Cow::Owned(value))
    }
}

impl<'value> Serialize for StreamMapKey<'value> {
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

// This trait impl proposfully prints numbers the same way as strings
// to use it in map-to-scalar cast.
impl<'value> Display for StreamMapKey<'value> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StreamMapKey::Str(s) => write!(f, "{}", s),
            StreamMapKey::U64(n) => write!(f, "{}", n),
            StreamMapKey::I64(n) => write!(f, "{}", n),
        }
    }
}
