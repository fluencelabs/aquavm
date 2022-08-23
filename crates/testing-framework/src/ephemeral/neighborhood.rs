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

use super::{Data, Network, Peer, PeerId};

use std::{
    borrow::Borrow,
    collections::{HashSet, VecDeque},
    hash::Hash,
    ops::Deref,
};

pub(crate) type PeerSet = HashSet<PeerId>;

#[derive(Debug, Default)]
pub struct Neighborhood {
    neighbors: PeerSet,
    // A neighbor can be unreachable for some time.
    failing: PeerSet,
}

impl Neighborhood {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_neighbors(&mut self, neighbors: PeerSet) {
        self.neighbors = neighbors;
    }

    pub fn iter(&self) -> impl Iterator<Item = &PeerId> {
        self.into_iter()
    }

    pub fn insert(&mut self, other_peer_id: impl Into<PeerId>) {
        let other_peer_id = other_peer_id.into();
        self.unfail(&other_peer_id);
        self.neighbors.insert(other_peer_id);
    }

    pub fn remove<Id>(&mut self, other_peer_id: &Id)
    where
        PeerId: Borrow<Id>,
        Id: Eq + Hash + ?Sized,
    {
        self.unfail(other_peer_id);
        self.neighbors.remove(other_peer_id);
    }

    pub fn fail(&mut self, target: impl Into<PeerId>) {
        self.failing.insert(target.into());
    }

    pub fn unfail<Id>(&mut self, target: &Id)
    where
        PeerId: Borrow<Id>,
        Id: Eq + Hash + ?Sized,
    {
        self.failing.remove(target);
    }

    pub fn is_reachable(&self, target: impl Deref<Target = PeerId>) -> bool {
        let t = target.deref();
        self.neighbors.contains(t) && !self.failing.contains(t)
    }
}

impl<'a> std::iter::IntoIterator for &'a Neighborhood {
    type Item = &'a PeerId;

    type IntoIter = std::collections::hash_set::Iter<'a, PeerId>;

    fn into_iter(self) -> Self::IntoIter {
        self.neighbors.iter()
    }
}


#[derive(Debug)]
pub struct PeerWithNeighborhood {
    pub(crate) peer: Peer,
    failed: bool,
    neighborhood: Neighborhood,
    pub(crate) data_queue: VecDeque<Data>,
}

impl PeerWithNeighborhood {
    pub fn new(peer: Peer) -> Self {
        Self {
            peer,
            failed: false,
            neighborhood: Default::default(),
            data_queue: Default::default(),
        }
    }

    pub fn is_failed(&self) -> bool {
        self.failed
    }

    pub fn set_failed(&mut self, failed: bool) {
        self.failed = failed;
    }

    pub fn is_reachable(&self, target: impl Deref<Target = PeerId>) -> bool {
        if self.is_failed() {
            return false;
        }

        let t = target.deref();
        if &self.peer.peer_id == t {
            return true;
        }

        self.neighborhood.is_reachable(target)
    }

    pub fn extend_neighborhood(&mut self, peers: impl Iterator<Item = impl Into<PeerId>>) {
        for peer_id in peers {
            self.neighborhood.insert(peer_id.into());
        }
    }

    pub fn remove_from_neighborhood<'a, Id>(&mut self, peers: impl Iterator<Item = &'a Id>)
    where
        PeerId: std::borrow::Borrow<Id>,
        Id: Eq + Hash + ?Sized + 'a,
    {
        for peer_id in peers {
            self.neighborhood.remove(peer_id);
        }
    }

    pub fn get_neighborhood(&self) -> &Neighborhood {
        &self.neighborhood
    }

    pub fn get_neighborhood_mut(&mut self) -> &mut Neighborhood {
        &mut self.neighborhood
    }

    pub fn iter(&self) -> impl Iterator<Item = &PeerId> {
        self.neighborhood.iter()
    }

    pub fn send_data(&mut self, data: Data) {
        self.data_queue.push_back(data);
    }

    pub fn execute_once(
        &mut self,
        air: impl Into<String>,
        network: &Network,
    ) -> Option<Result<air_test_utils::RawAVMOutcome, String>> {
        let maybe_data = self.data_queue.pop_front();

        maybe_data.map(|data| {
            let res = self.peer.invoke(air, data, network.test_parameters.clone());

            if let Ok(outcome) = &res {
                network.distribute_to_peers(&outcome.next_peer_pks, &outcome.data)
            }

            res
        })
    }
}

