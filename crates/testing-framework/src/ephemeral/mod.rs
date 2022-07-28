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
use crate::services::FunctionOutcome;

use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    hash::Hash,
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

pub struct Particle {}

#[derive(Debug)]
pub struct Peer {
    peer_id: PeerId,
    // We presume that only one particle is run over the network.
    prev_data: Data,
}

impl Peer {
    pub fn new(peer_id: impl Into<PeerId>) -> Self {
        Self {
            peer_id: Into::into(peer_id),
            prev_data: vec![],
        }
    }

    pub fn set_prev_data(&mut self, prev_data: Vec<u8>) {
        self.prev_data = prev_data;
    }

    pub fn invoke(&mut self, particle: Particle) -> FunctionOutcome {
        todo!()
    }
}

#[derive(Debug)]
pub struct Network {
    peers: HashMap<PeerId, PeerWithNeighborhood>,
    default_neiborhood: HashSet<PeerId>,
}

impl Network {
    pub fn empty() -> Self {
        Self::new(std::iter::empty::<&str>())
    }

    pub fn new(default_neiborhoud: impl Iterator<Item = impl Into<PeerId>>) -> Self {
        Self {
            peers: Default::default(),
            default_neiborhood: default_neiborhoud.map(Into::into).collect(),
        }
    }

    pub fn from_vec(nodes: Vec<Peer>) -> Self {
        let mut network = Self::empty();
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
        self.peers.insert(peer_id.clone(), peer_with_neigh);
        self.peers.get_mut(&peer_id).unwrap()
    }

    /// Add a peer with default neighborhood.
    pub fn add_peer(&mut self, peer: Peer) -> &mut PeerWithNeighborhood {
        let peer_id = peer.peer_id.clone();
        let mut peer_with_neigh = PeerWithNeighborhood::new(peer);
        peer_with_neigh.extend_neighborhood(self.default_neiborhood.iter().cloned());
        self.peers.insert(peer_id.clone(), peer_with_neigh);
        self.peers.get_mut(&peer_id).unwrap()
    }

    pub fn set_peer_failed<Id>(&mut self, peer_id: &Id, failed: bool)
    where
        PeerId: Borrow<Id>,
        Id: Hash + Eq + ?Sized,
    {
        self.peers
            .get_mut(peer_id)
            .expect("unknown peer")
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
            .get_neighborhood_mut()
            .unfail(target_peer_id);
    }

    // TODO there is some kind of unsymmetry between these methods and the fail/unfail:
    // the latters panic on unknown peer.
    pub fn get_peer<Id>(&self, peer_id: &Id) -> Option<&PeerWithNeighborhood>
    where
        PeerId: Borrow<Id>,
        Id: Hash + Eq + ?Sized,
    {
        self.peers.get(peer_id)
    }

    pub fn get_peer_mut<Id>(&mut self, peer_id: &Id) -> Option<&mut PeerWithNeighborhood>
    where
        PeerId: Borrow<Id>,
        Id: Hash + Eq + ?Sized,
    {
        self.peers.get_mut(peer_id)
    }
}
