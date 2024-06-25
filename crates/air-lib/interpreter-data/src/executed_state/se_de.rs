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
