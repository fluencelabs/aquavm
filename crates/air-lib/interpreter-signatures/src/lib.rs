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

pub use fluence_keypair::KeyPair;

use serde::{Deserialize, Serialize};

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
        particle_id: &str,
        signature: &fluence_keypair::Signature,
    ) -> Result<(), fluence_keypair::error::VerificationError> {
        let pk = &**self;

        let serialized_value = serde_json::to_vec(&SaltedData::new(&value, particle_id))
            .expect("default serialization shouldn't fail");

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

#[derive(Serialize)]
pub(crate) struct SaltedData<'ctx, Data>(&'ctx Data, &'ctx str);

impl<'ctx, Data: Serialize> SaltedData<'ctx, Data> {
    pub(crate) fn new(data: &'ctx Data, particle_id: &'ctx str) -> Self {
        Self(data, particle_id)
    }
}
