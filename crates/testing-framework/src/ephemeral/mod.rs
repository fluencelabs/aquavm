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
use crate::{clock::Clock, queue::Queue, services::FunctionOutcome};

use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
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
    clock: Clock,
    peers: HashMap<PeerId, PeerWithNeighborhood>,
    task_queue: Queue,
    default_neiborhood: HashSet<PeerId>,
}

impl Network {
    pub fn empty() -> Self {
        Self::new(std::iter::empty::<&str>())
    }

    pub fn new(default_neiborhoud: impl Iterator<Item = impl Into<PeerId>>) -> Self {
        Self {
            clock: Clock::new(),
            peers: Default::default(),
            task_queue: Default::default(),
            default_neiborhood: default_neiborhoud.map(Into::into).collect(),
        }
    }

    /// Add a peer with default neighborhood.
    pub fn add_peer(&mut self, peer: Peer) -> &mut PeerWithNeighborhood {
        let peer_id = peer.peer_id.clone();
        let mut peer_with_neigh = PeerWithNeighborhood::new(peer);
        peer_with_neigh.extend_neighborhood(self.default_neiborhood.iter().cloned());
        self.peers.insert(peer_id.clone(), peer_with_neigh);
        self.peers.get_mut(&peer_id).unwrap()
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
}
