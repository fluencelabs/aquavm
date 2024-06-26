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
