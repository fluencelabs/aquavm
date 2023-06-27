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

pub(crate) mod errors;
pub(crate) mod stream_map_key;

use self::stream_map_key::StreamMapKey;
use crate::execution_step::boxed_value::StreamMap;
use crate::execution_step::ExecutionResult;
use crate::execution_step::Generation;
use crate::execution_step::ValueAggregate;

use air_interpreter_data::GenerationIdx;
use air_parser::ast::Span;
use air_parser::AirPos;
use air_trace_handler::merger::ValueSource;
use air_trace_handler::TraceHandler;

use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::fmt;

// TODO This module should be unified with its Stream counterpart.
pub(crate) struct StreamMapValueDescriptor<'stream_name> {
    pub value: ValueAggregate,
    pub name: &'stream_name str,
    pub source: ValueSource,
    pub generation: Generation,
    pub position: AirPos,
}

impl<'stream_name> StreamMapValueDescriptor<'stream_name> {
    pub fn new(
        value: ValueAggregate,
        name: &'stream_name str,
        source: ValueSource,
        generation: Generation,
        position: AirPos,
    ) -> Self {
        Self {
            value,
            name,
            source,
            generation,
            position,
        }
    }
}

pub(crate) struct StreamMapDescriptor {
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
    descriptors
        .rev()
        .find(|d| d.span.contains_position(position))
        .map(|d| &d.stream_map)
}

pub(super) fn find_closest_mut<'d>(
    descriptors: impl DoubleEndedIterator<Item = &'d mut StreamMapDescriptor>,
    position: AirPos,
) -> Option<&'d mut StreamMap> {
    // descriptors are placed in a order of decreasing scopes, so it's enough to get the latest suitable
    descriptors
        .rev()
        .find(|d| d.span.contains_position(position))
        .map(|d| &mut d.stream_map)
}
#[derive(Default)]
pub(crate) struct StreamMaps {
    // this one is optimized for speed (not for memory), because it's unexpected
    // that a script could have a lot of new.
    // TODO: use shared string (Rc<String>) to avoid copying.
    stream_maps: HashMap<String, Vec<StreamMapDescriptor>>,

    /// Contains stream generations from previous data that a restricted stream
    /// should have at the scope start.
    previous_restricted_stream_maps_gens: RestrictedStreamMapGens,

    /// Contains stream generations from current data that a restricted stream
    /// should have at the scope start.
    current_restricted_stream_maps_gens: RestrictedStreamMapGens,

    /// Contains stream generations that each private stream had at the scope end.
    /// Then it's placed into data
    new_restricted_stream_maps_gens: RestrictedStreamMapGens,
}

impl StreamMaps {
    pub(crate) fn get(&self, name: &str, position: AirPos) -> Option<&StreamMap> {
        self.stream_maps
            .get(name)
            .and_then(|descriptors| find_closest(descriptors.iter(), position))
    }

    pub(crate) fn get_mut(&mut self, name: &str, position: AirPos) -> Option<&mut StreamMap> {
        self.stream_maps
            .get_mut(name)
            .and_then(|descriptors| find_closest_mut(descriptors.iter_mut(), position))
    }

    pub(crate) fn add_stream_map_value(
        &mut self,
        key: StreamMapKey<'_>,
        value_descriptor: StreamMapValueDescriptor<'_>,
    ) -> ExecutionResult<GenerationIdx> {
        let StreamMapValueDescriptor {
            value,
            name,
            source,
            generation,
            position,
        } = value_descriptor;

        match self.get_mut(name, position) {
            Some(stream_map) => stream_map.insert(key, &value, generation, source),
            None => {
                // streams could be created in three ways:
                //  - after met new instruction with stream name that isn't present in streams
                //    (it's the only way to create restricted streams)
                //  - by calling add_global_stream with generation that come from data
                //    for global streams
                //  - and by this function, and if there is no such a streams in streams,
                //    it means that a new global one should be created.
                let stream_map = StreamMap::from_value(key, &value);
                let descriptor = StreamMapDescriptor::global(stream_map);
                self.stream_maps.insert(name.to_string(), vec![descriptor]);
                let generation = 0;
                Ok(generation.into())
            }
        }
    }

    pub(crate) fn meet_scope_start(&mut self, name: impl Into<String>, span: Span, iteration: usize) {
        let name = name.into();
        let (prev_gens_count, current_gens_count) = self.stream_generation_from_data(&name, span.left, iteration);

        let new_stream_map = StreamMap::from_generations_count(prev_gens_count, current_gens_count);
        let new_descriptor = StreamMapDescriptor::restricted(new_stream_map, span);
        match self.stream_maps.entry(name) {
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
        let stream_map_descriptors = self.stream_maps.get_mut(&name).unwrap();
        // delete a stream after exit from a scope
        let last_descriptor = stream_map_descriptors.pop().unwrap();
        if stream_map_descriptors.is_empty() {
            // streams should contain only non-empty stream embodiments
            self.stream_maps.remove(&name);
        }
        let gens_count = last_descriptor.stream_map.compactify(trace_ctx)?;

        self.collect_stream_generation(name, position, gens_count);
        Ok(())
    }

    fn stream_generation_from_data(
        &self,
        name: &str,
        position: AirPos,
        iteration: usize,
    ) -> (GenerationIdx, GenerationIdx) {
        let previous_generation =
            Self::restricted_stream_generation(&self.previous_restricted_stream_maps_gens, name, position, iteration)
                .unwrap_or_default();
        let current_generation =
            Self::restricted_stream_generation(&self.current_restricted_stream_maps_gens, name, position, iteration)
                .unwrap_or_default();

        (previous_generation, current_generation)
    }

    fn restricted_stream_generation(
        restricted_stream_maps_gens: &RestrictedStreamGens,
        name: &str,
        position: AirPos,
        iteration: usize,
    ) -> Option<GenerationIdx> {
        restricted_stream_maps_gens
            .get(name)
            .and_then(|scopes| scopes.get(&position).and_then(|iterations| iterations.get(iteration)))
            .copied()
    }

    fn collect_stream_generation(&mut self, name: String, position: AirPos, generation: GenerationIdx) {
        match self.new_restricted_stream_maps_gens.entry(name) {
            Occupied(mut streams) => streams.get_mut().entry(position).or_default().push(generation),
            Vacant(entry) => {
                let iterations = maplit::hashmap! {
                    position => vec![generation],
                };
                entry.insert(iterations);
            }
        }
    }
}

impl fmt::Display for StreamMaps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (name, descriptors) in self.stream_maps.iter() {
            if let Some(last_descriptor) = descriptors.last() {
                writeln!(f, "{name} => {last_descriptor}")?;
            }
        }
        Ok(())
    }
}
