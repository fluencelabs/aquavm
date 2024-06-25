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
use crate::JValue;

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

        let jvalue = match &result {
            JValue::Array(array) => &array[self.cursor],
            _ => unimplemented!("this jvalue is set only by fold instruction, so it must have an array type"),
        };

        let mut tetraplet = (*tetraplet).clone();
        tetraplet.add_lens(&format!(".$.[{}]", self.cursor));

        let result = IterableItem::RefValue((jvalue, tetraplet.into(), trace_pos, provenance));
        Some(result)
    }

    fn len(&self) -> usize {
        match self.call_result.get_result() {
            JValue::Array(array) => array.len(),
            _ => unimplemented!("this jvalue is set only by fold instruction, so it must have an array type"),
        }
    }
}
