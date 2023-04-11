/*
 * Copyright 2023 Fluence Labs Limited
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
