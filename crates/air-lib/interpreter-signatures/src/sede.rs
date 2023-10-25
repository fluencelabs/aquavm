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

pub(crate) fn public_key_to_b58<S: serde::Serializer>(
    key: &fluence_keypair::PublicKey,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&bs58::encode(key.encode()).into_string())
}

pub(crate) fn b58_to_public_key<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<fluence_keypair::PublicKey, D::Error> {
    deserializer.deserialize_str(PublicKeyVisitor)
}

/// Visitor who tries to decode base58-encoded string to a fluence_keypair::PublicKey.
struct PublicKeyVisitor;

impl serde::de::Visitor<'_> for PublicKeyVisitor {
    type Value = fluence_keypair::PublicKey;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a base58-encoded public key string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use serde::de;

        fluence_keypair::PublicKey::from_base58(v)
            .map_err(|_| de::Error::invalid_value(de::Unexpected::Str(v), &self))
    }
}

pub(crate) fn signature_to_b58<S: serde::Serializer>(
    signature: &fluence_keypair::Signature,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&bs58::encode(signature.encode()).into_string())
}

pub(crate) fn b58_to_signature<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<fluence_keypair::Signature, D::Error> {
    deserializer.deserialize_str(SignatureVisitor)
}

/// Visitor who tries to decode base58-encoded string to a fluence_keypair::Signature.
struct SignatureVisitor;

impl serde::de::Visitor<'_> for SignatureVisitor {
    type Value = fluence_keypair::Signature;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("expecting a base58-encoded signature string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use serde::de;

        let sig_bytes = bs58::decode(v)
            .into_vec()
            .map_err(|_| de::Error::invalid_value(de::Unexpected::Str(v), &self))?;
        fluence_keypair::Signature::decode(sig_bytes)
            .map_err(|_| de::Error::invalid_value(de::Unexpected::Str(v), &self))
    }
}
