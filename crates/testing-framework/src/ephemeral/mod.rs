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

pub mod neiborhood;

use self::neiborhood::{PeerSet, PeerWithNeighborhood};
use crate::services::{services_to_call_service_closure, Service};

use air_test_utils::{
    test_runner::{create_avm, TestRunParameters, TestRunner},
    RawAVMOutcome,
};

use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::{HashMap, HashSet},
    hash::Hash,
    rc::Rc,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PeerId(String);

impl PeerId {
    pub fn new(peer_id: impl Into<String>) -> Self {
        Self(peer_id.into())
    }
}
impl From<String> for PeerId {
    fn from(source: String) -> Self {
        Self(source)
    }
}

impl From<&str> for PeerId {
    fn from(source: &str) -> Self {
        Self(source.to_owned())
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
    pub fn new(peer_id: impl Into<PeerId>, services: Rc<[Rc<dyn Service>]>) -> Self {
        let peer_id = Into::into(peer_id);
        let call_service = services_to_call_service_closure(services);
        let runner = create_avm(call_service, &peer_id.0);

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

#[derive(Debug)]
pub struct Network {
    test_parameters: TestRunParameters,
    peers: HashMap<PeerId, Rc<RefCell<PeerWithNeighborhood>>>,
    default_neiborhood: HashSet<PeerId>,
}

impl Network {
    pub fn empty(test_parameters: TestRunParameters) -> Self {
        Self::new(test_parameters, std::iter::empty::<&str>())
    }

    pub fn new(
        test_parameters: TestRunParameters,
        default_neiborhoud: impl Iterator<Item = impl Into<PeerId>>,
    ) -> Self {
        Self {
            test_parameters,
            peers: Default::default(),
            default_neiborhood: default_neiborhoud.map(Into::into).collect(),
        }
    }

    pub fn from_vec(test_parameters: TestRunParameters, nodes: Vec<Peer>) -> Self {
        let mut network = Self::empty(test_parameters);
        let neighborhood: PeerSet = nodes.iter().map(|peer| peer.peer_id.clone()).collect();
        for peer in nodes {
            // TODO can peer have itself as a neighbor?
            network.add_peer_with_neighborhood(peer, neighborhood.iter().cloned());
        }
        network
    }

    pub fn add_peer_with_neighborhood(
        &mut self,
        peer: Peer,
        neighborhood: impl IntoIterator<Item = impl Into<PeerId>>,
    ) -> &mut PeerWithNeighborhood {
        let peer_id = peer.peer_id.clone();
        let mut peer_with_neigh = PeerWithNeighborhood::new(peer);
        peer_with_neigh.extend_neighborhood(neighborhood.into_iter().map(Into::into));
        self.peers
            .insert(peer_id.clone(), Rc::new(peer_with_neigh.into()));
        Rc::get_mut(self.peers.get_mut(&peer_id).unwrap())
            .unwrap()
            .get_mut()
    }

    /// Add a peer with default neighborhood.
    pub fn add_peer(&mut self, peer: Peer) -> &mut PeerWithNeighborhood {
        let peer_id = peer.peer_id.clone();
        let mut peer_with_neigh = PeerWithNeighborhood::new(peer);
        peer_with_neigh.extend_neighborhood(self.default_neiborhood.iter().cloned());
        self.peers
            .insert(peer_id.clone(), Rc::new(peer_with_neigh.into()));
        Rc::get_mut(self.peers.get_mut(&peer_id).unwrap())
            .unwrap()
            .get_mut()
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
            .fail(target_peer_id);
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
            .unfail(target_peer_id);
    }

    // TODO there is some kind of unsymmetry between these methods and the fail/unfail:
    // the latters panic on unknown peer; perhaps, it's OK
    pub fn get_peer_env<Id>(&self, peer_id: &Id) -> Option<Rc<RefCell<PeerWithNeighborhood>>>
    where
        PeerId: Borrow<Id>,
        Id: Hash + Eq + ?Sized,
    {
        self.peers.get(peer_id).cloned()
    }

    pub fn iter_execution<'s, Id>(
        &'s self,
        air: &'s str,
        peer_id: &Id,
    ) -> Option<impl Iterator<Item = Result<RawAVMOutcome, String>> + 's>
    where
        PeerId: Borrow<Id>,
        Id: Eq + Hash + ?Sized,
    {
        let peer = self.get_peer_env(peer_id);

        peer.map(|peer_cell| {
            std::iter::from_fn(move || {
                let mut peer_env = peer_cell.borrow_mut();
                peer_env.execute_once(air, self)
            })
        })
    }

    pub fn distribute_to_peers(&self, peers: &[String], data: &Data) {
        for peer_id in peers {
            if let Some(peer_cell) = self.get_peer_env(peer_id.as_str()) {
                peer_cell.borrow_mut().data_queue.push_back(data.clone());
            }
        }
    }
}
