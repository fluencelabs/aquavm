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

mod canon_stream;
mod canon_stream_map;
mod lambda_result;
mod resolved_call;
mod vec_resolved_call;

pub(crate) use canon_stream::CanonStreamIterableIngredients;
pub(crate) use canon_stream_map::CanonStreamMapIterableIngredients;
pub(crate) use lambda_result::IterableLambdaResult;
pub(crate) use resolved_call::IterableResolvedCall;
pub(crate) use vec_resolved_call::IterableVecResolvedCall;

use super::ValueAggregate;
use crate::execution_step::RcSecurityTetraplet;
use crate::JValue;

use air_interpreter_data::Provenance;
use air_interpreter_data::TracePos;

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
    RefValue((&'ctx JValue, RcSecurityTetraplet, TracePos, Provenance)),
    RcValue((JValue, RcSecurityTetraplet, TracePos, Provenance)),
}

impl IterableItem<'_> {
    pub(crate) fn pos(&self) -> TracePos {
        use IterableItem::*;

        let pos = match self {
            RefValue((.., pos, _)) => pos,
            RcValue((.., pos, _)) => pos,
        };

        *pos
    }

    pub(crate) fn provenance(&self) -> Provenance {
        use IterableItem::*;

        match self {
            RefValue((.., ref prov)) => prov,
            RcValue((.., ref prov)) => prov,
        }
        .clone()
    }

    pub(crate) fn into_resolved_result(self) -> ValueAggregate {
        use IterableItem::*;

        let (value, tetraplet, pos, provenance) = match self {
            RefValue((value, tetraplet, pos, prov)) => (value.clone(), tetraplet, pos, prov),
            RcValue(ingredients) => ingredients,
        };

        ValueAggregate::new(value, tetraplet, pos, provenance)
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
