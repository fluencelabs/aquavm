/*
 * Copyright 2022 Fluence Labs Limited
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

use air_values::boxed_value::AIRIterableValueAlgebra;
use air_values::boxed_value::ValueAggregate;
use air_values::fold_iterable_state::IterableItem;
use boxed_value::foldable_next;
use boxed_value::foldable_prev;

/// Used for iterating over JValue of array type.
#[derive(Clone, Debug)]
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

const GET_BY_IDX_EXPECTATION: &str = "this jvalue is set only by fold instruction, so it must have an array type";

impl<'ctx> AIRIterableValueAlgebra<'ctx> for IterableResolvedCall {
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

        let ValueAggregate {
            value,
            tetraplet,
            trace_pos,
        } = &self.call_result;

        let value = value.get_by_idx(self.cursor).expect(GET_BY_IDX_EXPECTATION);
        let result = IterableItem::new(value, tetraplet, *trace_pos);
        Some(result)
    }

    fn len(&self) -> usize {
        self.len
    }
}
