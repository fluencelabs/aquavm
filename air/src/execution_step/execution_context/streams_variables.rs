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

mod stream_descriptor;
mod stream_value_descriptor;

use crate::execution_step::Stream;
use crate::execution_step::ExecutionResult;

use stream_descriptor::*;
pub(crate) use stream_value_descriptor::StreamValueDescriptor;

use air_parser::ast::Span;
use air_parser::AirPos;
use air_trace_handler::TraceHandler;

use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

#[derive(Default)]
pub(crate) struct Streams {
    // this one is optimized for speed (not for memory), because it's unexpected
    // that a script could have a lot of new.
    // TODO: use shared string (Rc<String>) to avoid copying.
    streams: HashMap<String, Vec<StreamDescriptor>>,
}

impl Streams {
    pub(crate) fn new() -> Self {
        Self {
            streams: <_>::default(),
        }
    }

    pub(crate) fn get(&self, name: &str, position: AirPos) -> Option<&Stream> {
        self.streams
            .get(name)
            .and_then(|descriptors| find_closest(descriptors.iter(), position))
    }

    pub(crate) fn get_mut(&mut self, name: &str, position: AirPos) -> Option<&mut Stream> {
        self.streams
            .get_mut(name)
            .and_then(|descriptors| find_closest_mut(descriptors.iter_mut(), position))
    }

    pub(crate) fn add_stream_value(&mut self, value_descriptor: StreamValueDescriptor<'_>) {
        let StreamValueDescriptor {
            value,
            name,
            generation,
            position,
        } = value_descriptor;

        match self.get_mut(name, position) {
            Some(stream) => stream.add_value(value, generation),
            None => {
                // streams could be created in three ways:
                //  - after met new instruction with stream name that isn't present in streams
                //    (it's the only way to create restricted streams)
                //  - by calling add_global_stream with generation that come from data
                //    for global streams
                //  - and by this function, and if there is no such a streams in streams,
                //    it means that a new global one should be created.
                let stream = Stream::from_new_value(value);
                let descriptor = StreamDescriptor::global(stream);
                self.streams.insert(name.to_string(), vec![descriptor]);
            }
        }
    }

    pub(crate) fn meet_scope_start(&mut self, name: impl Into<String>, span: Span) {
        let name = name.into();

        let new_stream = Stream::new();
        let new_descriptor = StreamDescriptor::restricted(new_stream, span);
        match self.streams.entry(name) {
            Occupied(mut entry) => {
                entry.get_mut().push(new_descriptor);
            }
            Vacant(entry) => {
                entry.insert(vec![new_descriptor]);
            }
        }
    }

    pub(crate) fn meet_scope_end(&mut self, name: String, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        // unwraps are safe here because met_scope_end must be called after met_scope_start
        let stream_descriptors = self.streams.get_mut(&name).unwrap();
        // delete a stream after exit from a scope
        let mut last_descriptor = stream_descriptors.pop().unwrap();
        if stream_descriptors.is_empty() {
            // streams should contain only non-empty stream embodiments
            self.streams.remove(&name);
        }

        last_descriptor.stream.compactify(trace_ctx)
    }

    pub(crate) fn compactify(&mut self, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        for (_, descriptors) in self.streams.iter_mut() {
            for descriptor in descriptors {
                descriptor.stream.compactify(trace_ctx)?;
            }
        }

        Ok(())
    }
}

use std::fmt;

impl fmt::Display for Streams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (name, descriptors) in self.streams.iter() {
            if let Some(last_descriptor) = descriptors.last() {
                writeln!(f, "{name} => {last_descriptor}")?;
            }
        }
        Ok(())
    }
}
