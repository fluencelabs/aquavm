/*
 * Copyright 2021 Fluence Labs Limited
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

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Span {
    pub left: usize,
    pub right: usize,
}

impl Span {
    pub fn new(left: usize, right: usize) -> Self {
        Self { left, right }
    }

    pub fn contains_position(&self, position: usize) -> bool {
        self.left < position && position < self.right
    }

    pub fn contains_span(&self, span: Self) -> bool {
        self.contains_position(span.left) && self.contains_position(span.right)
    }
}

use std::cmp::Ordering;

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_min = std::cmp::min(self.left, self.right);
        let other_min = std::cmp::min(other.left, other.right);

        if self_min < other_min {
            Some(Ordering::Less)
        } else if self == other {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl Ord for Span {
    fn cmp(&self, other: &Self) -> Ordering {
        // it's safe since partial_cmp always returns Some
        self.partial_cmp(other).unwrap()
    }
}
