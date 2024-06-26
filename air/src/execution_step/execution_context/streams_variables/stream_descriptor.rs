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

use crate::execution_step::Stream;

use air_parser::ast::Span;
use air_parser::AirPos;

use std::fmt;

pub(super) struct StreamDescriptor {
    pub(super) span: Span,
    pub(super) stream: Stream,
}

impl StreamDescriptor {
    pub(super) fn global(stream: Stream) -> Self {
        Self {
            span: Span::new(0.into(), usize::MAX.into()),
            stream,
        }
    }

    pub(super) fn restricted(stream: Stream, span: Span) -> Self {
        Self { span, stream }
    }
}

impl fmt::Display for StreamDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " <{}> - <{}>: {}", self.span.left, self.span.right, self.stream)
    }
}

pub(super) fn find_closest<'d>(
    descriptors: impl DoubleEndedIterator<Item = &'d StreamDescriptor>,
    position: AirPos,
) -> Option<&'d Stream> {
    // descriptors are placed in a order of decreasing scopes, so it's enough to get the latest suitable
    for descriptor in descriptors.rev() {
        if descriptor.span.contains_position(position) {
            return Some(&descriptor.stream);
        }
    }

    None
}

pub(super) fn find_closest_mut<'d>(
    descriptors: impl DoubleEndedIterator<Item = &'d mut StreamDescriptor>,
    position: AirPos,
) -> Option<&'d mut Stream> {
    // descriptors are placed in a order of decreasing scopes, so it's enough to get the latest suitable
    for descriptor in descriptors.rev() {
        if descriptor.span.contains_position(position) {
            return Some(&mut descriptor.stream);
        }
    }

    None
}
