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

use serde::Deserialize;
use serde::Serialize;

use std::cmp::Ordering;
use std::fmt::Debug;
use std::fmt::Display;

type GenerationIdxType = u32;

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize)]
#[archive(check_bytes)]
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

    pub fn stub() -> Self {
        const GENERATION_STUB: GenerationIdxType = 0xCAFEBABE;
        Self(GENERATION_STUB)
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
