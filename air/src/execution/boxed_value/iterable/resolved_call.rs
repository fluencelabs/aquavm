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
use crate::contexts::execution::ResolvedCallResult;
use crate::foldable_next;
use crate::foldable_prev;
use crate::JValue;
use crate::SecurityTetraplet;

/// Used for iterating over JValue of array type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IterableResolvedCall {
    pub(crate) call_result: ResolvedCallResult,
    pub(crate) cursor: usize,
    pub(crate) len: usize,
}

impl IterableResolvedCall {
    pub(crate) fn init(call_result: ResolvedCallResult, len: usize) -> Self {
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
        use std::ops::Deref;

        if self.len == 0 {
            return None;
        }

        let triplet = self.call_result.triplet.clone();
        let tetraplet = SecurityTetraplet {
            triplet,
            // TODO: consider set json_path to the current cursor here
            json_path: String::new(),
        };

        let jvalue = match &self.call_result.result.deref() {
            JValue::Array(array) => &array[self.cursor],
            _ => unimplemented!("this jvalue is set only by fold instruction, so it must have an array type"),
        };

        let result = IterableItem::RefValue((jvalue, tetraplet));
        Some(result)
    }
}
