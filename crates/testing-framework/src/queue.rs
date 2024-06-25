/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::ephemeral::{Data, Network, PeerId};

use air_test_utils::{
    test_runner::{AirRunner, TestRunParameters},
    RawAVMOutcome,
};

use futures::stream::StreamExt;

use std::pin::pin;
use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::{HashMap, VecDeque},
    future::Future,
    hash::Hash,
    ops::Deref,
    rc::Rc,
};

#[derive(Debug, Default)]
pub(crate) struct PeerQueueCell {
    queue: RefCell<VecDeque<Data>>,
    data: RefCell<Data>,
}

impl PeerQueueCell {
    pub(crate) fn pop_data(&self) -> Option<Data> {
        let mut cell_ref = self.queue.borrow_mut();
        cell_ref.pop_front()
    }

    pub(crate) fn push_data(&self, data: Data) {
        let mut cell_ref = self.queue.borrow_mut();
        cell_ref.push_back(data);
    }

    pub(crate) fn take_prev_data(&self) -> Data {
        let cell_ref = self.data.borrow_mut();
        (*cell_ref).clone()
    }

    pub(crate) fn set_prev_data(&self, data: Data) {
        let mut cell_ref = self.data.borrow_mut();
        *cell_ref = data;
    }
}

/// Per-particle message queue.
#[derive(Debug, Clone, Default)]
// TODO make it pub(crate) and see what is broken
pub(crate) struct ExecutionQueue {
    queues: Rc<RefCell<HashMap<PeerId, Rc<PeerQueueCell>>>>,
}

impl ExecutionQueue {
    pub(crate) fn new() -> Self {
        Default::default()
    }

    pub(crate) fn get_peer_queue_cell(&self, peer_id: PeerId) -> Rc<PeerQueueCell> {
        let mut queues_ref = RefCell::borrow_mut(&self.queues);
        queues_ref.entry(peer_id).or_default().clone()
    }

    /// Iterator for handling al the queued data.  It borrows peer env's `RefCell` only temporarily.
    /// Following test-utils' call_vm macro, it panics on failed VM.
    pub fn execution_iter<'ctx, Id, R: AirRunner + 'ctx>(
        &'ctx self,
        air: &'ctx str,
        network: Rc<Network<R>>,
        test_parameters: &'ctx TestRunParameters,
        peer_id: &Id,
    ) -> Option<impl futures::stream::Stream<Item = RawAVMOutcome> + 'ctx>
    where
        PeerId: Borrow<Id> + for<'a> From<&'a Id>,
        Id: Eq + Hash + ?Sized,
    {
        let peer_env = network.get_named_peer_env(peer_id);

        peer_env.map(|peer_env_cell| {
            futures::stream::poll_fn(move |ctx| {
                let mut peer_env = peer_env_cell.borrow_mut();
                let x = pin!(peer_env.execute_once(air, &network, self, test_parameters)).poll(ctx);
                x
            })
            .map(|r| r.unwrap_or_else(|err| panic!("VM call failed: {}", err)))
        })
    }

    pub fn distribute_to_peers<Id, R: AirRunner>(
        &self,
        network: &Network<R>,
        peers: &[Id],
        data: &Data,
    ) where
        Id: Deref<Target = str>,
    {
        for peer_id in peers {
            let peer_id: &str = peer_id;
            match network.get_peer_env(peer_id) {
                Some(peer_env_cell) => match peer_env_cell.try_borrow() {
                    Ok(peer_env_ref) => {
                        self.get_peer_queue_cell(peer_env_ref.peer.peer_id.clone())
                            .push_data(data.clone());
                    }
                    Err(_) => {
                        panic!("distributing data from peer to itself; probably, peer naming issue")
                    }
                },
                None => panic!("Unknown peer {:?}", peer_id),
            }
        }
    }
}
