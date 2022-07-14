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

use std::{
    cell::Cell,
    time::{Duration, Instant},
};

/// Virtual clock.
///
/// This is clock used by ephemeral network peers for their operations.
/// Unlike real clock, time can go here arbitrarily quickly.
#[derive(Debug)]
pub struct Clock {
    now: Cell<Instant>,
}

impl Clock {
    /// Initialize clock with current time.
    pub fn new() -> Self {
        Self {
            now: Cell::new(Instant::now()),
        }
    }

    /// Current clock's time.
    #[inline]
    pub fn now(&self) -> Instant {
        self.now.get()
    }

    #[inline]
    pub fn advance(&self, by: Duration) {
        self.now.replace(self.now.get() + by);
    }

    #[inline]
    pub fn set(&self, new_now: Instant) {
        assert!(self.now() <= new_now);

        self.now.replace(new_now);
    }
}
