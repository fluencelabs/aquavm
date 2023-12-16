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

use crate::KeyPair;
use crate::SaltedData;

use air_interpreter_cid::{CidRef, CID};
use fluence_keypair::error::SigningError;

use std::rc::Rc;

/// The tracker that collect current peer's CIDs only.
#[derive(Debug)]
pub struct PeerCidTracker {
    current_peer_id: Rc<String>,
    cids: Vec<Rc<CidRef>>,
}

impl PeerCidTracker {
    pub fn new(current_peer_id: impl Into<Rc<String>>) -> Self {
        Self {
            current_peer_id: current_peer_id.into(),
            cids: vec![],
        }
    }

    pub fn register<T>(&mut self, peer: &str, cid: &CID<T>) {
        if peer == *self.current_peer_id {
            self.cids.push(cid.get_inner())
        }
    }

    pub fn gen_signature(
        &self,
        salt: &str,
        keypair: &KeyPair,
    ) -> Result<crate::Signature, SigningError> {
        sign_cids(self.cids.clone(), salt, &keypair.0).map(Into::into)
    }
}

pub fn sign_cids(
    mut cids: Vec<Rc<CidRef>>,
    salt: &str,
    keypair: &fluence_keypair::KeyPair,
) -> Result<fluence_keypair::Signature, SigningError> {
    cids.sort_unstable();

    let serialized_cids = SaltedData::new(&cids, salt).serialize();
    keypair.sign(&serialized_cids)
}
