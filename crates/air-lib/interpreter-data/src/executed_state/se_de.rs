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

use super::*;
use serde::de::SeqAccess;
use serde::de::Visitor;
use serde::ser::SerializeSeq;
use serde::Deserializer;
use serde::Serializer;
use std::fmt;

pub mod par_serializer {
    use super::*;

    pub fn serialize<S>(value: &ParResult, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&value.left_size)?;
        seq.serialize_element(&value.right_size)?;
        seq.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<ParResult, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ParVisitor;
        impl<'de> Visitor<'de> for ParVisitor {
            type Value = ParResult;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("[left_size, right_size]")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let left_size = seq.next_element::<u32>()?;
                let right_size = seq.next_element::<u32>()?;

                let (left_size, right_size) = match (left_size, right_size) {
                    (Some(left_size), Some(right_size)) => (left_size, right_size),
                    _ => return Err(serde::de::Error::custom(
                        "failed to deserialize ParResult, not enough elements in serialized array",
                    )),
                };
                let par_result = ParResult::new(left_size, right_size);

                Ok(par_result)
            }
        }

        deserializer.deserialize_seq(ParVisitor {})
    }
}
