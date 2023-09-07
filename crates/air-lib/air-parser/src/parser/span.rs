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

use super::lexer::AirPos;

use serde::Deserialize;
use serde::Serialize;

use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Span {
    pub left: AirPos,
    pub right: AirPos,
}

impl Span {
    pub fn new(left: AirPos, right: AirPos) -> Self {
        Self { left, right }
    }

    pub fn contains_position(&self, position: AirPos) -> bool {
        self.left < position && position < self.right
    }

    pub fn contains_span(&self, span: Self) -> bool {
        self.contains_position(span.left) && self.contains_position(span.right)
    }
}

impl From<Range<AirPos>> for Span {
    fn from(range: Range<AirPos>) -> Self {
        Self {
            left: range.start,
            right: range.end,
        }
    }
}

use std::cmp::Ordering;

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Span {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_min = std::cmp::min(self.left, self.right);
        let other_min = std::cmp::min(other.left, other.right);

        if self_min < other_min {
            Ordering::Less
        } else if self == other {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}
