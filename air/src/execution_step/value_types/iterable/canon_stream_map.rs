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
use crate::execution_step::value_types::TracePosOperate;
use crate::execution_step::ValueAggregate;
use crate::foldable_next;
use crate::foldable_prev;

const EXPECT_VALUE_IN_MAP: &str = "value must exist, because length checked before creation and canonicalized stream map can not be modified during iteration";

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

        let value = self.values.get(self.cursor).expect(EXPECT_VALUE_IN_MAP);
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
