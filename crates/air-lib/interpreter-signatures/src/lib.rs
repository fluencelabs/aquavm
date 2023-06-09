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

use air_interpreter_cid::CID;
use fluence_keypair::error::SigningError;
use fluence_keypair::KeyPair;
use serde::{Deserialize, Serialize};

use std::borrow::Borrow;
use std::collections::HashMap;
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

impl PublicKey {
    pub fn verify<T: Serialize + ?Sized>(
        &self,
        value: &T,
        signature: &fluence_keypair::Signature,
    ) -> Result<(), fluence_keypair::error::VerificationError> {
        let pk = &**self;

        let serialized_value =
            serde_json::to_vec(value).expect("default serialization shouldn't fail");

        pk.verify(&serialized_value, signature)
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

impl From<fluence_keypair::PublicKey> for PublicKey {
    fn from(value: fluence_keypair::PublicKey) -> Self {
        Self(value)
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