impl<'a> IntoIterator for &'a PeerWithNeighborhood {
    type Item = <&'a Neighborhood as IntoIterator>::Item;
    type IntoIter = <&'a Neighborhood as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.neighborhood.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use std::{iter::FromIterator, rc::Rc};

    use super::*;

    #[test]
    fn test_empty_neighborhood() {
        let peer_id: PeerId = "someone".into();
        let other_id: PeerId = "other".into();
        let pwn = PeerWithNeighborhood::new(Peer::new(peer_id.clone(), Rc::from(vec![])));
        assert!(pwn.is_reachable(&peer_id));
        assert!(!pwn.is_reachable(&other_id));
    }

    #[test]
    fn test_no_self_disconnect() {
        let peer_id: PeerId = "someone".into();
        let other_id: PeerId = "other".into();
        let mut pwn = PeerWithNeighborhood::new(Peer::new(peer_id.clone(), Rc::from(vec![])));
        let nei = pwn.get_neighborhood_mut();
        nei.insert(peer_id.clone());
        nei.remove(&peer_id);
        assert!(pwn.is_reachable(&peer_id));
        assert!(!pwn.is_reachable(&other_id));
    }

    #[test]
    fn test_no_self_fail() {
        let peer_id: PeerId = "someone".into();
        let other_id: PeerId = "other".into();
        let mut pwn = PeerWithNeighborhood::new(Peer::new(peer_id.clone(), Rc::from(vec![])));
        pwn.get_neighborhood_mut().fail(peer_id.clone());
        assert!(pwn.is_reachable(&peer_id));
        assert!(!pwn.is_reachable(&other_id));
    }

    #[test]
    fn test_set_neighborhood() {
        let peer_id: PeerId = "someone".into();
        let other_id1: PeerId = "other1".into();
        let other_id2: PeerId = "other2".into();
        let mut pwn = PeerWithNeighborhood::new(Peer::new(peer_id.clone(), Rc::from(vec![])));

        // iter is empty
        assert!(pwn.iter().next().is_none());

        let expected_neighborhood = PeerSet::from([other_id1.clone(), other_id2.clone()]);
        pwn.get_neighborhood_mut()
            .set_neighbors(expected_neighborhood.clone());
        assert_eq!(
            pwn.iter().cloned().collect::<PeerSet>(),
            expected_neighborhood
        );
    }

    #[test]
    fn test_insert() {
        let peer_id: PeerId = "someone".into();
        let other_id1: PeerId = "other1".into();
        let other_id2: PeerId = "other2".into();
        let mut pwn = PeerWithNeighborhood::new(Peer::new(peer_id.clone(), Rc::from(vec![])));

        // iter is empty
        assert!(pwn.iter().next().is_none());
        let nei = pwn.get_neighborhood_mut();

        nei.insert(other_id1.clone());
        nei.insert(other_id2.clone());
        let expected_neighborhood = PeerSet::from([other_id1.clone(), other_id2.clone()]);
        assert_eq!(
            PeerSet::from_iter(pwn.iter().cloned()),
            expected_neighborhood
        );
    }

    #[test]
    fn test_insert_insert() {
        let peer_id: PeerId = "someone".into();
        let other_id1: PeerId = "other1".into();
        let mut pwn = PeerWithNeighborhood::new(Peer::new(peer_id.clone(), Rc::from(vec![])));

        // iter is empty
        assert!(pwn.iter().next().is_none());

        let nei = pwn.get_neighborhood_mut();
        nei.insert(other_id1.clone());
        nei.insert(other_id1.clone());
        let expected_neighborhood = vec![other_id1];
        assert_eq!(
            pwn.iter().cloned().collect::<Vec<_>>(),
            expected_neighborhood
        );
    }

