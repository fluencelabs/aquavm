/*
 * Copyright 2021 Fluence Labs Limited
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

use std::ops::{Add, Sub};

use serde::{Deserialize, Serialize};

/// Character position in the AIR script text.
#[derive(
    Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord,
)]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive))]
#[cfg_attr(feature = "rkyv", archive(check_bytes))]
#[serde(transparent)]
#[repr(transparent)]
pub struct AirPos(usize);

impl From<usize> for AirPos {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<AirPos> for usize {
    fn from(p: AirPos) -> Self {
        p.0
    }
}

impl Add<usize> for AirPos {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub<usize> for AirPos {
    type Output = AirPos;

    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Sub<AirPos> for AirPos {
    type Output = usize;

    fn sub(self, rhs: AirPos) -> Self::Output {
        self.0 - rhs.0
    }
}

impl PartialEq<usize> for AirPos {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

impl std::fmt::Display for AirPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
