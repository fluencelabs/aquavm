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

use crate::execution_step::ValueAggregate;
use crate::JValue;

use air_interpreter_value::JsonString;
use std::fmt::Display;
use std::fmt::Formatter;

pub(crate) static KEY_FIELD_NAME: &str = "key";

// TODO refactor the keys so that integer and string
// value domains overlap would become impossible or less harmful.
#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub(crate) enum StreamMapKey {
    Str(JsonString),
    U64(u64),
    I64(i64),
}

impl StreamMapKey {
    pub fn from_value(value: JValue) -> Option<Self> {
        match value {
            JValue::String(s) => Some(StreamMapKey::Str(s)),
            JValue::Number(n) if n.is_i64() => Some(StreamMapKey::I64(n.as_i64().unwrap())),
            JValue::Number(n) if n.is_u64() => Some(StreamMapKey::U64(n.as_u64().unwrap())),
            _ => None,
        }
    }

    pub fn from_value_ref(value: &JValue) -> Option<Self> {
        match value {
            JValue::String(s) => Some(StreamMapKey::Str(s.clone())),
            JValue::Number(n) if n.is_i64() => Some(StreamMapKey::I64(n.as_i64().unwrap())),
            JValue::Number(n) if n.is_u64() => Some(StreamMapKey::U64(n.as_u64().unwrap())),
            _ => None,
        }
    }

    pub(crate) fn from_kvpair_owned(value: &ValueAggregate) -> Option<Self> {
        let object = value.get_result().as_object()?;
        let key = object.get(KEY_FIELD_NAME)?.clone();
        StreamMapKey::from_value(key)
    }

    pub(crate) fn to_key(&self) -> JsonString {
        match self {
            StreamMapKey::Str(s) => s.clone(),
            StreamMapKey::U64(n) => format!("{n}").into(),
            StreamMapKey::I64(n) => format!("{n}").into(),
        }
    }
}

impl From<i64> for StreamMapKey {
    fn from(value: i64) -> Self {
        StreamMapKey::I64(value)
    }
}

impl From<u64> for StreamMapKey {
    fn from(value: u64) -> Self {
        StreamMapKey::U64(value)
    }
}

// TODO unify all types.
// This conversion is used to cast from numeric lambda accessor that leverages u32
// however larpop parser grammar uses i64 for numeric keys inserting into a stream map.
impl From<u32> for StreamMapKey {
    fn from(value: u32) -> Self {
        StreamMapKey::I64(value.into())
    }
}

impl From<&str> for StreamMapKey {
    fn from(value: &str) -> Self {
        StreamMapKey::Str(value.into())
    }
}

impl From<JsonString> for StreamMapKey {
    fn from(value: JsonString) -> Self {
        StreamMapKey::Str(value)
    }
}

impl From<StreamMapKey> for JValue {
    fn from(value: StreamMapKey) -> Self {
        match value {
            StreamMapKey::Str(s) => JValue::string(s),
            StreamMapKey::U64(n) => n.into(),
            StreamMapKey::I64(n) => n.into(),
        }
    }
}

// This trait impl proposefully prints numbers the same way as strings
// to use it in map-to-scalar cast.
impl Display for StreamMapKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StreamMapKey::Str(s) => write!(f, "{}", s),
            StreamMapKey::U64(n) => write!(f, "{}", n),
            StreamMapKey::I64(n) => write!(f, "{}", n),
        }
    }
}
