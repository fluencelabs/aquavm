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

/// This trait represent bidirectional iterator and
/// is used to abstract values used in fold as iterables.
#[allow(clippy::len_without_is_empty)]
pub trait AIRIterableValueAlgebra<'ctx> {
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
