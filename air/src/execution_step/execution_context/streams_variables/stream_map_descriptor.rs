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

use crate::execution_step::StreamMap;

use air_parser::ast::Span;
use air_parser::AirPos;

use std::fmt;

pub(super) struct StreamMapDescriptor {
    pub(super) span: Span,
    pub(super) stream_map: StreamMap,
}

impl StreamMapDescriptor {
    pub(super) fn global(stream_map: StreamMap) -> Self {
        Self {
            span: Span::new(0.into(), usize::MAX.into()),
            stream_map,
        }
    }

    pub(super) fn restricted(stream_map: StreamMap, span: Span) -> Self {
        Self { span, stream_map }
    }
}

impl fmt::Display for StreamMapDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " <{}> - <{}>: {}", self.span.left, self.span.right, self.stream_map)
    }
}

pub(super) fn find_closest<'d>(
    descriptors: impl DoubleEndedIterator<Item = &'d StreamMapDescriptor>,
    position: AirPos,
) -> Option<&'d StreamMap> {
    // descriptors are placed in a order of decreasing scopes, so it's enough to get the latest suitable
    for descriptor in descriptors.rev() {
        if descriptor.span.contains_position(position) {
            return Some(&descriptor.stream_map);
        }
    }

    None
}

pub(super) fn find_closest_mut<'d>(
    descriptors: impl DoubleEndedIterator<Item = &'d mut StreamMapDescriptor>,
    position: AirPos,
) -> Option<&'d mut StreamMap> {
    // descriptors are placed in a order of decreasing scopes, so it's enough to get the latest suitable
    for descriptor in descriptors.rev() {
        if descriptor.span.contains_position(position) {
            return Some(&mut descriptor.stream_map);
        }
    }

    None
}
