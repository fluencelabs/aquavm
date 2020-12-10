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
use crate::ExecutedCallResult;

use std::rc::Rc;

pub(crate) trait Foldable<'ctx> {
    type Item;

    fn next(&mut self) -> bool;

    fn back(&mut self) -> bool;

    fn peek(&'ctx self) -> Option<Self::Item>;
}

pub(crate) enum FoldableResult<'ctx> {
    RefRef((&'ctx JValue, &'ctx SecurityTetraplet)),
    RefValue((&'ctx JValue, SecurityTetraplet)),
    RcValue((Rc<JValue>, SecurityTetraplet)),
}

pub(crate) struct FoldableRcResult {
    pub call_result: ExecutedCallResult,
    pub cursor: usize,
    pub len: usize,
}

pub(crate) struct FoldableVecRcResult {
    pub call_results: Vec<ExecutedCallResult>,
    pub cursor: usize,
    pub len: usize,
}

pub(crate) struct FoldableJsonPathResult {
    pub jvalues: Vec<JValue>,
    pub tetraplet: SecurityTetraplet,
    pub cursor: usize,
    pub len: usize,
}

pub(crate) struct FoldableVecJsonPathResult {
    pub jvalues: Vec<JValue>,
    pub tetraplets: Vec<SecurityTetraplet>,
    pub cursor: usize,
    pub len: usize,
}

macro_rules! foldable_next {
    ($self: expr) => {{
        if $self.cursor < $self.len {
            $self.cursor += 1;
            true
        } else {
            false
        }
    }}
}

macro_rules! foldable_prev {
    ($self: expr) => {{
        if $self.cursor != 0 {
            $self.cursor -= 1;
            true
        } else {
            false
        }
    }}
}

impl<'ctx> Foldable<'ctx> for FoldableRcResult {
    type Item = FoldableResult<'ctx>;

    fn next(&mut self) -> bool {
        foldable_next!(self)
    }

    fn back(&mut self) -> bool {
        foldable_prev!(self)
    }

    fn peek(&'ctx self) -> Option<Self::Item> {
        if self.len == 0  || self.cursor >= self.len {
            return None;
        }

        let triplet = self.call_result.triplet.clone();
        let tetraplet = SecurityTetraplet {
            triplet,
            json_path: self.len.into()
        };

        let jvalue = match &self.call_result.result {
            JValue::Array(array) => &array[self.cursor],
            _ => unimplemented!("this jvalue is set only by fold instruction, so it must have array type"),
        };

        let result = FoldableResult::RefValue((jvalue, tetraplet));
        Some(result)
    }
}

impl<'ctx> Foldable<'ctx> for FoldableVecRcResult {
    type Item = FoldableResult<'ctx>;

    fn next(&mut self) -> bool {
        foldable_next!(self)
    }

    fn back(&mut self) -> bool {
        foldable_prev!(self)
    }

    fn peek(&'ctx self) -> Option<Self::Item> {
        if self.len == 0  || self.cursor >= self.len {
            return None;
        }

        let ExecutedCallResult {result, triplet} = self.call_results[self.cursor].clone();
        let tetraplet = SecurityTetraplet {
            triplet,
            json_path: String::new()
        };

        let result = FoldableResult::RcValue((result, tetraplet));
        Some(result)
    }
}

impl<'ctx> Foldable<'ctx> for FoldableJsonPathResult {
    type Item = FoldableResult<'ctx>;

    fn next(&mut self) -> bool {
        foldable_next!(self)
    }

    fn back(&mut self) -> bool {
        foldable_prev!(self)
    }

    fn peek(&'ctx self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        let jvalue = &self.jvalues[self.cursor];
        let result = FoldableResult::RefRef((jvalue, &self.tetraplet));

        Some(result)
    }
}

impl<'ctx> Foldable<'ctx> for FoldableVecJsonPathResult {
    type Item = FoldableResult<'ctx>;

    fn next(&mut self) -> bool {
        foldable_next!(self)
    }

    fn back(&mut self) -> bool {
        foldable_prev!(self)
    }

    fn peek(&'ctx self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        let jvalue = &self.jvalues[self.cursor];
        let tetraplet = &self.tetraplets[self.cursor];
        let result = FoldableResult::RefRef((jvalue, tetraplet));

        Some(result)
    }
}
