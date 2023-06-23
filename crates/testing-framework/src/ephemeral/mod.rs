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
use crate::{
    queue::PeerQueueCell,
    services::{services_to_call_service_closure, MarineServiceHandle, NetworkServices},
};

use air_test_utils::{
    test_runner::{create_custom_avm, AirRunner, DefaultAirRunner, TestRunParameters, TestRunner},
    RawAVMOutcome,
};

use std::{borrow::Borrow, cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PeerId(Rc<str>);

impl PeerId {
    pub fn new<'any>(peer_id: impl Into<&'any str>) -> Self {
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

pub struct Peer<R> {
    pub(crate) peer_id: PeerId,
    runner: TestRunner<R>,
}

impl<R: AirRunner> Peer<R> {
    pub fn new(peer_id: impl Into<PeerId>, services: Rc<[MarineServiceHandle]>) -> Self {
        let peer_id = Into::into(peer_id);
        let call_service = services_to_call_service_closure(services);
        let runner = create_custom_avm(call_service, &*peer_id.0);

        Self { peer_id, runner }
    }

    pub(crate) fn invoke(
        &mut self,
        air: impl Into<String>,
        data: Data,
        test_run_params: TestRunParameters,
        queue_cell: &PeerQueueCell,
    ) -> Result<RawAVMOutcome, String> {
        let prev_data = queue_cell.take_prev_data();
        let res = self.runner.call(air, prev_data, data, test_run_params);
        if let Ok(outcome) = &res {
            queue_cell.set_prev_data(outcome.data.clone());
        }
        res
    }
}

impl<R: AirRunner> std::fmt::Debug for Peer<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Peer")
            .field("peer_id", &self.peer_id)
            .field("services", &"...")
            .finish()
    }
}

pub struct Network<R = DefaultAirRunner> {
    peers: RefCell<HashMap<PeerId, Rc<RefCell<PeerEnv<R>>>>>,
    services: Rc<NetworkServices>,
}

// it is implemented only for the default runner for compatibility reasons
// Rust fails to deduce type for `Network::empty()` without
//   extencive test code changes
impl Network<DefaultAirRunner> {
    pub fn empty() -> Rc<Self> {
        Self::new(std::iter::empty::<PeerId>(), vec![])
    }
}

impl<R: AirRunner> Network<R> {
    pub fn new(
        peers: impl Iterator<Item = impl Into<PeerId>>,
        common_services: Vec<MarineServiceHandle>,
    ) -> Rc<Self> {
        let network = Rc::new(Self {
            peers: Default::default(),
            services: NetworkServices::new(common_services).into(),
        });
        for peer_id in peers {
            network.ensure_peer(peer_id);
        }
        network
    }

    pub fn from_peers(nodes: Vec<Peer<R>>) -> Rc<Self> {
        let network = Self::new(std::iter::empty::<PeerId>(), vec![]);
        let neighborhood: PeerSet = nodes.iter().map(|peer| peer.peer_id.clone()).collect();
        for peer in nodes {
            network.add_peer_env(peer, neighborhood.iter().cloned());
        }
        network
    }

    pub fn add_peer_env(
        self: &Rc<Self>,
        peer: Peer<R>,
        neighborhood: impl IntoIterator<Item = impl Into<PeerId>>,
    ) {
        let peer_id = peer.peer_id.clone();
        let mut peer_env = PeerEnv::new(peer, self);
        peer_env.extend_neighborhood(neighborhood.into_iter());
        self.insert_peer_env_entry(peer_id, peer_env);
    }

    pub fn ensure_peer(self: &Rc<Self>, peer_id: impl Into<PeerId>) {
        let peer_id = peer_id.into();
        let exists = {
            let peers_ref = self.peers.borrow();
            peers_ref.contains_key(&peer_id)
        };
        if !exists {
            let peer = Peer::new(peer_id, self.services.get_services());
            self.add_peer(peer);
        }
    }

    /// Add a peer with default neighborhood.
    pub fn add_peer(self: &Rc<Self>, peer: Peer<R>) {
        let peer_id = peer.peer_id.clone();
        let peer_env = PeerEnv::new(peer, self);
        self.insert_peer_env_entry(peer_id, peer_env);
    }

    fn insert_peer_env_entry(&self, peer_id: PeerId, peer_env: PeerEnv<R>) {
        let mut peers_ref = self.peers.borrow_mut();
        let peer_env = Rc::new(peer_env.into());
        // It will be simplified with entry_insert stabilization
        // https://github.com/rust-lang/rust/issues/65225
        match peers_ref.entry(peer_id) {
            std::collections::hash_map::Entry::Occupied(ent) => {
                let cell = ent.into_mut();
                *cell = peer_env;
                cell
            }
            std::collections::hash_map::Entry::Vacant(ent) => ent.insert(peer_env),
        };
    }

    pub fn set_peer_failed<Id>(&mut self, peer_id: &Id, failed: bool)
    where
        PeerId: Borrow<Id>,
        Id: Hash + Eq + ?Sized,
    {
        let mut peers_ref = self.peers.borrow_mut();
        peers_ref
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
        let mut peers_ref = self.peers.borrow_mut();
        peers_ref
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
        let mut peers_ref = self.peers.borrow_mut();
        peers_ref
            .get_mut(source_peer_id)
            .expect("unknown peer")
            .as_ref()
            .borrow_mut()
            .get_neighborhood_mut()
            .unset_target_unreachable(target_peer_id);
    }

    // TODO there is some kind of unsymmetry between these methods and the fail/unfail:
    // the latters panic on unknown peer; perhaps, it's OK
    pub fn get_peer_env<Id>(&self, peer_id: &Id) -> Option<Rc<RefCell<PeerEnv<R>>>>
    where
        PeerId: Borrow<Id>,
        Id: Hash + Eq + ?Sized,
    {
        let peers_ref = self.peers.borrow();
        peers_ref.get(peer_id).cloned()
    }

    pub(crate) fn get_services(&self) -> Rc<NetworkServices> {
        self.services.clone()
    }

    pub fn get_peers(&self) -> impl Iterator<Item = PeerId> {
        let peers_ref = self.peers.borrow();
        peers_ref.keys().cloned().collect::<Vec<_>>().into_iter()
    }
}
