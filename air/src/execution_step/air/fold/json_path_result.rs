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
use air_values::boxed_value::BoxedValue;
use air_values::boxed_value::RcSecurityTetraplet;
use air_values::fold_iterable_state::IterableItem;
use boxed_value::foldable_next;
use boxed_value::foldable_prev;

/// Used for iterating over a result of applied to a JValue lambda.
pub(crate) struct IterableLambdaResult<'ctx> {
    pub(crate) iterator: Box<dyn ExactSizeIterator<Item = &'ctx dyn BoxedValue>>,
    // consider adding index for each tetraplet
    pub(crate) tetraplet: RcSecurityTetraplet,
    pub(crate) cursor: usize,
}

impl IterableLambdaResult<'_> {
    pub(crate) fn init(
        iterator: Box<dyn ExactSizeIterator<Item = &dyn BoxedValue>>,
        tetraplet: RcSecurityTetraplet,
    ) -> Self {
        Self {
            iterator,
            tetraplet,
            cursor: 0,
        }
    }
}

impl<'ctx> AIRIterableValueAlgebra<'ctx> for IterableLambdaResult<'ctx> {
    type Item = IterableItem<'ctx>;

    fn next(&mut self) -> bool {
        foldable_next!(self, self.iterator.len())
    }

    fn prev(&mut self) -> bool {
        foldable_prev!(self)
    }

    fn peek(&'ctx self) -> Option<Self::Item> {
        if self.iterator.len() == 0 {
            return None;
        }

        let jvalue = &self.jvalues[self.cursor];
        let result = IterableItem::RefRef((jvalue, &self.tetraplet, 0));

        Some(result)
    }

    fn len(&self) -> usize {
        self.jvalues.len()
    }
}
