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

use super::Stream;
use super::ValueAggregate;
use crate::execution_step::Generation;
use crate::JValue;

use std::fmt::Formatter;

#[derive(Debug, Default, Clone)]
pub struct CanonStream(Vec<ValueAggregate>);

impl CanonStream {
    pub(crate) fn new(values: Vec<ValueAggregate>) -> Self {
        Self(values)
    }

    pub(crate) fn from_stream(stream: &Stream) -> Self {
        let values = stream.iter(Generation::Last).unwrap().cloned().collect::<Vec<_>>();
        CanonStream(values)
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn as_jvalue(&self) -> JValue {
        use std::ops::Deref;

        // TODO: this clone will be removed after boxed values
        let jvalue_array = self.0.iter().map(|r| r.result.deref().clone()).collect::<Vec<_>>();
        JValue::Array(jvalue_array)
    }

    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = &ValueAggregate> {
        self.0.iter()
    }
}

use std::fmt;

impl fmt::Display for CanonStream {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "#[{}]", self.0.join(", "))
    }
}
