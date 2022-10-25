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

use super::{Network, Peer, PeerId};
use crate::queue::ExecutionQueue;

use air_test_utils::test_runner::TestRunParameters;

use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::Deref,
};

pub(crate) type PeerSet = HashSet<PeerId>;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LinkState {
    Reachable,
    Unreachable,
}

/// Neighbors of particular node, including set of nodes unreachable from this one (but they might be
/// reachable from others).
#[derive(Debug, Default)]
pub struct Neighborhood {
    // the value is true is link from this peer to neighbor is failng
    neighbors: HashMap<PeerId, LinkState>,
}

impl Neighborhood {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_neighbors(&mut self, neighbors: PeerSet) {
        self.neighbors = neighbors
            .into_iter()
            .map(|peer_id| (peer_id, LinkState::Reachable))
            .collect();
    }

    pub fn iter(&self) -> impl Iterator<Item = &PeerId> {
        self.into_iter()
    }

    pub fn insert(&mut self, other_peer_id: impl Into<PeerId>) {
        let other_peer_id = other_peer_id.into();
        self.neighbors.insert(other_peer_id, LinkState::Reachable);
    }

    /// Removes the other_peer_id from neighborhood, also removes unreachable status.
    pub fn remove<Id>(&mut self, other_peer_id: &Id)
    where
        PeerId: Borrow<Id>,
        Id: Eq + Hash + ?Sized,
    {
        self.neighbors.remove(other_peer_id);
    }

    pub fn set_target_unreachable(&mut self, target: impl Into<PeerId>) {
        *self.neighbors.get_mut(&target.into()).unwrap() = LinkState::Unreachable;
    }

    pub fn unset_target_unreachable<Id>(&mut self, target: &Id)
    where
        PeerId: Borrow<Id>,
        Id: Eq + Hash + ?Sized,
    {
        *self.neighbors.get_mut(target).unwrap() = LinkState::Reachable;
    }

    pub fn is_reachable(&self, target: impl Deref<Target = PeerId>) -> bool {
        let target_peer_id = target.deref();
        self.neighbors.get(target_peer_id) == Some(&LinkState::Reachable)
    }
}

impl<'a> std::iter::IntoIterator for &'a Neighborhood {
    type Item = &'a PeerId;

    type IntoIter = std::collections::hash_map::Keys<'a, PeerId, LinkState>;

    fn into_iter(self) -> Self::IntoIter {
        self.neighbors.keys()
    }
}

#[derive(Debug)]
pub struct PeerEnv {
    pub(crate) peer: Peer,
    // failed for everyone
    failed: bool,
    neighborhood: Neighborhood,
}

impl PeerEnv {
    pub fn new(peer: Peer) -> Self {
        Self {
            peer,
            failed: false,
            neighborhood: Default::default(),
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

        let target_peer_id = target.deref();
        if &self.peer.peer_id == target_peer_id {
            return true;
        }

        self.neighborhood.is_reachable(target)
    }

    pub fn extend_neighborhood(&mut self, peers: impl Iterator<Item = impl Into<PeerId>>) {
        let peer_id = self.peer.peer_id.clone();
        for other_peer_id in peers
            .map(Into::into)
            .filter(|other_id| other_id != &peer_id)
        {
            self.neighborhood.insert(other_peer_id);
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

    pub(crate) fn execute_once(
        &mut self,
        air: impl Into<String>,
        network: &Network,
        queue: &ExecutionQueue,
        test_parameters: &TestRunParameters,
    ) -> Option<Result<air_test_utils::RawAVMOutcome, String>> {
        let queue = queue.clone();
        let maybe_data = queue
            .get_peer_queue_cell(self.peer.peer_id.clone())
            .pop_data();

        maybe_data.map(|data| {
            let res = self.peer.invoke(air, data, test_parameters.clone());

            if let Ok(outcome) = &res {
                queue.distribute_to_peers(network, &outcome.next_peer_pks, &outcome.data)
            }

            res
        })
    }
}

impl<'a> IntoIterator for &'a PeerEnv {
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
        let pwn = PeerEnv::new(Peer::new(peer_id.clone(), Rc::from(vec![])));
        assert!(pwn.is_reachable(&peer_id));
        assert!(!pwn.is_reachable(&other_id));
    }

    #[test]
    fn test_no_self_disconnect() {
        let peer_id: PeerId = "someone".into();
        let other_id: PeerId = "other".into();
        let mut pwn = PeerEnv::new(Peer::new(peer_id.clone(), Rc::from(vec![])));
        let nei = pwn.get_neighborhood_mut();
        nei.insert(peer_id.clone());
        nei.remove(&peer_id);
        assert!(pwn.is_reachable(&peer_id));
        assert!(!pwn.is_reachable(&other_id));
    }

    #[test]
    fn test_set_neighborhood() {
        let peer_id: PeerId = "someone".into();
        let other_id1: PeerId = "other1".into();
        let other_id2: PeerId = "other2".into();
        let mut pwn = PeerEnv::new(Peer::new(peer_id.clone(), Rc::from(vec![])));

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
        let mut pwn = PeerEnv::new(Peer::new(peer_id.clone(), Rc::from(vec![])));

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
        let mut pwn = PeerEnv::new(Peer::new(peer_id.clone(), Rc::from(vec![])));

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
        let mut pwn = PeerEnv::new(Peer::new(peer_id.clone(), Rc::from(vec![])));
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
        let mut pwn = PeerEnv::new(Peer::new(peer_id.clone(), Rc::from(vec![])));
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
        let mut pwn = PeerEnv::new(Peer::new(peer_id.clone(), Rc::from(vec![])));
        let nei = pwn.get_neighborhood_mut();
        nei.insert(other_id.clone());
        nei.set_target_unreachable(other_id.clone());

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
        let mut pwn = PeerEnv::new(Peer::new(peer_id.clone(), Rc::from(vec![])));

        let nei = pwn.get_neighborhood_mut();
        nei.insert(other_id.clone());
        nei.set_target_unreachable(other_id.clone());
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
        let mut pwn = PeerEnv::new(Peer::new(peer_id.clone(), Rc::from(vec![])));

        let nei = pwn.get_neighborhood_mut();
        nei.insert(other_id.clone());
        nei.set_target_unreachable(other_id.clone());
        assert!(!pwn.is_reachable(&other_id));

        let nei = pwn.get_neighborhood_mut();
        nei.unset_target_unreachable(&other_id);
        assert!(pwn.is_reachable(&other_id));
    }

    #[test]
    fn test_failed() {
        let peer_id: PeerId = "someone".into();
        let other_id: PeerId = "other".into();
        let remote_id: PeerId = "remote".into();
        let mut pwn = PeerEnv::new(Peer::new(peer_id.clone(), Rc::from(vec![])));
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
