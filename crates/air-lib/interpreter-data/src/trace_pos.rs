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

pub type PosType = u32;

macro_rules! auto_checked_add {
    [$type:ty] => {
        impl ::num_traits::CheckedAdd for $type {
            fn checked_add(&self, other: &Self) -> Option<Self> {
                self.0.checked_add(other.0).map(Self)
            }
        }
    };
}

macro_rules! auto_wrapper_types_from {
    [$dst_type:ident, $wrapped_type:ty, $src_type:ty ] => {
        impl ::std::convert::From<$src_type> for $dst_type {
            fn from(value: $src_type) -> Self {
                $dst_type(value as $wrapped_type) // ? saturation semantics ?
            }
        }
    };
}

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
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
auto_wrapper_types_from![TracePos, PosType, usize];

impl From<TracePos> for usize {
    fn from(value: TracePos) -> Self {
        value.0 as Self // ? saturation semantics ?
    }
}
