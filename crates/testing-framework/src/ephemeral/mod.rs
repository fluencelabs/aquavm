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
    key_utils::derive_dummy_keypair,
    test_runner::{
        create_avm_with_key, AirRunner, DefaultAirRunner, TestRunParameters, TestRunner,
    },
    RawAVMOutcome,
};
use fluence_keypair::KeyPair;

use std::{borrow::Borrow, cell::RefCell, collections::HashMap, hash::Hash, ops::Deref, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PeerId(Rc<str>);

impl PeerId {
    pub fn new<'any>(peer_id: impl Into<&'any str>) -> Self {
        Self(peer_id.into().into())
    }

    pub fn from_keypair(keypair: &KeyPair) -> Self {
        Self::new(keypair.public().to_peer_id().to_string().as_str())
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

impl From<&PeerId> for PeerId {
    fn from(value: &PeerId) -> Self {
        value.clone()
    }
}

impl Borrow<str> for PeerId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl Deref for PeerId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type Data = Vec<u8>;

pub struct Peer<R> {
    pub(crate) peer_id: PeerId,
    runner: TestRunner<R>,
}

impl<R: AirRunner> Peer<R> {
    pub fn new(keypair: KeyPair, services: Rc<[MarineServiceHandle]>) -> Self {
        let call_service = services_to_call_service_closure(services);

        let runner = create_avm_with_key::<R>(keypair, call_service);
        let peer_id = runner.runner.get_current_peer_id().into();

        Self { peer_id, runner }
    }

    pub fn get_peer_id(&self) -> &PeerId {
        &self.peer_id
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

    // Default peer services.
    services: Rc<NetworkServices>,

    // Resolves human-readable peer names to real peer IDs.
    resolver: RefCell<HashMap<PeerId, PeerId>>,
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
        named_peers: impl Iterator<Item = impl Into<PeerId>>,
        common_services: Vec<MarineServiceHandle>,
    ) -> Rc<Self> {
        let network = Rc::new(Self {
            peers: Default::default(),
            services: NetworkServices::new(common_services).into(),
            resolver: Default::default(),
        });
        for peer_name in named_peers {
            network.ensure_named_peer(peer_name);
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

    pub fn ensure_named_peer(self: &Rc<Self>, name: impl Into<PeerId>) -> PeerId {
        use std::collections::hash_map::Entry;

        let name = name.into();

        match self.resolver.borrow_mut().entry(name.clone()) {
            Entry::Occupied(entry) => entry.get().clone(),
            Entry::Vacant(empty) => {
                let (keypair, _) = derive_dummy_keypair(&name);
                let peer = Peer::new(keypair, self.services.get_services());
                let peer_id = peer.get_peer_id().clone();
                self.add_peer(peer);

                empty.insert(peer_id.clone());

                peer_id
            }
        }
    }

    /// Add a peer with default neighborhood.
    pub fn add_peer(self: &Rc<Self>, peer: Peer<R>) {
        let peer_id = peer.peer_id.clone();
        let peer_env = PeerEnv::new(peer, self);
        self.insert_peer_env_entry(peer_id, peer_env);
    }

    fn insert_peer_env_entry(&self, peer_id: PeerId, peer_env: PeerEnv<R>) {
        use std::collections::hash_map::Entry;

        let mut peers_ref = self.peers.borrow_mut();
        let peer_env = Rc::new(peer_env.into());
        // It will be simplified with entry_insert stabilization
        // https://github.com/rust-lang/rust/issues/65225
        match peers_ref.entry(peer_id) {
            Entry::Occupied(ent) => {
                let cell = ent.into_mut();
                *cell = peer_env;
                cell
            }
            Entry::Vacant(ent) => ent.insert(peer_env),
        };
    }

    // TODO named peer
    pub fn set_peer_failed<Id>(&mut self, name: &Id, failed: bool)
    where
        PeerId: Borrow<Id> + for<'a> From<&'a Id>,
        Id: Hash + Eq + ?Sized,
    {
        let peer_id = self.resolve_peer(name);
        let mut peers_ref = self.peers.borrow_mut();
        peers_ref
            .get_mut::<PeerId>(&peer_id)
            .expect("unknown peer")
            .as_ref()
            .borrow_mut()
            .set_failed(failed);
    }

    // TODO named peer
    pub fn fail_peer_for<Id1, Id2>(&mut self, source_peer_name: &Id1, target_peer_name: &Id2)
    where
        PeerId: Borrow<Id1> + for<'a> From<&'a Id1>,
        Id1: Hash + Eq + ?Sized,
        PeerId: Borrow<Id2> + for<'a> From<&'a Id2>,
        Id2: Hash + Eq + ?Sized,
    {
        let source_peer_id = self.resolve_peer(source_peer_name);
        let target_peer_id = self.resolve_peer(target_peer_name);

        let mut peers_ref = self.peers.borrow_mut();
        peers_ref
            .get_mut::<PeerId>(&source_peer_id)
            .expect("unknown peer")
            .as_ref()
            .borrow_mut()
            .get_neighborhood_mut()
            .set_target_unreachable(&target_peer_id);
    }

    // TODO named peer
    pub fn unfail_peer_for<Id1, Id2>(&mut self, source_peer_name: &Id1, target_peer_name: &Id2)
    where
        PeerId: Borrow<Id1> + for<'a> From<&'a Id1>,
        Id1: Hash + Eq + ?Sized,
        PeerId: Borrow<Id2> + for<'a> From<&'a Id2>,
        Id2: Hash + Eq + ?Sized,
    {
        let source_peer_id = self.resolve_peer(source_peer_name);
        let target_peer_id = self.resolve_peer(target_peer_name);
        let mut peers_ref = self.peers.borrow_mut();
        peers_ref
            .get_mut(&source_peer_id)
            .expect("unknown peer")
            .as_ref()
            .borrow_mut()
            .get_neighborhood_mut()
            .unset_target_unreachable(&target_peer_id);
    }

    // TODO there is some kind of unsymmetry between these methods and the fail/unfail:
    // the latters panic on unknown peer; perhaps, it's OK
    // TODO named peer
    pub fn get_peer_env<Id>(&self, peer_id: &Id) -> Option<Rc<RefCell<PeerEnv<R>>>>
    where
        PeerId: Borrow<Id> + for<'a> From<&'a Id>,
        Id: Hash + Eq + ?Sized,
    {
        let peers_ref = self.peers.borrow();
        peers_ref.get(peer_id).cloned()
    }

    pub fn get_named_peer_env<Id>(&self, peer_name: &Id) -> Option<Rc<RefCell<PeerEnv<R>>>>
    where
        PeerId: Borrow<Id> + for<'a> From<&'a Id>,
        Id: Hash + Eq + ?Sized,
    {
        let peer_id: PeerId = self.resolve_peer(peer_name);
        self.get_peer_env::<PeerId>(&peer_id)
    }

    pub(crate) fn get_services(&self) -> Rc<NetworkServices> {
        self.services.clone()
    }

    pub fn get_peers(&self) -> impl Iterator<Item = PeerId> {
        let peers_ref = self.peers.borrow();
        peers_ref.keys().cloned().collect::<Vec<_>>().into_iter()
    }

    // TODO it sounds cool, but actually, name and PeerId should be
    // distinct and have distinct API for working with a peer by name: &str
    // and by PeerId
    pub fn resolve_peer<Id>(&self, name: &Id) -> PeerId
    where
        PeerId: Borrow<Id> + for<'a> From<&'a Id>,
        Id: Hash + Eq + ?Sized,
    {
        let resolver = self.resolver.borrow();
        resolver.get(name).cloned().unwrap_or_else(|| (name).into())
    }
}
