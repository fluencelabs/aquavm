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

use air_utils::auto_checked_add;

use newtype_derive::*;

use serde::Deserialize;
use serde::Serialize;

use std::convert::TryFrom;

pub type PosType = u32;

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive))]
#[cfg_attr(feature = "rkyv", archive(check_bytes))]
#[serde(transparent)]
#[repr(transparent)]
pub struct TracePos(PosType);

NewtypeFrom! { () pub struct TracePos(PosType); }
NewtypeAdd! { (PosType) pub struct TracePos(PosType); }
NewtypeAdd! { () pub struct TracePos(PosType); }
NewtypeAddAssign! { () pub struct TracePos(PosType); }
NewtypeAddAssign! { (PosType) pub struct TracePos(PosType); }
NewtypeSub! { () pub struct TracePos(PosType); }
NewtypeSub! { (PosType) pub struct TracePos(PosType); }
NewtypeDebug! { () pub struct TracePos(PosType); }
NewtypeDisplay! { () pub struct TracePos(PosType); }
auto_checked_add![TracePos];

impl From<TracePos> for usize {
    fn from(value: TracePos) -> Self {
        value.0 as Self
    }
}

impl TryFrom<usize> for TracePos {
    type Error = <PosType as TryFrom<usize>>::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        PosType::try_from(value).map(TracePos)
    }
}
