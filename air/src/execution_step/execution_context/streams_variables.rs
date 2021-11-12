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
    // TODO: get rid of RefCell in a separate PR
    streams: HashMap<String, Vec<RefCell<Stream>>>,

    /// Contains stream generation that private stream should have at the scope start.
    data_private_stream_generations: RestrictedStreamGens,

    /// Contains stream generations that each private stream had at the scope end.
    /// Then it's placed into data
    collected_restricted_stream_gens: RestrictedStreamGens,
}

impl Streams {
    pub(crate) fn get(&self, name: &str) -> Option<&RefCell<Stream>> {
        self.streams.get(name).map(|embodiments| embodiments.last()).flatten()
    }

    pub(crate) fn add_stream_value(
        &mut self,
        executed_result: ValueAggregate,
        generation: Generation,
        stream_name: String,
    ) -> ExecutionResult<u32> {
        let generation = match self.streams.entry(stream_name) {
            Occupied(mut stream) => {
                // unwrap is safe here because streams contains only non-empty vec
                stream
                    .get_mut()
                    .last()
                    .unwrap()
                    .borrow_mut()
                    .add_value(executed_result, generation)?
            }
            Vacant(entry) => {
                let stream = RefCell::new(Stream::from_value(executed_result));
                entry.insert(vec![stream]);
                0
            }
        };

        Ok(generation)
    }

    pub(crate) fn add_global_stream(&mut self, name: String, stream: Stream) {
        self.streams.insert(name, vec![RefCell::new(stream)]);
    }

    pub(crate) fn meet_scope_start(&mut self, name: impl Into<String>, position: u32, iteration: u32) {
        let name = name.into();
        println!("met_scope_start: {} {} {}", name, position, iteration);
        let generations_count = self
            .stream_generation_from_data(&name, position, iteration as usize)
            .unwrap_or_default();

        let new_stream = RefCell::new(Stream::from_generations_count(generations_count as usize));
        match self.streams.entry(name) {
            Occupied(mut entry) => {
                entry.get_mut().push(new_stream);
            }
            Vacant(entry) => {
                entry.insert(vec![new_stream]);
            }
        }
    }

    pub(crate) fn meet_scope_end(&mut self, name: String, position: u32) {
        println!("meet_scope_end: {} {}", name, position);
        // unwraps are safe here because met_scope_end must be called after met_scope_start
        let stream_embodiments = self.streams.get_mut(&name).unwrap();
        // delete a stream after exit from a scope
        let last_stream = stream_embodiments.pop().unwrap();
        if stream_embodiments.is_empty() {
            // streams should contain only non-empty stream embodiments
            self.streams.remove(&name);
        }

        self.collect_stream_generation(name, position, last_stream.borrow().generations_count() as u32);
    }

    /// This method must be called at the end of execution, because it contains logic to collect
    /// all global streams depending on their presence in a streams field.
    pub(crate) fn into_streams_data(self) -> (GlobalStreamGens, RestrictedStreamGens) {
        // since it's called at the end of execution, streams contains only global ones,
        // because all private's been deleted after exiting a scope
        let global_streams = self
            .streams
            .into_iter()
            .map(|(name, mut embodiments)| {
                // unwrap is safe here because of invariant that streams contains non-empty vectors,
                // moreover it must contain only one values, because this method is called at the end
                // of the execution
                let generation = embodiments.pop().unwrap().borrow().generations_count();
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

use std::fmt;

impl fmt::Display for Streams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (name, embodiments) in self.streams.iter() {
            let value = embodiments.last();
            if let Some(last_value) = value {
                writeln!(f, "{} => {}", name, last_value.borrow())?;
            }
        }
        Ok(())
    }
}
