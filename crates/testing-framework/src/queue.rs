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

use air_test_utils::{test_runner::TestRunParameters, RawAVMOutcome};

use crate::ephemeral::{Data, Network, PeerId};

use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::{HashMap, VecDeque},
    hash::Hash,
    ops::Deref,
    rc::Rc,
};

#[derive(Debug, Default, Clone)]
pub(crate) struct PeerQueueCell {
    queue: Rc<RefCell<VecDeque<Data>>>,
}

impl PeerQueueCell {
    pub(crate) fn pop_data(&self) -> Option<Data> {
        let mut cell_ref = RefCell::borrow_mut(&self.queue);
        cell_ref.pop_front()
    }

    pub(crate) fn push_data(&self, data: Data) {
        let mut cell_ref = RefCell::borrow_mut(&self.queue);
        cell_ref.push_back(data);
    }
}

/// Per-particle message queue.
#[derive(Debug, Clone, Default)]
// TODO make it pub(crate) and see what is broken
pub(crate) struct ExecutionQueue {
    queues: Rc<RefCell<HashMap<PeerId, PeerQueueCell>>>,
}

impl ExecutionQueue {
    pub(crate) fn new() -> Self {
        Default::default()
    }

    pub(crate) fn get_peer_queue_cell(&self, peer_id: PeerId) -> PeerQueueCell {
        let mut queues_ref = RefCell::borrow_mut(&self.queues);
        queues_ref.entry(peer_id).or_default().clone()
    }

    /// Iterator for handling al the queued data.  It borrows peer env's `RefCell` only temporarily.
    /// Following test-utils' call_vm macro, it panics on failed VM.
    pub fn execution_iter<'s, Id>(
        &'s self,
        air: &'s str,
        network: Rc<Network>,
        test_parameters: &'s TestRunParameters,
        peer_id: &Id,
    ) -> Option<impl Iterator<Item = RawAVMOutcome> + 's>
    where
        PeerId: Borrow<Id>,
        Id: Eq + Hash + ?Sized,
    {
        let peer_env = network.get_peer_env(peer_id);

        peer_env.map(|peer_env_cell| {
            std::iter::from_fn(move || {
                let mut peer_env = RefCell::borrow_mut(&peer_env_cell);
                peer_env
                    .execute_once(air, &network, self, test_parameters)
                    .map(|r| r.unwrap_or_else(|err| panic!("VM call failed: {}", err)))
            })
        })
    }

    pub fn distribute_to_peers<Id>(&self, network: &Network, peers: &[Id], data: &Data)
    where
        Id: Deref<Target = str>,
    {
        for peer_id in peers {
            let peer_id: &str = peer_id;
            if let Some(peer_env_cell) = network.get_peer_env::<str>(peer_id) {
                let peer_env_ref = RefCell::borrow(&peer_env_cell);
                self.get_peer_queue_cell(peer_env_ref.peer.peer_id.clone())
                    .push_data(data.clone());
            }
        }
    }
}
