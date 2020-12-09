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

use crate::air::ExecutionCtx;
use crate::AValue;
use crate::JValue;
use crate::SecurityTetraplet;

use std::cell::Ref;

pub(crate) trait Foldable<'ctx> {
    type Item;

    fn next(&mut self) -> bool;

    fn back(&mut self) -> bool;

    fn peek<'i>(&'ctx self, exec_ctx: &'ctx ExecutionCtx<'i>) -> Option<Self::Item>;
}

pub(crate) enum FoldableResult<'ctx> {
    Raw((&'ctx JValue, &'ctx SecurityTetraplet)),
    Ref((Ref<'ctx, JValue>, Ref<'ctx, SecurityTetraplet>)),
}

/// for RefCell<Vec<Rc<ExecutedCallResult>>> and Rc<ExecutedCallResult>
struct FoldableNamedResult {
    pub name: String,
    pub cursor: usize,
    pub len: usize,
}

struct FoldableJsonPathResult {
    pub jvalues: Vec<JValue>,
    pub tetraplet: SecurityTetraplet,
    pub cursor: usize,
    pub len: usize,
}

struct FoldableVecJsonPathResult {
    pub jvalues: Vec<JValue>,
    pub tetraplets: Vec<SecurityTetraplet>,
    pub cursor: usize,
    pub len: usize,
}

impl<'ctx> Foldable<'ctx> for FoldableNamedResult {
    type Item = FoldableResult<'ctx>;

    fn next(&mut self) -> bool {
        if self.cursor < self.len {
            self.cursor += 1;
            true
        } else {
            false
        }
    }

    fn back(&mut self) -> bool {
        if self.cursor != 0 {
            self.cursor -= 1;
            true
        } else {
            false
        }
    }

    fn peek<'i>(&'ctx self, exec_ctx: &'ctx ExecutionCtx<'i>) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        let result = match exec_ctx.data_cache.get(&self.name) {
            Some(AValue::JValueAccumulatorRef(acc)) => {
                let jvalue = Ref::map(acc.borrow(), |v| &v[self.cursor].result);
                let tetraplet = Ref::map(acc.borrow(), |v| &v[self.cursor].triplet);
                FoldableResult::Ref((jvalue, tetraplet))
            }
            Some(AValue::JValueRef(call_result)) => FoldableResult::Raw((&call_result.result, &call_result.triplet)),
            _ => unreachable!(),
        };

        Some(result)
    }
}

impl<'ctx> Foldable<'ctx> for FoldableJsonPathResult {
    type Item = FoldableResult<'ctx>;

    fn next(&mut self) -> bool {
        if self.cursor < self.len {
            self.cursor += 1;
            true
        } else {
            false
        }
    }

    fn back(&mut self) -> bool {
        if self.cursor != 0 {
            self.cursor -= 1;
            true
        } else {
            false
        }
    }

    fn peek<'i>(&'ctx self, _exec_ctx: &'ctx ExecutionCtx<'i>) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        let jvalue = &self.jvalues[self.cursor];
        let result = FoldableResult::Raw((jvalue, &self.tetraplet));

        Some(result)
    }
}

impl<'ctx> Foldable<'ctx> for FoldableVecJsonPathResult {
    type Item = FoldableResult<'ctx>;

    fn next(&mut self) -> bool {
        if self.cursor < self.len {
            self.cursor += 1;
            true
        } else {
            false
        }
    }

    fn back(&mut self) -> bool {
        if self.cursor != 0 {
            self.cursor -= 1;
            true
        } else {
            false
        }
    }

    fn peek<'i>(&'ctx self, _exec_ctx: &'ctx ExecutionCtx<'i>) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        let jvalue = &self.jvalues[self.cursor];
        let tetraplet = &self.tetraplets[self.cursor];
        let result = FoldableResult::Raw((jvalue, tetraplet));

        Some(result)
    }
}
