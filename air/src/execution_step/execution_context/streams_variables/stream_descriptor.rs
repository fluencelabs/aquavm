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
