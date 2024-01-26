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

use crate::PublicKey;
use crate::Signature;

use serde::{Deserialize, Serialize};

use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

/// A dictionary-like structure that stores peer public keys and their particle data signatures.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive(check_bytes))]
pub struct SignatureStore<Key: Hash + Eq = PublicKey, Sign = Signature>(
    #[cfg_attr(feature = "rkyv", with(::rkyv::with::AsVec))] HashMap<Key, Sign>,
);

impl<Key: Hash + Eq, Sign> SignatureStore<Key, Sign> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
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

    pub fn iter(&self) -> <&HashMap<Key, Sign> as IntoIterator>::IntoIter {
        self.0.iter()
    }
}

impl<Key: Hash + Eq, Sign> Default for SignatureStore<Key, Sign> {
    fn default() -> Self {
        Self(Default::default())
    }
}
