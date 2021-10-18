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

mod json_path_result;
mod resolved_call;
mod vec_json_path_result;
mod vec_resolved_call;

pub(crate) use json_path_result::IterableLambdaResult;
pub(crate) use resolved_call::IterableResolvedCall;
pub(crate) use vec_resolved_call::IterableVecResolvedCall;

use super::ResolvedCallResult;
use crate::execution_step::RSecurityTetraplet;
use crate::JValue;

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

    /// Returns length of the current iterator.
    fn len(&self) -> usize;
}

/// Combines all possible iterable item types.
///
/// Iterable item is a variable that `fold` sets to each element of the collection it iterates
/// through, i.e., it is the `iterable` in the `(fold collection iterable instruction)` statement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum IterableItem<'ctx> {
    RefRef((&'ctx JValue, &'ctx RSecurityTetraplet, usize)),
    RefValue((&'ctx JValue, RSecurityTetraplet, usize)),
    RcValue((Rc<JValue>, RSecurityTetraplet, usize)),
}

impl IterableItem<'_> {
    pub(crate) fn pos(&self) -> usize {
        use IterableItem::*;

        let pos = match self {
            RefRef((.., pos)) => pos,
            RefValue((.., pos)) => pos,
            RcValue((.., pos)) => pos,
        };

        *pos
    }

    pub(crate) fn into_resolved_result(self) -> ResolvedCallResult {
        use IterableItem::*;

        let (value, tetraplet, pos) = match self {
            RefRef((value, tetraplet, pos)) => (Rc::new(value.clone()), tetraplet.clone(), pos),
            RefValue((value, tetraplet, pos)) => (Rc::new(value.clone()), tetraplet, pos),
            RcValue(ingredients) => ingredients,
        };

        ResolvedCallResult::new(value, tetraplet, pos)
    }
}

#[macro_export]
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

#[macro_export]
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
