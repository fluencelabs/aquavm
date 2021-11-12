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

use crate::execution_step::ExecutionResult;
use crate::execution_step::Generation;
use crate::execution_step::Stream;
use crate::execution_step::ValueAggregate;

use air_interpreter_data::GlobalStreamGens;
use air_interpreter_data::RestrictedStreamGens;

use std::cell::RefCell;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

#[derive(Default)]
pub(crate) struct Streams {
    // this one is optimized for speed (not for memory), because it's unexpected
    // that a script could have a lot of new.
    // TODO: use shared string (Rc<String>) to avoid copying.
    streams: HashMap<String, Vec<StreamDescriptor>>,

    /// Contains stream generation that private stream should have at the scope start.
    data_private_stream_generations: RestrictedStreamGens,

    /// Contains stream generations that each private stream had at the scope end.
    /// Then it's placed into data
    collected_restricted_stream_gens: RestrictedStreamGens,
}

struct StreamDescriptor {
    pub(self) left_position: usize,
    pub(self) right_position: usize,
    // TODO: get rid of RefCell in a separate PR
    pub(self) stream: RefCell<Stream>,
}

impl Streams {
    pub(crate) fn get(&self, name: &str, position: usize) -> Option<&RefCell<Stream>> {
        let result = self
            .streams
            .get(name)
            .map(|descriptors| find_closest(descriptors.iter(), position))
            .flatten();
        result
    }

    pub(crate) fn add_stream_value(
        &mut self,
        value: ValueAggregate,
        generation: Generation,
        stream_name: &str,
        position: usize,
    ) -> ExecutionResult<u32> {
        match self.get(stream_name, position) {
            Some(stream) => stream.borrow_mut().add_value(value, generation),
            None => {
                // streams could be created in three ways:
                //  - after met new instruction with stream name that isn't present in streams
                //    (it's the only way to create restricted streams)
                //  - by calling add_global_stream with generation that come from data
                //    for global streams
                //  - and by this function, and if there is no such a streams in streams,
                //    it means that a new global one should be created.
                let stream = Stream::from_value(value);
                self.add_global_stream(stream_name.to_string(), stream);
                Ok(0)
            }
        }
    }

    pub(crate) fn add_global_stream(&mut self, name: String, stream: Stream) {
        let descriptor = StreamDescriptor::global(RefCell::new(stream));
        self.streams.insert(name, vec![descriptor]);
    }

    pub(crate) fn meet_scope_start(
        &mut self,
        name: impl Into<String>,
        left_position: usize,
        right_position: usize,
        iteration: u32,
    ) {
        let name = name.into();
        let generations_count = self
            .stream_generation_from_data(&name, left_position as u32, iteration as usize)
            .unwrap_or_default();

        let new_stream = RefCell::new(Stream::from_generations_count(generations_count as usize));
        let new_descriptor = StreamDescriptor::restricted(new_stream, left_position, right_position);
        match self.streams.entry(name) {
            Occupied(mut entry) => {
                entry.get_mut().push(new_descriptor);
            }
            Vacant(entry) => {
                entry.insert(vec![new_descriptor]);
            }
        }
    }

    pub(crate) fn meet_scope_end(&mut self, name: String, position: u32) {
        // unwraps are safe here because met_scope_end must be called after met_scope_start
        let stream_descriptors = self.streams.get_mut(&name).unwrap();
        // delete a stream after exit from a scope
        let last_descriptor = stream_descriptors.pop().unwrap();
        if stream_descriptors.is_empty() {
            // streams should contain only non-empty stream embodiments
            self.streams.remove(&name);
        }

        self.collect_stream_generation(
            name,
            position,
            last_descriptor.stream.borrow().generations_count() as u32,
        );
    }

    /// This method must be called at the end of execution, because it contains logic to collect
    /// all global streams depending on their presence in a streams field.
    pub(crate) fn into_streams_data(self) -> (GlobalStreamGens, RestrictedStreamGens) {
        // since it's called at the end of execution, streams contains only global ones,
        // because all private's been deleted after exiting a scope
        let global_streams = self
            .streams
            .into_iter()
            .map(|(name, mut descriptors)| {
                // unwrap is safe here because of invariant that streams contains non-empty vectors,
                // moreover it must contain only one values, because this method is called at the end
                // of the execution
                let generation = descriptors.pop().unwrap().stream.borrow().generations_count();
                (name, generation as u32)
            })
            .collect::<GlobalStreamGens>();

        (global_streams, self.collected_restricted_stream_gens)
    }

    fn stream_generation_from_data(&self, name: &str, position: u32, iteration: usize) -> Option<u32> {
        self.data_private_stream_generations
            .get(name)
            .and_then(|scopes| {
                scopes
                    .get(&position)
                    .map(|iterations| iterations.get(iteration))
                    .flatten()
            })
            .copied()
    }

    fn collect_stream_generation(&mut self, name: String, position: u32, generation: u32) {
        match self.collected_restricted_stream_gens.entry(name) {
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

impl StreamDescriptor {
    pub(self) fn global(stream: RefCell<Stream>) -> Self {
        Self {
            left_position: 0,
            right_position: usize::MAX,
            stream,
        }
    }

    pub(self) fn restricted(stream: RefCell<Stream>, left_position: usize, right_position: usize) -> Self {
        Self {
            left_position,
            right_position,
            stream,
        }
    }
}

fn find_closest<'d>(
    descriptors: impl DoubleEndedIterator<Item = &'d StreamDescriptor>,
    position: usize,
) -> Option<&'d RefCell<Stream>> {
    // descriptors are placed in a order of decreasing scopes, so it's enough to get the latest suitable
    for descriptor in descriptors.rev() {
        if descriptor.left_position < position && position < descriptor.right_position {
            return Some(&descriptor.stream);
        }
    }

    None
}

use std::fmt;

impl fmt::Display for Streams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (name, descriptors) in self.streams.iter() {
            let descriptor = descriptors.last();
            if let Some(last_descriptor) = descriptor {
                writeln!(f, "{} => {}", name, last_descriptor)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for StreamDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            " <{}> - <{}>: {}",
            self.left_position,
            self.right_position,
            self.stream.borrow()
        )
    }
}
