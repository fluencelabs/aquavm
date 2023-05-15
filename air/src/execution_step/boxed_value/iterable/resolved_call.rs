/*
 * Copyright 2020 Fluence Labs Limited
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

use super::Iterable;
use super::IterableItem;
use super::ValueAggregate;
use crate::foldable_next;
use crate::foldable_prev;
use crate::JValue;

use std::ops::Deref;

/// Used for iterating over JValue of array type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IterableResolvedCall {
    pub(crate) call_result: ValueAggregate,
    pub(crate) cursor: usize,
    pub(crate) len: usize,
}

impl IterableResolvedCall {
    pub(crate) fn init(call_result: ValueAggregate, len: usize) -> Self {
        Self {
            call_result,
            cursor: 0,
            len,
        }
    }
}

impl<'ctx> Iterable<'ctx> for IterableResolvedCall {
    type Item = IterableItem<'ctx>;

    fn next(&mut self) -> bool {
        foldable_next!(self, self.len)
    }

    fn prev(&mut self) -> bool {
        foldable_prev!(self)
    }

    fn peek(&'ctx self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        let (result, tetraplet, trace_pos) = self.call_result.as_inner_parts();
        let provenance = self.call_result.get_provenance();

        let jvalue = match &result.deref() {
            JValue::Array(array) => &array[self.cursor],
            _ => unimplemented!("this jvalue is set only by fold instruction, so it must have an array type"),
        };

        let mut tetraplet = (*tetraplet).clone();
        tetraplet.add_lambda(&format!(".$.[{}]", self.cursor));

        let result = IterableItem::RefValue((jvalue, tetraplet.into(), trace_pos, provenance));
        Some(result)
    }

    fn len(&self) -> usize {
        match self.call_result.get_result().deref() {
            JValue::Array(array) => array.len(),
            _ => unimplemented!("this jvalue is set only by fold instruction, so it must have an array type"),
        }
    }
}
