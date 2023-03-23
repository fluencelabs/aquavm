/*
 * Copyright 2023 Fluence Labs Limited
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

use serde::Deserialize;
use serde::Serialize;

use std::cmp::Ordering;
use std::fmt::Debug;
use std::fmt::Display;

type GenerationIdxType = u32;

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct GenerationIdx(GenerationIdxType);

impl GenerationIdx {
    pub fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).map(Self)
    }

    pub fn checked_sub(self, other: Self) -> Option<Self> {
        self.0.checked_sub(other.0).map(Self)
    }

    pub fn next(self) -> Self {
        // TODO: check for overflow
        Self::from(self.0 as usize + 1)
    }

    pub fn prev(self) -> Self {
        // TODO: check for overflow
        Self::from(self.0 as usize - 1)
    }
}

impl PartialOrd<usize> for GenerationIdx {
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        let self_as_usize: usize = (*self).into();
        self_as_usize.partial_cmp(other)
    }
}

impl PartialEq<usize> for GenerationIdx {
    fn eq(&self, other: &usize) -> bool {
        let self_as_usize: usize = (*self).into();
        self_as_usize == *other
    }
}

//TODO: replace these two traits with try-* versions
impl From<usize> for GenerationIdx {
    fn from(value: usize) -> Self {
        GenerationIdx(value as u32)
    }
}

impl From<GenerationIdx> for usize {
    fn from(value: GenerationIdx) -> Self {
        value.0 as usize
    }
}

impl Debug for GenerationIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for GenerationIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
