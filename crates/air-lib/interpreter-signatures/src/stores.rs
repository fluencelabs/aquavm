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
