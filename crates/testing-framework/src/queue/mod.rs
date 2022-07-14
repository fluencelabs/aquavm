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

use std::{collections::BinaryHeap, time::Instant};

use crate::{ephemeral::PeerId, services::FunctionOutcome};

#[derive(Debug)]
pub(crate) struct Task {
    key: Instant,
    peer_id: PeerId,
    call_id: i32,
    result: FunctionOutcome,
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for Task {}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

/// Execution queue.
///
/// This struct simulates delayed execution of operations.
/// Many nodes can execute many calls of different time length in parallel, and real
/// execution time is expected to be shorter than simulated one.  So, we can
///
/// TODO: can we just use some async runtime?  I beleive we cannot, as we need fake time and
/// might like to clone the queue.
#[derive(Debug, Default)]
pub(crate) struct Queue {
    queue: BinaryHeap<Task>,
}

impl Queue {
    pub(crate) fn new() -> Self {
        Default::default()
    }

    pub(crate) fn enqueue(&mut self, task: Task) {
        self.queue.push(task)
    }

    pub(crate) fn fetch(&mut self) -> Option<Task> {
        self.queue.pop()
    }
}
