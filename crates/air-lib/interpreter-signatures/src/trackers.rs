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

use air_interpreter_cid::CID;
use fluence_keypair::error::SigningError;
use fluence_keypair::KeyPair;

use std::rc::Rc;

pub trait CidTracker {
    type Signature;

    fn new(current_peer_id: impl Into<Rc<String>>) -> Self;

    fn register<T>(&mut self, peer: &str, cid: &CID<T>);

    fn gen_signature(self, keypair: &KeyPair) -> Result<Self::Signature, SigningError>;
}

/// The tracker that collect current peer's CIDs only.
#[derive(Debug)]
pub struct PeerCidTracker {
    current_peer_id: Rc<String>,
    cids: Vec<Box<str>>,
}

impl CidTracker for PeerCidTracker {
    type Signature = crate::Signature;
    fn new(current_peer_id: impl Into<Rc<String>>) -> Self {
        Self {
            current_peer_id: current_peer_id.into(),
            cids: vec![],
        }
    }

    fn register<T>(&mut self, peer: &str, cid: &CID<T>) {
        if peer == *self.current_peer_id {
            self.cids.push(cid.clone().into_inner().into())
        }
    }

    fn gen_signature(self, keypair: &KeyPair) -> Result<Self::Signature, SigningError> {
        sign_cids(self.cids, keypair)
    }
}

#[derive(Debug)]
pub struct NullCidTracker;

impl CidTracker for NullCidTracker {
    type Signature = ();

    fn new(_current_peer_id: impl Into<Rc<String>>) -> Self {
        Self
    }

    fn register<T>(&mut self, _peer: &str, _cid: &CID<T>) {}

    fn gen_signature(self, _keypair: &KeyPair) -> Result<(), SigningError> {
        Ok(())
    }
}

pub fn sign_cids(
    mut cids: Vec<Box<str>>,
    keypair: &KeyPair,
) -> Result<crate::Signature, SigningError> {
    cids.sort_unstable();

    // TODO make pluggable serialization
    // TODO it will be useful for CID too
    // TODO please note that using serde::Serializer is not enough
    let serialized_cids =
        serde_json::to_string(&cids).expect("default serialization shouldn't fail");

    keypair
        .sign(serialized_cids.as_bytes())
        .map(crate::Signature::new)
}
