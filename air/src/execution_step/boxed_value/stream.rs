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

use super::ExecutionError;
use super::ExecutionResult;
use super::ResolvedCallResult;
use crate::exec_err;
use crate::JValue;
use std::fmt::Formatter;

/// Streams are CRDT-like append only data structures. They are guaranteed to have the same order
/// of values on each peer.
///
/// The first Vec represents generations, the second values in a generation. Generation is a set
/// of values that interpreter obtained from one particle. It means that number of generation on
/// a peer is equal to number of the interpreter runs in context of one particle. And each set of
/// obtained values from a current_data that were not present in prev_data becomes a new generation.
// TODO: make it non-pub after boxed value refactoring.
#[derive(Debug, Default, Clone)]
pub(crate) struct Stream(pub(crate) Vec<Vec<ResolvedCallResult>>);

impl Stream {
    pub(crate) fn from_generations_count(count: usize) -> Self {
        Self(vec![vec![]; count + 1])
    }

    pub(crate) fn from_value(value: ResolvedCallResult) -> Self {
        Self(vec![vec![value]])
    }

    // if generation is None, value would be added to the last generation, otherwise it would
    // be added to given generation
    pub(crate) fn add_value(&mut self, value: ResolvedCallResult, generation: Generation) -> ExecutionResult<u32> {
        let generation = match generation {
            Generation::Last => self.0.len() - 1,
            Generation::Nth(id) => id as usize,
        };

        if generation >= self.0.len() {
            return exec_err!(ExecutionError::StreamDontHaveSuchGeneration(self.clone(), generation));
        }

        self.0[generation].push(value);
        Ok(generation as u32)
    }

    pub(crate) fn generations_count(&self) -> usize {
        let generations_count = self.0.len();

        // the last generation could be empty due to the logic of from_generations_count ctor
        if generations_count > 0 && self.0[generations_count - 1].is_empty() {
            generations_count - 1
        } else {
            generations_count
        }
    }

    pub(crate) fn elements_count(&self) -> usize {
        self.0.iter().map(|v| v.len()).sum()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn into_jvalue(self) -> JValue {
        use std::ops::Deref;

        let jvalue_array = self
            .0
            .iter()
            .flat_map(|g| g.iter().map(|v| v.result.deref().clone()))
            .collect::<Vec<_>>();
        JValue::Array(jvalue_array)
    }

    pub(crate) fn iter(&self) -> StreamIter<'_> {
        let iter = self.0.iter().flat_map(|v| v.iter());
        let len = self.elements_count();

        StreamIter {
            iter: Box::new(iter),
            len,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum Generation {
    Last,
    Nth(u32),
}

pub(crate) struct StreamIter<'a> {
    iter: Box<dyn Iterator<Item = &'a ResolvedCallResult> + 'a>,
    len: usize,
}

impl<'a> Iterator for StreamIter<'a> {
    type Item = &'a ResolvedCallResult;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.len -= 1;
        }
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a> ExactSizeIterator for StreamIter<'a> {}

use std::fmt;
impl fmt::Display for Stream {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            return write!(f, "[]");
        }

        write!(f, "[ ")?;
        for (id, generation) in self.0.iter().enumerate() {
            write!(f, " -- {}: ", id)?;
            for value in generation.iter() {
                write!(f, "{:?}, ", value)?;
            }
            writeln!(f)?;
        }

        write!(f, "]")
    }
}
