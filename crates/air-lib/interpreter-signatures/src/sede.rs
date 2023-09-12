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

pub(crate) fn to_b58<S: serde::Serializer>(
    key_data: &[u8],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&bs58::encode(key_data).into_string())
}

pub(crate) fn from_b58<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Box<[u8]>, D::Error> {
    deserializer.deserialize_str(B58Visitor)
}

/// Visitor who tries to decode base58-encoded string to Box<[u8]>.
struct B58Visitor;

impl serde::de::Visitor<'_> for B58Visitor {
    type Value = Box<[u8]>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("expecting a base58-encoded binary data")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use serde::de;

        let key_data: Box<[u8]> = bs58::decode(v)
            .into_vec()
            .map_err(|_| de::Error::invalid_value(de::Unexpected::Str(v), &self))?
            .into();
        Ok(key_data)
    }
}
