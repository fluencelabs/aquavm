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

use crate::JValue;
use crate::ResolvedCallResult;
use crate::SecurityTetraplet;

use std::rc::Rc;

/// This trait represent bidirectional iterator and
/// is used to abstract values used in fold as iterables.
pub(crate) trait Iterable<'ctx> {
    /// Represent iterable type.
    type Item;

    /// Move inner iterator to the next value and return true if it exists,
    /// does nothing and return false otherwise.
    fn next(&mut self) -> bool;

    /// Move inner iterator to the previous value and return true if it exists,
    /// does nothing and return false otherwise.
    fn prev(&mut self) -> bool;

    /// Return current iterable value if Iterable value is not empty and None otherwise.
    fn peek(&'ctx self) -> Option<Self::Item>;
}

/// Combines all possible iterable item types.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum IterableItem<'ctx> {
    RefRef((&'ctx JValue, &'ctx SecurityTetraplet)),
    RefValue((&'ctx JValue, SecurityTetraplet)),
    RcValue((Rc<JValue>, SecurityTetraplet)),
}

/// Used for iterating over JValue of array type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IterableResolvedCall {
    pub(crate) call_result: ResolvedCallResult,
    pub(crate) cursor: usize,
    pub(crate) len: usize,
}

impl IterableResolvedCall {
    pub(crate) fn init(call_result: ResolvedCallResult, len: usize) -> Self {
        Self {
            call_result,
            cursor: 0,
            len,
        }
    }
}

/// Used for iterating over accumulator with JValues.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IterableVecResolvedCall {
    pub(crate) call_results: Vec<ResolvedCallResult>,
    pub(crate) cursor: usize,
}

impl IterableVecResolvedCall {
    pub(crate) fn init(call_results: Vec<ResolvedCallResult>) -> Self {
        Self {
            call_results,
            cursor: 0,
        }
    }
}

/// Used for iterating over a result of applied to a JValue json path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IterableJsonPathResult {
    pub(crate) jvalues: Vec<JValue>,
    // consider adding index for each tetraplet
    pub(crate) tetraplet: SecurityTetraplet,
    pub(crate) cursor: usize,
}

impl IterableJsonPathResult {
    pub(crate) fn init(jvalues: Vec<JValue>, tetraplet: SecurityTetraplet) -> Self {
        Self {
            jvalues,
            tetraplet,
            cursor: 0,
        }
    }
}

/// Used for iterating over a result of applied to an accumulator json path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IterableVecJsonPathResult {
    pub(crate) jvalues: Vec<JValue>,
    pub(crate) tetraplets: Vec<SecurityTetraplet>,
    pub(crate) cursor: usize,
}

impl IterableVecJsonPathResult {
    pub(crate) fn init(jvalues: Vec<JValue>, tetraplets: Vec<SecurityTetraplet>) -> Self {
        // TODO: add assert on length
        Self {
            jvalues,
            tetraplets,
            cursor: 0,
        }
    }
}

macro_rules! foldable_next {
    ($self: expr, $len:expr) => {{
        if $self.cursor + 1 < $len {
            $self.cursor += 1;
            true
        } else {
            false
        }
    }};
}

macro_rules! foldable_prev {
    ($self: expr) => {{
        if $self.cursor >= 1 {
            $self.cursor -= 1;
            true
        } else {
            false
        }
    }};
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
        use std::ops::Deref;

        if self.len == 0 {
            return None;
        }

        let triplet = self.call_result.triplet.clone();
        let tetraplet = SecurityTetraplet {
            triplet,
            // TODO: consider set json_path to the current cursor here
            json_path: String::new(),
        };

        let jvalue = match &self.call_result.result.deref() {
            JValue::Array(array) => &array[self.cursor],
            _ => unimplemented!("this jvalue is set only by fold instruction, so it must have an array type"),
        };

        let result = IterableItem::RefValue((jvalue, tetraplet));
        Some(result)
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

        let ResolvedCallResult { result, triplet } = self.call_results[self.cursor].clone();
        let tetraplet = SecurityTetraplet {
            triplet,
            json_path: String::new(),
        };

        let result = IterableItem::RcValue((result, tetraplet));
        Some(result)
    }
}

impl<'ctx> Iterable<'ctx> for IterableJsonPathResult {
    type Item = IterableItem<'ctx>;

    fn next(&mut self) -> bool {
        foldable_next!(self, self.jvalues.len())
    }

    fn prev(&mut self) -> bool {
        foldable_prev!(self)
    }

    fn peek(&'ctx self) -> Option<Self::Item> {
        if self.jvalues.is_empty() {
            return None;
        }

        let jvalue = &self.jvalues[self.cursor];
        let result = IterableItem::RefRef((jvalue, &self.tetraplet));

        Some(result)
    }
}

impl<'ctx> Iterable<'ctx> for IterableVecJsonPathResult {
    type Item = IterableItem<'ctx>;

    fn next(&mut self) -> bool {
        foldable_next!(self, self.jvalues.len())
    }

    fn prev(&mut self) -> bool {
        foldable_prev!(self)
    }

    fn peek(&'ctx self) -> Option<Self::Item> {
        if self.jvalues.is_empty() {
            return None;
        }

        let jvalue = &self.jvalues[self.cursor];
        let tetraplet = &self.tetraplets[self.cursor];
        let result = IterableItem::RefRef((jvalue, tetraplet));

        Some(result)
    }
}
