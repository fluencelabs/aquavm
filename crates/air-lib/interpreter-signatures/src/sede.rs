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

pub(crate) fn public_key_to_b58<S: serde::Serializer>(
    key: &[u8],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&bs58::encode(key).into_string())
}

pub(crate) fn b58_to_public_key<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Box<[u8]>, D::Error> {
    deserializer.deserialize_str(PublicKeyVisitor)
}

/// Visitor who tries to decode base58-encoded string to a fluence_keypair::PublicKey.
struct PublicKeyVisitor;

impl serde::de::Visitor<'_> for PublicKeyVisitor {
    type Value = Box<[u8]>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a base58-encoded public key string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use serde::de;

        bs58::decode(v)
            .into_vec()
            .map(Into::into)
            .map_err(|_| de::Error::invalid_value(de::Unexpected::Str(v), &self))
    }
}

pub(crate) fn signature_to_b58<S: serde::Serializer>(
    signature: &[u8],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&bs58::encode(signature).into_string())
}

pub(crate) fn b58_to_signature<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Box<[u8]>, D::Error> {
    deserializer.deserialize_str(SignatureVisitor)
}

/// Visitor who tries to decode base58-encoded string to a fluence_keypair::Signature.
struct SignatureVisitor;

impl serde::de::Visitor<'_> for SignatureVisitor {
    type Value = Box<[u8]>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("expecting a base58-encoded signature string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use serde::de;

        bs58::decode(v)
            .into_vec()
            .map(Into::into)
            .map_err(|_| de::Error::invalid_value(de::Unexpected::Str(v), &self))
    }
}
