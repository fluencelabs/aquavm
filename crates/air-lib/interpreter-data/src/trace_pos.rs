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

use air_utils::auto_checked_add;

use newtype_derive::*;

use serde::Deserialize;
use serde::Serialize;

pub type PosType = u32;

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize)]
#[archive(check_bytes)]
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
