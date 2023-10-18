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

#![forbid(unsafe_code)]
#![warn(rust_2018_idioms)]
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]

mod sede;
mod stores;
mod trackers;

pub use crate::stores::*;
pub use crate::trackers::*;

use fluence_keypair::KeyFormat;

use borsh::BorshSerialize;
use fluence_keypair::error::SigningError;
use serde::{Deserialize, Serialize};

use std::convert::TryFrom;
use std::hash::Hash;
use std::ops::Deref;

/// An opaque serializable representation of a public key.
///
/// It can be a string or a binary, you shouldn't care about it unless you change serialization format.
// surrent implementation serializes to string as it is used as a key in a JSON map
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct PublicKey(
    #[serde(
        deserialize_with = "sede::b58_to_public_key",
        serialize_with = "sede::public_key_to_b58"
    )]
    fluence_keypair::PublicKey,
);

#[derive(thiserror::Error, Debug)]
pub enum KeyError {
    #[error("algorithm {0:?} not allowed")]
    AlgorithmNotAllowed(fluence_keypair::KeyFormat),
    #[error("invalid key data: {0}")]
    InvalidKeyData(#[from] fluence_keypair::error::DecodingError),
}

impl PublicKey {
    pub fn new(inner: fluence_keypair::PublicKey) -> Result<Self, KeyError> {
        // We accept only deterministic algorithms, i.e. that do
        let key_format = inner.get_key_format();
        validate_with_key_format(inner, key_format).map(Self)
    }

    pub fn verify<T: BorshSerialize + ?Sized>(
        &self,
        value: &T,
        salt: &str,
        signature: &fluence_keypair::Signature,
    ) -> Result<(), fluence_keypair::error::VerificationError> {
        let pk = &**self;

        let serialized_value = SaltedData::new(&value, salt).serialize();
        pk.verify(&serialized_value, signature)
    }

    pub fn to_peer_id(&self) -> String {
        self.0.to_peer_id().to_string()
    }
}

impl Deref for PublicKey {
    type Target = fluence_keypair::PublicKey;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Hash for PublicKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_vec().hash(state);
    }
}

impl TryFrom<fluence_keypair::PublicKey> for PublicKey {
    type Error = KeyError;

    fn try_from(value: fluence_keypair::PublicKey) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

pub struct KeyPair(fluence_keypair::KeyPair);

impl KeyPair {
    pub fn new(inner: fluence_keypair::KeyPair) -> Result<Self, KeyError> {
        let key_format = inner.key_format();
        validate_with_key_format((), key_format)?;

        Ok(Self(inner))
    }

    pub fn from_secret_key(secret_key: Vec<u8>, key_format: KeyFormat) -> Result<Self, KeyError> {
        let inner = fluence_keypair::KeyPair::from_secret_key(secret_key, key_format)?;
        Self::new(inner)
    }

    pub fn public(&self) -> PublicKey {
        PublicKey(self.0.public())
    }

    pub fn key_format(&self) -> KeyFormat {
        self.0.key_format()
    }

    pub fn sign(&self, msg: &[u8]) -> Result<Signature, SigningError> {
        self.0.sign(msg).map(Signature::new)
    }

    pub fn secret(&self) -> Vec<u8> {
        self.0.secret().expect("cannot fail on supported formats")
    }

    pub fn as_inner(&self) -> &fluence_keypair::KeyPair {
        &self.0
    }

    #[cfg(feature = "rand")]
    pub fn generate(key_format: KeyFormat) -> Result<Self, KeyError> {
        validate_with_key_format((), key_format)?;

        Ok(Self(fluence_keypair::KeyPair::generate(key_format)))
    }
}

impl TryFrom<fluence_keypair::KeyPair> for KeyPair {
    type Error = KeyError;

    fn try_from(value: fluence_keypair::KeyPair) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

pub(crate) fn validate_with_key_format<V>(inner: V, key_format: KeyFormat) -> Result<V, KeyError> {
    match key_format {
        fluence_keypair::KeyFormat::Ed25519 => Ok(inner),
        _ => Err(KeyError::AlgorithmNotAllowed(key_format)),
    }
}

/// An opaque serializable representation of signature key.
///
/// It can be string or binary, you shouldn't care about it unless you change serialization format.
// surrent implementation serializes string as more compact in JSON representation than number array
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Signature(
    #[serde(
        deserialize_with = "sede::b58_to_signature",
        serialize_with = "sede::signature_to_b58"
    )]
    fluence_keypair::Signature,
);

impl Signature {
    fn new(signature: fluence_keypair::Signature) -> Self {
        Self(signature)
    }
}

impl Deref for Signature {
    type Target = fluence_keypair::Signature;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<fluence_keypair::Signature> for Signature {
    fn from(value: fluence_keypair::Signature) -> Self {
        Self(value)
    }
}

impl From<Signature> for fluence_keypair::Signature {
    fn from(value: Signature) -> Self {
        value.0
    }
}

#[derive(borsh_derive::BorshSerialize)]
pub(crate) struct SaltedData<'ctx, Data: BorshSerialize>(&'ctx Data, &'ctx str);

impl<'ctx, Data: BorshSerialize> SaltedData<'ctx, Data> {
    pub(crate) fn new(data: &'ctx Data, salt: &'ctx str) -> Self {
        Self(data, salt)
    }

    pub(crate) fn serialize(&self) -> Vec<u8> {
        // TODO make pluggable serialization
        // TODO it will be useful for CID too
        // TODO please note that using serde::Serializer is not enough
        borsh::to_vec(&self).expect("borsh serializer shouldn't fail")
    }
}