    #[test]
    fn test_extend_neighborhood() {
        let peer_id: PeerId = "someone".into();
        let mut pwn = PeerWithNeighborhood::new(Peer::new(peer_id.clone(), Rc::from(vec![])));
        pwn.get_neighborhood_mut().insert("zero");
        pwn.extend_neighborhood(IntoIterator::into_iter(["one", "two"]));

        assert_eq!(
            PeerSet::from_iter(pwn.iter().cloned()),
            PeerSet::from_iter(IntoIterator::into_iter(["zero", "one", "two"]).map(PeerId::from)),
        );
    }

    #[test]
    fn test_remove_from_neiborhood() {
        let peer_id: PeerId = "someone".into();
        let mut pwn = PeerWithNeighborhood::new(Peer::new(peer_id.clone(), Rc::from(vec![])));
        pwn.get_neighborhood_mut().insert("zero");
        pwn.extend_neighborhood(IntoIterator::into_iter(["one", "two"]));
        pwn.remove_from_neighborhood(IntoIterator::into_iter(["zero", "two"]));

        assert_eq!(
            pwn.iter().cloned().collect::<HashSet<_>>(),
            IntoIterator::into_iter(["one"])
                .map(PeerId::from)
                .collect::<HashSet<_>>()
        );
    }
    #[test]
    fn test_fail() {
        let peer_id: PeerId = "someone".into();
        let other_id: PeerId = "other".into();
        let mut pwn = PeerWithNeighborhood::new(Peer::new(peer_id.clone(), Rc::from(vec![])));
        let nei = pwn.get_neighborhood_mut();
        nei.insert(other_id.clone());
        nei.fail(other_id.clone());

        let expected_neighborhood = PeerSet::from([other_id.clone()]);
        assert_eq!(
            PeerSet::from_iter(pwn.iter().cloned()),
            expected_neighborhood
        );
        assert!(!pwn.is_reachable(&other_id));
    }

    #[test]
    fn test_fail_remove() {
        let peer_id: PeerId = "someone".into();
        let other_id: PeerId = "other".into();
        let mut pwn = PeerWithNeighborhood::new(Peer::new(peer_id.clone(), Rc::from(vec![])));

        let nei = pwn.get_neighborhood_mut();
        nei.insert(other_id.clone());
        nei.fail(other_id.clone());
        assert!(!pwn.is_reachable(&other_id));

        let nei = pwn.get_neighborhood_mut();
        nei.remove(&other_id);
        assert!(!pwn.is_reachable(&other_id));

        let nei = pwn.get_neighborhood_mut();
        nei.insert(other_id.clone());
        assert!(pwn.is_reachable(&other_id));
    }

    #[test]
    fn test_fail_unfail() {
        let peer_id: PeerId = "someone".into();
        let other_id: PeerId = "other".into();
        let mut pwn = PeerWithNeighborhood::new(Peer::new(peer_id.clone(), Rc::from(vec![])));

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
        let mut pwn = PeerWithNeighborhood::new(Peer::new(peer_id.clone(), Rc::from(vec![])));

        let nei = pwn.get_neighborhood_mut();
        nei.fail(other_id.clone());
        assert!(!pwn.is_reachable(&other_id));

        let nei = pwn.get_neighborhood_mut();
        nei.unfail(&other_id);
        assert!(!pwn.is_reachable(&other_id));
    }

    #[test]
    fn test_failed() {
        let peer_id: PeerId = "someone".into();
        let other_id: PeerId = "other".into();
        let remote_id: PeerId = "remote".into();
        let mut pwn = PeerWithNeighborhood::new(Peer::new(peer_id.clone(), Rc::from(vec![])));
        pwn.get_neighborhood_mut().insert(other_id.clone());

        assert!(pwn.is_reachable(&peer_id));
        assert!(pwn.is_reachable(&other_id));
        assert!(!pwn.is_reachable(&remote_id));

        pwn.set_failed(true);
        assert!(!pwn.is_reachable(&peer_id));
        assert!(!pwn.is_reachable(&other_id));
        assert!(!pwn.is_reachable(&remote_id));

        pwn.set_failed(false);
        assert!(pwn.is_reachable(&peer_id));
        assert!(pwn.is_reachable(&other_id));
        assert!(!pwn.is_reachable(&remote_id));
    }
}
