/*
 * Copyright 2024 Fluence Labs Limited
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

/*
 * This file is based on serde_json crate by Erick Tryzelaar and David Tolnay
 * licensed under conditions of MIT License and Apache License, Version 2.0.
 */

use crate::value::JValue;
use core::result;
use serde::ser::Serialize;

impl Serialize for JValue {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        match self {
            JValue::Null => serializer.serialize_unit(),
            JValue::Bool(b) => serializer.serialize_bool(*b),
            JValue::Number(n) => n.serialize(serializer),
            JValue::String(s) => serializer.serialize_str(s),
            JValue::Array(v) => v.serialize(serializer),
            JValue::Object(m) => {
                use serde::ser::SerializeMap;
                let mut map = tri!(serializer.serialize_map(Some(m.len())));
                for (k, v) in &**m {
                    tri!(map.serialize_entry(k, v));
                }
                map.end()
            }
        }
    }
}
