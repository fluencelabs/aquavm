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


use std::{collections::{HashMap, HashSet}, ops::Deref};

use crate::{clock::Clock, queue::Queue, services::FunctionOutcome};

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

type PeerSet = HashSet<PeerId>;

#[derive(Debug, Default)]
pub struct Neighborghood {
    neighbors: PeerSet,
    // A neighbor can be unreachable for some time.
    failing: PeerSet,
}

#[derive(Debug)]
pub struct PeerWithNeighborhood {
    peer: Peer,
    neighborhood: Neighborghood,
}

impl PeerWithNeighborhood {
    pub fn new(peer: Peer) -> Self {
        Self {
            peer,
            neighborhood: Default::default(),
        }
    }

    pub fn is_reachable(&self, target: impl Deref<Target = PeerId>) -> bool {
        let t = target.deref();
        (&self.peer.peer_id == t) || self.neighborhood.is_reachable(target)
    }

    pub fn get_neighborhood(&self) -> &Neighborghood {
        &self.neighborhood
    }

    pub fn get_neighborhood_mut(&mut self) -> &mut Neighborghood {
        &mut self.neighborhood
    }

    pub fn iter_neighbors(&mut self) -> impl Iterator<Item=&PeerId> {
        self.neighborhood.iter_neighbors()
    }

}

impl Neighborghood {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_neighbors(&mut self, neighbors: PeerSet) {
        self.neighbors = neighbors;
    }

    pub fn iter_neighbors(&mut self) -> impl Iterator<Item=&PeerId> {
        self.neighbors.iter()
    }

    pub fn insert(&mut self, other_peer_id: impl Into<PeerId>) {
        let other_peer_id = other_peer_id.into();
        self.unfail(&other_peer_id);
        self.neighbors.insert(other_peer_id);
    }

    pub fn remove(&mut self, other_peer_id: impl Into<PeerId>) {
        let other_peer_id = other_peer_id.into();
        self.unfail(&other_peer_id);
        self.neighbors.remove(&other_peer_id);
    }

    pub fn fail(&mut self, target: impl Into<PeerId>) {
        self.failing.insert(target.into());
    }

    pub fn unfail(&mut self, target: impl Deref<Target = PeerId>) {
        self.failing.remove(&target);
    }

    pub fn is_reachable(&self, target: impl Deref<Target = PeerId>) -> bool {
        let t = target.deref();
        self.neighbors.contains(t) && !self.failing.contains(t)
    }
}

#[derive(Debug)]
pub struct Network {
    clock: Clock,
    peers: HashMap<PeerId, PeerWithNeighborhood>,
    task_queue: Queue,
}

impl Network {
    pub fn new() -> Self {
        Self {
            clock: Clock::new(),
            peers: Default::default(),
            task_queue: Default::default(),
        }
    }

    pub fn from_vec(nodes: Vec<Peer>) -> Self {
        let mut network = Self::new();
        let neighborhood: PeerSet = nodes.iter().map(|peer| peer.peer_id.clone()).collect();
        for peer in nodes {
            // TODO can peer have itself as a neighbor?
            network.add_peer_with_neighborhood(peer, neighborhood.iter().cloned());
        }
        network
    }

