/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use super::Iterable;
use super::IterableItem;
use crate::execution_step::value_types::CanonStream;
use crate::execution_step::value_types::TracePosOperate;
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

        let value = self.canon_stream.nth(self.cursor).expect(EXPECT_VALUE_IN_STREAM);
        let result = IterableItem::RefValue((
            value.get_result(),
            value.get_tetraplet(),
            value.get_trace_pos(),
            value.get_provenance(),
        ));
        Some(result)
    }

    fn len(&self) -> usize {
        self.canon_stream.len()
    }
}
