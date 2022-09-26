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
#[serde(transparent)]
#[repr(transparent)]
pub struct TextPos(usize);

impl From<usize> for TextPos {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<TextPos> for usize {
    fn from(p: TextPos) -> Self {
        p.0
    }
}

impl Add<usize> for TextPos {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub<usize> for TextPos {
    type Output = TextPos;

    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Sub<TextPos> for TextPos {
    type Output = usize;

    fn sub(self, rhs: TextPos) -> Self::Output {
        self.0 - rhs.0
    }
}

impl PartialEq<usize> for TextPos {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

impl std::fmt::Display for TextPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
