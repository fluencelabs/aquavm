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

mod air_value;
mod lambda_applier;
mod to_iterable;

use std::fmt::Formatter;
use erased_serde::private::serde;
pub use air_value::Value;
pub use lambda_applier::LambdaApplier;
pub use to_iterable::ToIterable;

use erased_serde::Serialize;
use crate::serde::{Deserializer, Serializer};

pub trait BoxedValue: Value + LambdaApplier + ToIterable {}

impl PartialEq<Self> for dyn BoxedValue + '_ {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for dyn BoxedValue + '_ {}

impl serde::Serialize for &(dyn BoxedValue + '_) {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        todo!()
    }
}

impl<'de> serde::Deserialize<'de> for &(dyn BoxedValue + '_) {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
    {
        todo!()
    }
}

impl std::fmt::Debug for dyn BoxedValue + '_ {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
