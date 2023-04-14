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
use crate::execution_step::boxed_value::CanonStream;
use crate::foldable_next;
use crate::foldable_prev;

const EXPECT_VALUE_IN_STREAM: &str = "value must exist, because length checked before creation and canonicalized stream can't be modified during iteration";

pub(crate) struct CanonStreamIterableIngredients {
    canon_stream: CanonStream,
    cursor: usize,
}

impl CanonStreamIterableIngredients {
    pub(crate) fn init(canon_stream: CanonStream) -> Self {
        Self {
            canon_stream,
            cursor: 0,
        }
    }
}

impl<'ctx> Iterable<'ctx> for CanonStreamIterableIngredients {
    type Item = IterableItem<'ctx>;

    fn next(&mut self) -> bool {
        foldable_next!(self, self.len())
    }

    fn prev(&mut self) -> bool {
        foldable_prev!(self)
    }

    fn peek(&'ctx self) -> Option<Self::Item> {
        if self.canon_stream.is_empty() {
            return None;
        }

        let value_w_prov = self.canon_stream.nth(self.cursor).expect(EXPECT_VALUE_IN_STREAM);
        let value = &**value_w_prov;
        let result = IterableItem::RefRef((
            &value.result,
            &value.tetraplet,
            value.trace_pos,
            value_w_prov.provenance.clone(),
        ));
        Some(result)
    }

    fn len(&self) -> usize {
        self.canon_stream.len()
    }
}
