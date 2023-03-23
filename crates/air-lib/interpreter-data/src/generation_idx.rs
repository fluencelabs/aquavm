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

 use serde::{Deserialize, Serialize};
 use std::convert::TryFrom;
 use std::{
     fmt::{Debug, Display},
     ops::{Add, AddAssign, Sub},
 };
 
 #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
 #[serde(transparent)]
 #[repr(transparent)]
pub struct GenerationIdx(u32);

impl GenerationIdx {
   pub fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).map(Self)
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

impl From<u32> for GenerationIdx {
    fn from(pos: u32) -> Self {
        GenerationIdx(pos)
    }
}

impl TryFrom<usize> for GenerationIdx {
    type Error = <u32 as TryFrom<usize>>::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        u32::try_from(value).map(GenerationIdx)
    }
}

impl AddAssign<u32> for GenerationIdx {
    fn add_assign(&mut self, rhs: u32) {
        self.0 += rhs;
    }
}

impl Add<u32> for GenerationIdx {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        GenerationIdx(self.0 + rhs)
    }
}

impl Sub<u32> for GenerationIdx {
    type Output = Self;

    fn sub(self, rhs: u32) -> Self::Output {
        GenerationIdx(self.0 - rhs)
    }
}

impl Sub<GenerationIdx> for GenerationIdx {
    type Output = u32;

    fn sub(self, rhs: GenerationIdx) -> Self::Output {
        self.0 - rhs.0
    }
}
