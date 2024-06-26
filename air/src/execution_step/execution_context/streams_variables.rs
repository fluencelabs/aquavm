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

mod stream_descriptor;
mod stream_value_descriptor;

use crate::execution_step::ExecutionResult;
use crate::execution_step::Stream;

use stream_descriptor::*;
pub(crate) use stream_value_descriptor::StreamValueDescriptor;

use air_parser::ast::Span;
use air_parser::AirPos;
use air_trace_handler::TraceHandler;

use std::collections::hash_map::Entry::Occupied;
use std::collections::hash_map::Entry::Vacant;
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

    pub(crate) fn add_stream_value(&mut self, value_descriptor: StreamValueDescriptor<'_>) -> ExecutionResult<()> {
        let StreamValueDescriptor {
            value,
            name,
            generation,
            position,
        } = value_descriptor;

        match self.get_mut(name, position) {
            Some(stream) => stream.add_value(value, generation)?,
            None => {
                // streams are created:
                //  - when meet_scope_start is called by `new` instruction to create a restricted stream
                //  - when this f() is called and there is no global stream with the same name in ExecCtx.
                let mut stream = Stream::new();
                stream.add_value(value, generation)?;
                let descriptor = StreamDescriptor::global(stream);
                self.streams.insert(name.to_string(), vec![descriptor]);
            }
        }
        Ok(())
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
