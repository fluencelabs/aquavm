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

use super::foldable_next;
use super::foldable_prev;
use super::Iterable;

/// Used for iterating over accumulator with JValues.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IterableVecResolvedCall {
    pub(crate) call_results: Vec<ResolvedCallResult>,
    pub(crate) cursor: usize,
}

impl IterableVecResolvedCall {
    pub(crate) fn init(call_results: Vec<ResolvedCallResult>) -> Self {
        Self {
            call_results,
            cursor: 0,
        }
    }
}

impl<'ctx> Iterable<'ctx> for IterableVecResolvedCall {
    type Item = IterableItem<'ctx>;

    fn next(&mut self) -> bool {
        foldable_next!(self, self.call_results.len())
    }

    fn prev(&mut self) -> bool {
        foldable_prev!(self)
    }

    fn peek(&'ctx self) -> Option<Self::Item> {
        if self.call_results.is_empty() {
            return None;
        }

        let ResolvedCallResult { result, triplet } = self.call_results[self.cursor].clone();
        let tetraplet = SecurityTetraplet {
            triplet,
            json_path: String::new(),
        };

        let result = IterableItem::RcValue((result, tetraplet));
        Some(result)
    }
}
