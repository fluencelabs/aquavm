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

mod lambda_applier;
mod value;

pub use lambda_applier::LambdaApplier;
pub use lambda_applier::ValueLambdaError;
pub use value::Value;

use serde::Deserializer;
use serde::Serializer;

use std::fmt::Formatter;

pub trait BoxedValue: Value + LambdaApplier {}

impl PartialEq<Self> for dyn BoxedValue + '_ {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}

impl Eq for dyn BoxedValue + '_ {}

impl serde::Serialize for &(dyn BoxedValue + '_) {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!()
    }
}

impl<'de> serde::Deserialize<'de> for &(dyn BoxedValue + '_) {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}

impl std::fmt::Debug for dyn BoxedValue + '_ {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
