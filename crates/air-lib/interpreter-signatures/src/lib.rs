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

pub use fluence_keypair::KeyFormat;

use borsh::BorshSerialize;
use fluence_keypair::error::DecodingError;
use fluence_keypair::error::SigningError;
use serde::{Deserialize, Serialize};

use std::hash::Hash;

#[derive(Debug, thiserror::Error)]
pub enum KeyError {
    #[error("signature algorithm {0:?} not whitelisted")]
    AlgorithmNotWhitelisted(fluence_keypair::KeyFormat),
    #[error("invalid key data: {0}")]
    InvalidKeyData(#[from] DecodingError),
}

#[derive(Debug, thiserror::Error)]
pub enum VerificationError {
    #[error("incorrect key: {0}")]
    InvalidKey(DecodingError),
    #[error("incorrect signature: {0}")]
    InvalidSignature(DecodingError),
    #[error(transparent)]
    Verification(#[from] fluence_keypair::error::VerificationError),
}

/// An opaque serializable representation of a public key.
///
/// It can be a string or a binary, you shouldn't care about it unless you change serialization format.
// surrent implementation serializes to string as it is used as a key in a JSON map
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
#[cfg_attr(
    feature = "rkyv",
    derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(PartialEq, Eq, Hash)))]
#[cfg_attr(feature = "rkyv", archive(check_bytes))]
pub struct PublicKey(
    #[serde(
        deserialize_with = "sede::b58_to_public_key",
        serialize_with = "sede::public_key_to_b58"
    )]
    Box<[u8]>,
);

impl PublicKey {
    pub fn new(inner: fluence_keypair::PublicKey) -> Self {
        Self(inner.encode().into())
    }

    pub fn verify<T: BorshSerialize + ?Sized>(
        &self,
        value: &T,
        salt: &str,
        signature: &Signature,
    ) -> Result<(), VerificationError> {
        let pk =
            fluence_keypair::PublicKey::decode(&self.0).map_err(VerificationError::InvalidKey)?;
        let signature = fluence_keypair::Signature::decode(signature.0.to_vec())
            .map_err(VerificationError::InvalidSignature)?;

        let serialized_value = SaltedData::new(&value, salt).serialize();
        Ok(pk.verify(&serialized_value, &signature)?)
    }

    pub fn to_peer_id(&self) -> Result<String, KeyError> {
        // TODO cache the public key, or verify key format in Rkyv verification/deserialization
        let pk = fluence_keypair::PublicKey::decode(&self.0)?;
        Ok(pk.to_peer_id().to_string())
    }

    pub fn validate(&self) -> Result<(), KeyError> {
        let pk = fluence_keypair::PublicKey::decode(&self.0)?;
        let key_format = pk.get_key_format();
        validate_with_key_format((), key_format)
    }
}

impl ToString for PublicKey {
    fn to_string(&self) -> String {
        bs58::encode(self.0.as_ref()).into_string()
    }
}

#[derive(Clone)]
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
        PublicKey::new(self.0.public())
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

    pub fn into_inner(self) -> fluence_keypair::KeyPair {
        self.0
    }

    pub fn as_inner(&self) -> &fluence_keypair::KeyPair {
        &self.0
    }
}

impl TryFrom<fluence_keypair::KeyPair> for KeyPair {
    type Error = KeyError;

    fn try_from(value: fluence_keypair::KeyPair) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<KeyPair> for fluence_keypair::KeyPair {
    fn from(value: KeyPair) -> Self {
        value.0
    }
}

pub(crate) fn validate_with_key_format<V>(inner: V, key_format: KeyFormat) -> Result<V, KeyError> {
    // this allow is needed in order to support old versions of the fluence_keypair
    // repos which is used to build it for RISC-0
    #[allow(unreachable_patterns)]
    match key_format {
        fluence_keypair::KeyFormat::Ed25519 => Ok(inner),
        _ => Err(KeyError::AlgorithmNotWhitelisted(key_format)),
    }
}

/// An opaque serializable representation of signature key.
///
/// It can be string or binary, you shouldn't care about it unless you change serialization format.
// surrent implementation serializes string as more compact in JSON representation than number array
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
#[cfg_attr(
    feature = "rkyv",
    derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive(check_bytes))]
pub struct Signature(
    #[serde(
        deserialize_with = "sede::b58_to_signature",
        serialize_with = "sede::signature_to_b58"
    )]
    Box<[u8]>,
);

impl Signature {
    fn new(signature: fluence_keypair::Signature) -> Self {
        Self(signature.encode().into())
    }
}

impl From<fluence_keypair::Signature> for Signature {
    fn from(value: fluence_keypair::Signature) -> Self {
        Self(value.encode().into())
    }
}

#[derive(BorshSerialize)]
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
