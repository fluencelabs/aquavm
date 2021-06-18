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

pub(crate) use json_path_result::IterableJsonPathResult;
pub(crate) use resolved_call::IterableResolvedCall;
pub(crate) use vec_json_path_result::IterableVecJsonPathResult;
pub(crate) use vec_resolved_call::IterableVecResolvedCall;

use crate::execution_step::trace_handler::ValueAndPos;
use crate::JValue;
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
///
/// Iterable item is a variable that `fold` sets to each element of the collection it iterates
/// through, i.e., it is the `iterable` in the `(fold collection iterable instruction)` statement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum IterableItem<'ctx> {
    RefRef((&'ctx JValue, &'ctx SecurityTetraplet, usize)),
    RefValue((&'ctx JValue, SecurityTetraplet, usize)),
    RcValue((Rc<JValue>, SecurityTetraplet, usize)),
}

impl IterableItem<'_> {
    pub(crate) fn into_value_and_pos(self) -> ValueAndPos {
        use Self::*;

        // this method is called only from RcValue (in fold_stream and next),
        // so copying isn't actually happened here
        let (value, pos) = match self {
            RefRef((value, _, pos)) => (Rc::new(value.clone()), pos),
            RefValue((value, _, pos)) => (Rc::new(value.clone()), pos),
            RcValue((value, _, pos)) => (value, pos),
        };

        ValueAndPos { value, pos }
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
