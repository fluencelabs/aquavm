/*
 * Copyright 2022 Fluence Labs Limited
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

pub mod neighborhood;

use self::neighborhood::{PeerEnv, PeerSet};
use crate::services::{services_to_call_service_closure, MarineServiceHandle};

use air_test_utils::{
    test_runner::{create_avm, TestRunParameters, TestRunner},
    RawAVMOutcome,
};

use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::Deref,
    rc::Rc,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PeerId(Rc<str>);

impl PeerId {
    pub fn new<'a>(peer_id: impl Into<&'a str>) -> Self {
        Self(peer_id.into().into())
    }
}
impl From<String> for PeerId {
    fn from(source: String) -> Self {
        Self(source.as_str().into())
    }
}

impl From<&str> for PeerId {
    fn from(source: &str) -> Self {
        Self(source.into())
    }
}

impl Borrow<str> for PeerId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

pub type Data = Vec<u8>;

pub struct Peer {
    peer_id: PeerId,
    // We presume that only one particle is run over the network.
    prev_data: Data,
    runner: TestRunner,
}

impl Peer {
    pub fn new(peer_id: impl Into<PeerId>, services: Rc<[MarineServiceHandle]>) -> Self {
        let peer_id = Into::into(peer_id);
        let call_service = services_to_call_service_closure(services);
        let runner = create_avm(call_service, &*peer_id.0);

        Self {
            peer_id,
            prev_data: vec![],
            runner,
        }
    }

    pub fn invoke(
        &mut self,
        air: impl Into<String>,
        data: Data,
        test_run_params: TestRunParameters,
    ) -> Result<RawAVMOutcome, String> {
        let mut prev_data = vec![];
        std::mem::swap(&mut prev_data, &mut self.prev_data);
        let res = self.runner.call(air, prev_data, data, test_run_params);
        if let Ok(outcome) = &res {
            self.prev_data = outcome.data.clone();
        }
        res
    }
}

impl std::fmt::Debug for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Peer")
            .field("peer_id", &self.peer_id)
            .field("prev_data", &self.prev_data)
            .field("services", &"...")
            .finish()
    }
}

#[derive(Debug, Default)]
pub struct Network {
    peers: HashMap<PeerId, Rc<RefCell<PeerEnv>>>,
    default_neighborhood: HashSet<PeerId>,
}

impl Network {
    pub fn empty() -> Self {
        Default::default()
    }

    pub fn new(default_neiborhoud: impl Iterator<Item = impl Into<PeerId>>) -> Self {
        Self {
            peers: Default::default(),
            default_neighborhood: default_neiborhoud.map(Into::into).collect(),
        }
    }

    pub fn from_peers(nodes: Vec<Peer>) -> Self {
        let mut network = Self::empty();
        let neighborhood: PeerSet = nodes.iter().map(|peer| peer.peer_id.clone()).collect();
        for peer in nodes {
            network.add_peer_env(peer, neighborhood.iter().cloned());
        }
        network
    }

    pub fn add_peer_env(
        &mut self,
        peer: Peer,
        neighborhood: impl IntoIterator<Item = impl Into<PeerId>>,
    ) -> &mut PeerEnv {
        let peer_id = peer.peer_id.clone();
        let mut peer_env = PeerEnv::new(peer);
        peer_env.extend_neighborhood(neighborhood.into_iter());
        self.insert_peer_env_entry(peer_id, peer_env)
    }

    /// Add a peer with default neighborhood.
    pub fn add_peer(&mut self, peer: Peer) -> &mut PeerEnv {
        let peer_id = peer.peer_id.clone();
        let mut peer_env = PeerEnv::new(peer);
        peer_env.extend_neighborhood(self.default_neighborhood.iter().cloned());
        self.insert_peer_env_entry(peer_id, peer_env)
    }

    fn insert_peer_env_entry(&mut self, peer_id: PeerId, peer_env: PeerEnv) -> &mut PeerEnv {
        let peer_env = Rc::new(peer_env.into());
        // It will be simplified with entry_insert stabilization
        // https://github.com/rust-lang/rust/issues/65225
        let cell = match self.peers.entry(peer_id) {
            std::collections::hash_map::Entry::Occupied(ent) => {
                let cell = ent.into_mut();
                *cell = peer_env;
                cell
            }
            std::collections::hash_map::Entry::Vacant(ent) => ent.insert(peer_env),
        };
        // never panics because Rc have been just created and there's just single reference
        Rc::get_mut(cell).unwrap().get_mut()
    }

    pub fn set_peer_failed<Id>(&mut self, peer_id: &Id, failed: bool)
    where
        PeerId: Borrow<Id>,
        Id: Hash + Eq + ?Sized,
    {
        self.peers
            .get_mut(peer_id)
            .expect("unknown peer")
            .as_ref()
            .borrow_mut()
            .set_failed(failed);
    }

    pub fn fail_peer_for<Id>(&mut self, source_peer_id: &Id, target_peer_id: impl Into<PeerId>)
    where
        PeerId: Borrow<Id>,
        Id: Hash + Eq + ?Sized,
    {
        self.peers
            .get_mut(source_peer_id)
            .expect("unknown peer")
            .as_ref()
            .borrow_mut()
            .get_neighborhood_mut()
            .set_target_unreachable(target_peer_id);
    }

    pub fn unfail_peer_for<Id1, Id2>(&mut self, source_peer_id: &Id1, target_peer_id: &Id2)
    where
        PeerId: Borrow<Id1>,
        Id1: Hash + Eq + ?Sized,
        PeerId: Borrow<Id2>,
        Id2: Hash + Eq + ?Sized,
    {
        self.peers
            .get_mut(source_peer_id)
            .expect("unknown peer")
            .as_ref()
            .borrow_mut()
            .get_neighborhood_mut()
            .unset_target_unreachable(target_peer_id);
    }

    // TODO there is some kind of unsymmetry between these methods and the fail/unfail:
    // the latters panic on unknown peer; perhaps, it's OK
    pub fn get_peer_env<Id>(&self, peer_id: &Id) -> Option<Rc<RefCell<PeerEnv>>>
    where
        PeerId: Borrow<Id>,
        Id: Hash + Eq + ?Sized,
    {
        self.peers.get(peer_id).cloned()
    }

    /// Iterator for handling al the queued data.  It borrows peer env's `RefCell` only temporarily.
    /// Following test-utils' call_vm macro, it panics on failed VM.
    pub fn execution_iter<'s, Id>(
        &'s self,
        air: &'s str,
        test_parameters: &'s TestRunParameters,
        peer_id: &Id,
    ) -> Option<impl Iterator<Item = RawAVMOutcome> + 's>
    where
        PeerId: Borrow<Id>,
        Id: Eq + Hash + ?Sized,
    {
        let peer_env = self.get_peer_env(peer_id);

        peer_env.map(|peer_env_cell| {
            std::iter::from_fn(move || {
                let mut peer_env = peer_env_cell.borrow_mut();
                peer_env
                    .execute_once(air, self, test_parameters)
                    .map(|r| r.unwrap_or_else(|err| panic!("VM call failed: {}", err)))
            })
        })
    }

    pub fn distribute_to_peers<Id>(&self, peers: &[Id], data: &Data)
    where
        Id: Deref<Target = str>,
    {
        for peer_id in peers {
            if let Some(peer_env_cell) = self.get_peer_env::<str>(peer_id) {
                peer_env_cell
                    .borrow_mut()
                    .data_queue
                    .push_back(data.clone());
            }
        }
    }
}
