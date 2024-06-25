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

use std::ops::{Add, Sub};

use serde::{Deserialize, Serialize};

/// Character position in the AIR script text.
#[derive(
    Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord,
)]
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
