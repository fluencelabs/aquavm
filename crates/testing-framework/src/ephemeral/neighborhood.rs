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
    rc::{Rc, Weak},
};

const EXPECT_VALID_NETWORK: &str = "Using a peer of a destroyed network";

pub(crate) type PeerSet = HashSet<PeerId>;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum AlterState {
    Added,
    Removed,
}

/// Neighbors of particular node, including set of nodes unreachable from this one (but they might be
/// reachable from others).
pub struct Neighborhood {
    // the value is true is link from this peer to neighbor is failng
    network: Weak<Network>,
    unreachable: HashSet<PeerId>,
    altered: HashMap<PeerId, AlterState>,
}

impl Neighborhood {
    pub fn new(network: &Rc<Network>) -> Self {
        Self {
            network: Rc::downgrade(network),
            unreachable: <_>::default(),
            altered: <_>::default(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = PeerId> {
        self.into_iter()
    }

    pub fn alter(&mut self, other_peer_id: impl Into<PeerId>, state: AlterState) {
        let other_peer_id = other_peer_id.into();

        self.altered.insert(other_peer_id, state);
    }

    pub fn unalter<Id>(&mut self, other_peer_id: &Id)
    where
        PeerId: Borrow<Id>,
        Id: Eq + Hash + ?Sized,
    {
        self.altered.remove(other_peer_id);
    }

    pub fn get_alter_state<Id>(&self, other_peer_id: &Id) -> Option<AlterState>
    where
        PeerId: Borrow<Id>,
        Id: Eq + Hash + ?Sized,
    {
        self.altered.get(other_peer_id).copied()
    }

    pub fn set_target_unreachable(&mut self, target: impl Into<PeerId>) {
        self.unreachable.insert(target.into());
    }

    pub fn unset_target_unreachable<Id>(&mut self, target: &Id)
    where
        PeerId: Borrow<Id>,
        Id: Eq + Hash + ?Sized,
    {
        self.unreachable.remove(target);
    }

    pub fn is_reachable<Id>(&self, target: &Id) -> bool
    where
        PeerId: Borrow<Id>,
        Id: Eq + Hash + ?Sized,
    {
        let network = self.network.upgrade().expect(EXPECT_VALID_NETWORK);
        if network.get_peer_env(target).is_some()
            || self.altered.get(target) == Some(&AlterState::Added)
        {
            !self.unreachable.contains(target)
        } else {
            false
        }
    }
}

impl std::iter::IntoIterator for &Neighborhood {
    type Item = PeerId;

    type IntoIter = std::collections::hash_set::IntoIter<PeerId>;

    fn into_iter(self) -> Self::IntoIter {
        let network = self.network.upgrade().expect(EXPECT_VALID_NETWORK);
        let mut peers: HashSet<_> = network
            .get_peers()
            .filter(|peer| self.altered.get(peer) != Some(&AlterState::Removed))
            .collect();
        for (peer, &state) in self.altered.iter() {
            if state == AlterState::Added {
                peers.insert(peer.clone());
            }
        }
        peers.into_iter()
    }
}

pub struct PeerEnv {
    pub(crate) peer: Peer,
    // failed for everyone
    failed: bool,
    neighborhood: Neighborhood,
}

impl PeerEnv {
    pub fn new(peer: Peer, network: &Rc<Network>) -> Self {
        Self {
            peer,
            failed: false,
            neighborhood: Neighborhood::new(network),
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

        self.neighborhood.is_reachable(target_peer_id)
    }

    pub fn extend_neighborhood(&mut self, peers: impl Iterator<Item = impl Into<PeerId>>) {
        let peer_id = &self.peer.peer_id;
        for other_peer_id in peers.map(Into::into).filter(|other_id| other_id != peer_id) {
            self.neighborhood.alter(other_peer_id, AlterState::Added);
        }
    }

    pub fn remove_from_neighborhood(&mut self, peers: impl Iterator<Item = impl Into<PeerId>>) {
        let peer_id = &self.peer.peer_id;
        for other_peer_id in peers.map(Into::into).filter(|other_id| other_id != peer_id) {
            self.neighborhood.alter(other_peer_id, AlterState::Removed);
        }
    }

    pub fn get_neighborhood(&self) -> &Neighborhood {
        &self.neighborhood
    }

    pub fn get_neighborhood_mut(&mut self) -> &mut Neighborhood {
        &mut self.neighborhood
    }

    pub fn iter(&self) -> impl Iterator<Item = PeerId> {
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
        let queue_cell = queue.get_peer_queue_cell(self.peer.peer_id.clone());
        let maybe_data = queue_cell.pop_data();

        maybe_data.map(|data| {
            let res = self
                .peer
                .invoke(air, data, test_parameters.clone(), &queue_cell);

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
        let network = Network::empty();
        let peer_id: PeerId = "someone".into();
        let other_id: PeerId = "other".into();
        let penv = PeerEnv::new(Peer::new(peer_id.clone(), Rc::from(vec![])), &network);
        assert!(penv.is_reachable(&peer_id));
        assert!(!penv.is_reachable(&other_id));
    }

    #[test]
    fn test_no_self_disconnect() {
        let network = Network::empty();
        let peer_id: PeerId = "someone".into();
        let other_id: PeerId = "other".into();
        let mut penv = PeerEnv::new(Peer::new(peer_id.clone(), Rc::from(vec![])), &network);
        {
            let nei = penv.get_neighborhood_mut();

            nei.alter(peer_id.clone(), AlterState::Added);
            nei.alter(peer_id.clone(), AlterState::Removed);
        }
        assert!(penv.is_reachable(&peer_id));
        assert!(!penv.is_reachable(&other_id));

        let nei = penv.get_neighborhood_mut();
        nei.unalter(&peer_id);
        assert!(penv.is_reachable(&peer_id));
        assert!(!penv.is_reachable(&other_id));
    }

    #[test]
    fn test_set_neighborhood() {
        let peer_id: PeerId = "someone".into();
        let other_id1: PeerId = "other1".into();
        let other_id2: PeerId = "other2".into();
        let network = Network::empty();

        let penv = PeerEnv::new(Peer::new(peer_id, Rc::from(vec![])), &network);
        // iter is empty
        assert!(penv.iter().next().is_none());

        network.ensure_peer(other_id1.clone());
        network.ensure_peer(other_id2.clone());
        let expected_neighborhood = PeerSet::from([other_id1, other_id2]);
        assert_eq!(penv.iter().collect::<PeerSet>(), expected_neighborhood);
    }

    #[test]
    fn test_insert() {
        let network = Network::empty();
        let peer_id: PeerId = "someone".into();
        let other_id1: PeerId = "other1".into();
        let other_id2: PeerId = "other2".into();
        let penv = PeerEnv::new(Peer::new(peer_id, Rc::from(vec![])), &network);

        // iter is empty
        assert!(penv.iter().next().is_none());

        network.ensure_peer(other_id1.clone());
        network.ensure_peer(other_id2.clone());
        let expected_neighborhood = PeerSet::from([other_id1, other_id2]);
        assert_eq!(PeerSet::from_iter(penv.iter()), expected_neighborhood);
    }

    #[test]
    fn test_ensure() {
        let network = Network::empty();
        let peer_id: PeerId = "someone".into();
        let other_id1: PeerId = "other1".into();
        let other_id2: PeerId = "other2".into();
        let mut penv = PeerEnv::new(Peer::new(peer_id, Rc::from(vec![])), &network);

        // iter is empty
        assert!(penv.iter().next().is_none());
        let nei = penv.get_neighborhood_mut();
        nei.alter(other_id1.clone(), AlterState::Added);
        nei.alter(other_id2.clone(), AlterState::Added);

        let expected_neighborhood = PeerSet::from([other_id1, other_id2]);
        assert_eq!(PeerSet::from_iter(penv.iter()), expected_neighborhood);
    }

    #[test]
    fn test_insert_insert() {
        let network = Network::empty();
        let peer_id: PeerId = "someone".into();
        let other_id1: PeerId = "other1".into();
        let mut penv = PeerEnv::new(Peer::new(peer_id, Rc::from(vec![])), &network);

        // iter is empty
        assert!(penv.iter().next().is_none());

        let nei = penv.get_neighborhood_mut();
        nei.alter(other_id1.clone(), AlterState::Added);
        nei.alter(other_id1.clone(), AlterState::Added);

        let expected_neighborhood = vec![other_id1];
        assert_eq!(penv.iter().collect::<Vec<_>>(), expected_neighborhood);
    }

    #[test]
    fn test_extend_neighborhood() {
        let network = Network::empty();
        let peer_id: PeerId = "someone".into();
        let mut penv = PeerEnv::new(Peer::new(peer_id, Rc::from(vec![])), &network);
        penv.get_neighborhood_mut()
            .alter(PeerId::from("zero"), AlterState::Added);
        penv.extend_neighborhood(IntoIterator::into_iter(["one", "two"]));

        assert_eq!(
            PeerSet::from_iter(penv.iter()),
            PeerSet::from_iter(IntoIterator::into_iter(["zero", "one", "two"]).map(PeerId::from)),
        );
    }

    #[test]
    fn test_remove_from_neiborhood() {
        let network = Network::empty();
        let peer_id: PeerId = "someone".into();
        let mut penv = PeerEnv::new(Peer::new(peer_id, Rc::from(vec![])), &network);
        penv.get_neighborhood_mut()
            .alter(PeerId::from("zero"), AlterState::Added);
        penv.extend_neighborhood(IntoIterator::into_iter(["one", "two"]));
        penv.remove_from_neighborhood(IntoIterator::into_iter(["zero", "two"]));

        assert_eq!(
            penv.iter().collect::<PeerSet>(),
            maplit::hashset! {
                PeerId::from("one"),
            },
        );
    }
    #[test]
    fn test_fail() {
        let network = Network::empty();
        let peer_id: PeerId = "someone".into();
        let other_id: PeerId = "other".into();
        let mut penv = PeerEnv::new(Peer::new(peer_id, Rc::from(vec![])), &network);

        let nei = penv.get_neighborhood_mut();
        nei.alter(other_id.clone(), AlterState::Added);
        nei.set_target_unreachable(other_id.clone());

        let expected_neighborhood = PeerSet::from([other_id.clone()]);
        assert_eq!(PeerSet::from_iter(penv.iter()), expected_neighborhood);
        assert!(!penv.is_reachable(&other_id));
    }

    #[test]
    fn test_fail_remove() {
        let network = Network::empty();
        let peer_id: PeerId = "someone".into();
        let other_id: PeerId = "other".into();
        let mut penv = PeerEnv::new(Peer::new(peer_id, Rc::from(vec![])), &network);

        let nei = penv.get_neighborhood_mut();
        nei.alter(other_id.clone(), AlterState::Added);
        nei.set_target_unreachable(other_id.clone());
        assert!(!penv.is_reachable(&other_id));

        let nei = penv.get_neighborhood_mut();
        nei.unalter(&other_id);
        assert!(!penv.is_reachable(&other_id));

        let nei = penv.get_neighborhood_mut();
        nei.alter(other_id.clone(), AlterState::Added);
        assert!(!penv.is_reachable(&other_id));
    }

    #[test]
    fn test_fail_unfail() {
        let network = Network::empty();
        let peer_id: PeerId = "someone".into();
        let other_id: PeerId = "other".into();
        let mut penv = PeerEnv::new(Peer::new(peer_id, Rc::from(vec![])), &network);

        let nei = penv.get_neighborhood_mut();
        nei.alter(other_id.clone(), AlterState::Added);
        nei.set_target_unreachable(other_id.clone());
        assert!(!penv.is_reachable(&other_id));

        let nei = penv.get_neighborhood_mut();
        nei.unset_target_unreachable(&other_id);
        assert!(penv.is_reachable(&other_id));
    }

    #[test]
    fn test_failed() {
        let network = Network::empty();
        let peer_id: PeerId = "someone".into();
        let other_id: PeerId = "other".into();
        let remote_id: PeerId = "remote".into();
        let mut penv = PeerEnv::new(Peer::new(peer_id.clone(), Rc::from(vec![])), &network);
        penv.get_neighborhood_mut()
            .alter(other_id.clone(), AlterState::Added);

        assert!(penv.is_reachable(&peer_id));
        assert!(penv.is_reachable(&other_id));
        assert!(!penv.is_reachable(&remote_id));

        penv.set_failed(true);
        assert!(!penv.is_reachable(&peer_id));
        assert!(!penv.is_reachable(&other_id));
        assert!(!penv.is_reachable(&remote_id));

        penv.set_failed(false);
        assert!(penv.is_reachable(&peer_id));
        assert!(penv.is_reachable(&other_id));
        assert!(!penv.is_reachable(&remote_id));
    }
}
