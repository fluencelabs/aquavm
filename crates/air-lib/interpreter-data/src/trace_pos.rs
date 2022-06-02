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

use crate::{ExecutedState, ExecutionTrace};

use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Index, Sub},
};

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct TracePos(usize);

impl TracePos {
    pub fn checked_add(self, other: usize) -> Option<Self> {
        self.0.checked_add(other).map(Self)
    }
}

impl Debug for TracePos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for TracePos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl From<usize> for TracePos {
    fn from(pos: usize) -> Self {
        TracePos(pos)
    }
}

impl From<TracePos> for usize {
    fn from(pos: TracePos) -> Self {
        pos.0
    }
}

impl AddAssign<usize> for TracePos {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl Add<usize> for TracePos {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        TracePos(self.0 + rhs)
    }
}

impl Sub<usize> for TracePos {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        TracePos(self.0 - rhs)
    }
}

impl Sub<TracePos> for TracePos {
    type Output = usize;

    fn sub(self, rhs: TracePos) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Index<TracePos> for ExecutionTrace {
    type Output = ExecutedState;

    fn index(&self, index: TracePos) -> &Self::Output {
        &self[index.0]
    }
}
