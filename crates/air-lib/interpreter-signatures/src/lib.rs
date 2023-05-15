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

use air_interpreter_cid::CID;
use fluence_keypair::error::SigningError;
use fluence_keypair::KeyPair;
use serde::{Deserialize, Serialize};

use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

/// An opaque serializable representation of public key.
///
/// It can be string or binary, you shouldn't care about it unless you change serialization format.
// surrent implementation uses string as it is used as a key in a JSON map
#[derive(Debug, Hash, Clone, Eq, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PublicKey(Box<str>);

impl From<fluence_keypair::PublicKey> for PublicKey {
    fn from(value: fluence_keypair::PublicKey) -> Self {
        Self(bs58::encode(&value.to_vec()).into_string().into())
    }
}

/// An opaque serializable representation of signature key.
///
/// It can be string or binary, you shouldn't care about it unless you change serialization format.
// surrent implementation uses string as more compact in JSON representation than number array
#[derive(Debug, Hash, Clone, Eq, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Signature(Box<str>);

impl Signature {
    fn new(signature: fluence_keypair::Signature) -> Self {
        signature.into()
    }
}

impl From<fluence_keypair::Signature> for Signature {
    fn from(value: fluence_keypair::Signature) -> Self {
        Self(bs58::encode(value.to_vec()).into_string().into())
    }
}

#[derive(Debug, Default)]
pub struct SignatureTracker {
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