    pub fn add_peer_with_neighborhood(&mut self, peer: Peer, neighborhood: impl IntoIterator<Item = impl Into<PeerId>>) -> &mut PeerWithNeighborhood {
        let peer_id = peer.peer_id.clone();
        let mut peer_with_neigh = PeerWithNeighborhood::new(peer);
        peer_with_neigh.neighborhood.neighbors.extend(neighborhood.into_iter().map(Into::into));
        self.peers.insert(peer_id.clone(), peer_with_neigh);
        self.peers.get_mut(&peer_id).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_neighborhood() {
        let peer_id: PeerId = "someone".into();
        let other_id: PeerId = "other".into();
        let pwn = PeerWithNeighborhood::new(Peer::new(peer_id.clone()));
        assert!(pwn.is_reachable(&peer_id));
        assert!(!pwn.is_reachable(&other_id));
    }

    #[test]
    fn test_no_self_disconnect() {
        let peer_id: PeerId = "someone".into();
        let other_id: PeerId = "other".into();
        let mut pwn = PeerWithNeighborhood::new(Peer::new(peer_id.clone()));
        let nei = pwn.get_neighborhood_mut();
        nei.insert(peer_id.clone());
        nei.remove(peer_id.clone());
        assert!(pwn.is_reachable(&peer_id));
        assert!(!pwn.is_reachable(&other_id));
    }

    #[test]
    fn test_no_self_fail() {
        let peer_id: PeerId = "someone".into();
        let other_id: PeerId = "other".into();
        let mut pwn = PeerWithNeighborhood::new(Peer::new(peer_id.clone()));
        pwn.get_neighborhood_mut().fail(peer_id.clone());
        assert!(pwn.is_reachable(&peer_id));
        assert!(!pwn.is_reachable(&other_id));
    }

    #[test]
    fn test_set_neighborhood() {
        let peer_id: PeerId = "someone".into();
        let other_id1: PeerId = "other1".into();
        let other_id2: PeerId = "other2".into();
        let mut pwn = PeerWithNeighborhood::new(Peer::new(peer_id.clone()));

        // iter is empty
        assert!(pwn.iter_neighbors().next().is_none());

        let expected_neighborhood = vec![other_id1.clone(), other_id2.clone()].into_iter().collect::<PeerSet>();
        pwn.get_neighborhood_mut().set_neighbors(expected_neighborhood.clone());
        assert_eq!(pwn.iter_neighbors().cloned().collect::<PeerSet>(), expected_neighborhood);
    }

    #[test]
    fn test_insert() {
        let peer_id: PeerId = "someone".into();
        let other_id1: PeerId = "other1".into();
        let other_id2: PeerId = "other2".into();
        let mut pwn = PeerWithNeighborhood::new(Peer::new(peer_id.clone()));

        // iter is empty
        assert!(pwn.iter_neighbors().next().is_none());
        let nei = pwn.get_neighborhood_mut();

        nei.insert(other_id1.clone());
        nei.insert(other_id2.clone());
        let expected_neighborhood = vec![other_id1.clone(), other_id2.clone()].into_iter().collect::<PeerSet>();
        assert_eq!(pwn.iter_neighbors().cloned().collect::<PeerSet>(), expected_neighborhood);
    }

    #[test]
    fn test_insert_insert() {
        let peer_id: PeerId = "someone".into();
        let other_id1: PeerId = "other1".into();
        let mut pwn = PeerWithNeighborhood::new(Peer::new(peer_id.clone()));

        // iter is empty
        assert!(pwn.iter_neighbors().next().is_none());

        let nei = pwn.get_neighborhood_mut();
        nei.insert(other_id1.clone());
        nei.insert(other_id1.clone());
        let expected_neighborhood = vec![other_id1];
        assert_eq!(pwn.iter_neighbors().cloned().collect::<Vec<_>>(), expected_neighborhood);
    }

    #[test]
    fn test_fail() {
        let peer_id: PeerId = "someone".into();
        let other_id: PeerId = "other".into();
        let mut pwn = PeerWithNeighborhood::new(Peer::new(peer_id.clone()));
        let nei = pwn.get_neighborhood_mut();
        nei.insert(other_id.clone());
        nei.fail(other_id.clone());

        let expected_neighborhood = vec![other_id.clone()].into_iter().collect::<PeerSet>();
        assert_eq!(pwn.iter_neighbors().cloned().collect::<PeerSet>(), expected_neighborhood);
        assert!(!pwn.is_reachable(&other_id));
    }

    #[test]
    fn test_fail_remove() {
        let peer_id: PeerId = "someone".into();
        let other_id: PeerId = "other".into();
        let mut pwn = PeerWithNeighborhood::new(Peer::new(peer_id.clone()));

        let nei = pwn.get_neighborhood_mut();
        nei.insert(other_id.clone());
        nei.fail(other_id.clone());
        assert!(!pwn.is_reachable(&other_id));

        let nei = pwn.get_neighborhood_mut();
        nei.remove(other_id.clone());
        assert!(!pwn.is_reachable(&other_id));

        let nei = pwn.get_neighborhood_mut();
        nei.insert(other_id.clone());
        assert!(pwn.is_reachable(&other_id));
    }

    #[test]
    fn test_fail_unfail() {
        let peer_id: PeerId = "someone".into();
        let other_id: PeerId = "other".into();
        let mut pwn = PeerWithNeighborhood::new(Peer::new(peer_id.clone()));

        let nei = pwn.get_neighborhood_mut();
        nei.insert(other_id.clone());
        nei.fail(other_id.clone());
        assert!(!pwn.is_reachable(&other_id));

        let nei = pwn.get_neighborhood_mut();
        nei.unfail(&other_id);
        assert!(pwn.is_reachable(&other_id));
    }

    #[test]
    fn test_uninserted_fail_unfail() {
        let peer_id: PeerId = "someone".into();
        let other_id: PeerId = "other".into();
        let mut pwn = PeerWithNeighborhood::new(Peer::new(peer_id.clone()));

        let nei = pwn.get_neighborhood_mut();
        nei.fail(other_id.clone());
        assert!(!pwn.is_reachable(&other_id));

        let nei = pwn.get_neighborhood_mut();
        nei.unfail(&other_id);
        assert!(!pwn.is_reachable(&other_id));
    }
}
