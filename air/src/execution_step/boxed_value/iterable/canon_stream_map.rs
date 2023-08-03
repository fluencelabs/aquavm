/*
 * Copyright 2023 Fluence Labs Limited
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
use crate::execution_step::ValueAggregate;
use crate::foldable_next;
use crate::foldable_prev;

const EXPECT_VALUE_IN_STREAM: &str = "value must exist, because length checked before creation and canonicalized stream map can not be modified during iteration";

pub(crate) struct CanonStreamMapIterableIngredients {
    values: Vec<ValueAggregate>,
    cursor: usize,
}

impl CanonStreamMapIterableIngredients {
    pub(crate) fn init(values: Vec<ValueAggregate>) -> Self {
        Self { values, cursor: 0 }
    }
}

impl<'ctx> Iterable<'ctx> for CanonStreamMapIterableIngredients {
    type Item = IterableItem<'ctx>;

    fn next(&mut self) -> bool {
        foldable_next!(self, self.len())
    }

    fn prev(&mut self) -> bool {
        foldable_prev!(self)
    }

    fn peek(&'ctx self) -> Option<Self::Item> {
        if self.values.is_empty() {
            return None;
        }

        let value = self.values.get(self.cursor).expect(EXPECT_VALUE_IN_STREAM);
        let result = IterableItem::RefValue((
            value.get_result(),
            value.get_tetraplet(),
            value.get_trace_pos(),
            value.get_provenance(),
        ));
        Some(result)
    }

    fn len(&self) -> usize {
        self.values.len()
    }
}
