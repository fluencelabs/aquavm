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
mod utils;

use crate::execution_step::ExecutionResult;
use crate::execution_step::Stream;
use crate::ExecutionError;
use air_interpreter_data::GenerationIdx;
use stream_descriptor::*;
pub(crate) use stream_value_descriptor::StreamValueDescriptor;

use air_interpreter_data::GlobalStreamGens;
use air_interpreter_data::RestrictedStreamGens;
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

    /// Contains stream generations from previous data that a restricted stream
    /// should have at the scope start.
    previous_restricted_stream_gens: RestrictedStreamGens,

    /// Contains stream generations from current data that a restricted stream
    /// should have at the scope start.
    current_restricted_stream_gens: RestrictedStreamGens,

    /// Contains stream generations that each private stream had at the scope end.
    /// Then it's placed into data
    new_restricted_stream_gens: RestrictedStreamGens,
}

impl Streams {
    pub(crate) fn from_data(
        previous_global_streams: GlobalStreamGens,
        current_global_streams: GlobalStreamGens,
        previous_restricted_stream_gens: RestrictedStreamGens,
        current_restricted_stream_gens: RestrictedStreamGens,
    ) -> Self {
        let streams = utils::merge_global_streams(previous_global_streams, current_global_streams);

        Self {
            streams,
            previous_restricted_stream_gens,
            current_restricted_stream_gens,
            new_restricted_stream_gens: <_>::default(),
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

    pub(crate) fn add_stream_value(&mut self, value_descriptor: StreamValueDescriptor<'_>) -> ExecutionResult<u32> {
        let StreamValueDescriptor {
            value,
            name,
            source,
            generation,
            position,
        } = value_descriptor;

        match self.get_mut(name, position) {
            Some(stream) => stream.add_value(value, generation, source),
            None => {
                // streams could be created in three ways:
                //  - after met new instruction with stream name that isn't present in streams
                //    (it's the only way to create restricted streams)
                //  - by calling add_global_stream with generation that come from data
                //    for global streams
                //  - and by this function, and if there is no such a streams in streams,
                //    it means that a new global one should be created.
                let stream = Stream::from_value(value);
                let descriptor = StreamDescriptor::global(stream);
                self.streams.insert(name.to_string(), vec![descriptor]);
                let generation = 0;
                Ok(generation)
            }
        }
    }

    pub(crate) fn meet_scope_start(&mut self, name: impl Into<String>, span: Span, iteration: u32) {
        let name = name.into();
        let (prev_gens_count, current_gens_count) =
            self.stream_generation_from_data(&name, span.left, iteration as usize);

        let new_stream = Stream::from_generations_count(prev_gens_count as usize, current_gens_count as usize);
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

    pub(crate) fn meet_scope_end(
        &mut self,
        name: String,
        position: AirPos,
        trace_ctx: &mut TraceHandler,
    ) -> ExecutionResult<()> {
        // unwraps are safe here because met_scope_end must be called after met_scope_start
        let stream_descriptors = self.streams.get_mut(&name).unwrap();
        // delete a stream after exit from a scope
        let last_descriptor = stream_descriptors.pop().unwrap();
        if stream_descriptors.is_empty() {
            // streams should contain only non-empty stream embodiments
            self.streams.remove(&name);
        }
        let gens_count = last_descriptor.stream.compactify(trace_ctx)?;

        self.collect_stream_generation(name, position, gens_count as u32);
        Ok(())
    }

    /// This method must be called at the end of execution, because it contains logic to collect
    /// all global streams depending on their presence in a streams field.
    pub(crate) fn into_streams_data(
        self,
        trace_ctx: &mut TraceHandler,
    ) -> ExecutionResult<(GlobalStreamGens, RestrictedStreamGens)> {
        // since it's called at the end of execution, streams contains only global ones,
        // because all private's been deleted after exiting a scope
        let global_streams = self
            .streams
            .into_iter()
            .map(|(name, mut descriptors)| -> Result<_, ExecutionError> {
                // unwrap is safe here because of invariant that streams contains non-empty vectors,
                // moreover it must contain only one value, because this method is called at the end
                // of the execution
                let stream = descriptors.pop().unwrap().stream;
                let gens_count = stream.compactify(trace_ctx)?;
                Ok((name, gens_count as u32))
            })
            .collect::<Result<GlobalStreamGens, _>>()?;

        Ok((global_streams, self.new_restricted_stream_gens))
    }

    fn stream_generation_from_data(&self, name: &str, position: AirPos, iteration: usize) -> (u32, u32) {
        let previous_generation =
            Self::restricted_stream_generation(&self.previous_restricted_stream_gens, name, position, iteration)
                .unwrap_or_default();
        let current_generation =
            Self::restricted_stream_generation(&self.current_restricted_stream_gens, name, position, iteration)
                .unwrap_or_default();

        (previous_generation, current_generation)
    }

    fn restricted_stream_generation(
        restricted_stream_gens: &RestrictedStreamGens,
        name: &str,
        position: AirPos,
        iteration: usize,
    ) -> Option<u32> {
        restricted_stream_gens
            .get(name)
            .and_then(|scopes| scopes.get(&position).and_then(|iterations| iterations.get(iteration)))
            .copied()
    }

    fn collect_stream_generation(&mut self, name: String, position: AirPos, generation: GenerationIdx) {
        match self.new_restricted_stream_gens.entry(name) {
            Occupied(mut streams) => match streams.get_mut().entry(position) {
                Occupied(mut iterations) => iterations.get_mut().push(generation),
                Vacant(entry) => {
                    entry.insert(vec![generation]);
                }
            },
            Vacant(entry) => {
                let iterations = maplit::hashmap! {
                    position => vec![generation],
                };
                entry.insert(iterations);
            }
        }
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
