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

// #![forbid(unsafe_code)]
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

use air_interpreter_cid::CID;
use fluence_keypair::error::SigningError;
use fluence_keypair::KeyPair;
use serde::{Deserialize, Serialize};

use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

/// An opaque serializable representation of a public key.
///
/// It can be a string or a binary, you shouldn't care about it unless you change serialization format.
// surrent implementation serializes to string as it is used as a key in a JSON map
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive(check_bytes))]
#[cfg_attr(feature = "rkyv", archive_attr(derive(Debug, Hash, Eq, PartialEq)))]
pub struct PublicKey(
    #[serde(deserialize_with = "sede::from_b58", serialize_with = "sede::to_b58")]
    #[cfg_attr(feature = "rkyv", with(sede::B58PublicKey))]
    Box<[u8]>,
);

impl PublicKey {
    pub fn verify<T: Serialize + ?Sized>(
        &self,
        value: &T,
        signature: &Signature,
    ) -> Result<(), fluence_keypair::error::VerificationError> {
        let pk = fluence_keypair::PublicKey::decode(&self.0).expect("TODO error variant");
        let signature = fluence_keypair::Signature::decode(signature.0.clone().into())
            .expect("TODO error variant");

        let serialized_value =
            serde_json::to_vec(value).expect("default serialization shouldn't fail");

        pk.verify(&serialized_value, &signature)
    }
}

impl From<fluence_keypair::PublicKey> for PublicKey {
    fn from(value: fluence_keypair::PublicKey) -> Self {
        Self(value.encode().into())
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
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive(check_bytes))]
#[cfg_attr(feature = "rkyv", archive_attr(derive(Debug)))]
pub struct Signature(
    #[serde(deserialize_with = "sede::from_b58", serialize_with = "sede::to_b58")]
    #[cfg_attr(feature = "rkyv", with(sede::B58Signature))]
    Box<[u8]>,
);

impl Signature {
    fn new(signature: fluence_keypair::Signature) -> Self {
        Self(signature.encode().into())
    }
}

impl From<fluence_keypair::Signature> for Signature {
    fn from(value: fluence_keypair::Signature) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Default)]
pub struct SignatureTracker {
    // from peer id to CID strings
    peer_to_cids: HashMap<Box<str>, Vec<Box<str>>>,
}

impl SignatureTracker {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn register<T>(&mut self, peer_id: impl Into<Box<str>>, cid: &CID<T>) {
        self.peer_to_cids
            .entry(peer_id.into())
            .or_default()
            .push(cid.clone().into_inner().into());
    }

    pub fn into_signature(
        &mut self,
        peer_id: &str,
        signer: &KeyPair,
    ) -> Result<Signature, SigningError> {
        let mut cids = self.peer_to_cids.get(peer_id).cloned().unwrap_or_default();
        cids.sort_unstable();

        // TODO make pluggable serialization
        // TODO it will be useful for CID too
        // TODO please note that using serde::Serializer is not enough
        let serialized_cids =
            serde_json::to_string(&cids).expect("default serialization shouldn't fail");

        signer.sign(serialized_cids.as_bytes()).map(Signature::new)
    }
}

/// A dictionary-like structure that stores peer public keys and their particle data signatures.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive(check_bytes))]
// #[cfg_attr(feature = "rkyv", archive_attr(derive(Debug)))]
pub struct SignatureStore<Key: Hash + Eq = PublicKey, Sign = Signature>(HashMap<Key, Sign>);

impl<Key: Hash + Eq, Sign> SignatureStore<Key, Sign> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get<Q>(&self, peer_pk: &Q) -> Option<&Sign>
    where
        Key: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.0.get(peer_pk)
    }

    pub fn put(&mut self, peer_pk: Key, signature: Sign) {
        self.0.insert(peer_pk, signature);
    }

    pub fn merge(prev: Self, _current: Self) -> Self {
        // TODO STUB
        prev
    }
}

impl<Key: Hash + Eq, Sign> Default for SignatureStore<Key, Sign> {
    fn default() -> Self {
        Self(Default::default())
    }
}
