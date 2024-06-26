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
