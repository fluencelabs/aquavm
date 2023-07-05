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

// use air_parser::ast::CanonStreamMap;

use super::Iterable;
use super::IterableItem;
use crate::execution_step::boxed_value::CanonStreamMap;
use crate::foldable_next;
use crate::foldable_prev;

const EXPECT_VALUE_IN_STREAM: &str = "value must exist, because length checked before creation and canonicalized stream can't be modified during iteration";

pub(crate) struct CanonStreamMapIterableIngredients<'a> {
    canon_stream_map: CanonStreamMap<'a>,
    cursor: usize,
}

// impl<'a, 'b: 'a> CanonStreamMapIterableIngredients<'a> {
//     pub(crate) fn init(canon_stream_map: CanonStreamMap<'b>) -> Self {
//         Self {
//             canon_stream_map,
//             cursor: 0,
//         }
//     }
// }

impl<'ctx> Iterable<'ctx> for CanonStreamMapIterableIngredients<'_> {
    type Item = IterableItem<'ctx>;

    fn next(&mut self) -> bool {
        foldable_next!(self, self.len())
    }

    fn prev(&mut self) -> bool {
        foldable_prev!(self)
    }

    fn peek(&'ctx self) -> Option<Self::Item> {
        if self.canon_stream_map.is_empty() {
            return None;
        }

        let value = self.canon_stream_map.nth(self.cursor).expect(EXPECT_VALUE_IN_STREAM);
        let result = IterableItem::RefValue((
            value.get_result(),
            value.get_tetraplet(),
            value.get_trace_pos(),
            value.get_provenance(),
        ));
        Some(result)
    }

    fn len(&self) -> usize {
        self.canon_stream_map.len()
    }
}
