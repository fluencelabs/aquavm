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
use super::ValueAggregate;
use crate::foldable_next;
use crate::foldable_prev;

/// Used for iterating over stream with JValues.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IterableVecResolvedCall {
    pub(crate) call_results: Vec<ValueAggregate>,
    pub(crate) cursor: usize,
}

impl IterableVecResolvedCall {
    pub(crate) fn init(call_results: Vec<ValueAggregate>) -> Self {
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

        let (result, tetraplet, trace_pos) = self.call_results[self.cursor].as_inner_parts();

        let result = IterableItem::RcValue((
            result.clone(),
            tetraplet,
            trace_pos,
            self.call_results[self.cursor].get_provenance(),
        ));
        Some(result)
    }

    fn len(&self) -> usize {
        self.call_results.len()
    }
}
