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
use super::ValueAggregate;
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
#[derive(Debug, Default, Clone)]
pub struct Stream(Vec<Vec<ValueAggregate>>);

impl Stream {
    pub(crate) fn from_generations_count(count: usize) -> Self {
        Self(vec![vec![]; count + 1])
    }

    pub(crate) fn from_value(value: ValueAggregate) -> Self {
        Self(vec![vec![value]])
    }

    // if generation is None, value would be added to the last generation, otherwise it would
    // be added to given generation
    pub(crate) fn add_value(&mut self, value: ValueAggregate, generation: Generation) -> ExecutionResult<u32> {
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

    pub(crate) fn elements_count(&self, generation: Generation) -> Option<usize> {
        match generation {
            Generation::Nth(generation) if generation as usize > self.generations_count() => None,
            Generation::Nth(generation) => Some(self.0.iter().take(generation as usize).map(|v| v.len()).sum()),
            Generation::Last => Some(self.0.iter().map(|v| v.len()).sum()),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        if self.0.is_empty() {
            return false;
        }

        self.0.iter().all(|v| v.is_empty())
    }

    pub(crate) fn as_jvalue(&self, generation: Generation) -> Option<JValue> {
        use std::ops::Deref;

        let iter = self.iter(generation)?;
        let jvalue_array = iter.map(|r| r.result.deref().clone()).collect::<Vec<_>>();

        Some(JValue::Array(jvalue_array))
    }

    pub(crate) fn iter(&self, generation: Generation) -> Option<StreamIter<'_>> {
        let iter: Box<dyn Iterator<Item = &ValueAggregate>> = match generation {
            Generation::Nth(generation) if generation as usize >= self.generations_count() => return None,
            Generation::Nth(generation) => Box::new(self.0.iter().take(generation as usize + 1).flat_map(|v| v.iter())),
            Generation::Last => Box::new(self.0.iter().flat_map(|v| v.iter())),
        };
        // unwrap is safe here, because generation's been already checked
        let len = self.elements_count(generation).unwrap();

        let iter = StreamIter { iter, len };

        Some(iter)
    }

    pub(crate) fn slice_iter(&self, generation: Generation) -> Option<StreamSliceIter<'_>> {
        let iter: Box<dyn Iterator<Item = &[ValueAggregate]>> = match generation {
            Generation::Nth(generation) if generation as usize >= self.generations_count() => return None,
            Generation::Nth(generation) => Box::new(self.0.iter().take(generation as usize + 1).map(|v| v.as_slice())),
            Generation::Last => Box::new(self.0.iter().map(|v| v.as_slice())),
        };

        let len = match generation {
            Generation::Nth(generation) => generation as usize,
            Generation::Last => self.0.len(),
        };

        let iter = StreamSliceIter { iter, len };

        Some(iter)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Generation {
    Last,
    Nth(u32),
}

impl Generation {
    pub(crate) fn from_option(raw_generation: Option<u32>) -> Self {
        match raw_generation {
            Some(generation) => Generation::Nth(generation),
            None => Generation::Last,
        }
    }
}

pub(crate) struct StreamIter<'result> {
    iter: Box<dyn Iterator<Item = &'result ValueAggregate> + 'result>,
    len: usize,
}

impl<'result> Iterator for StreamIter<'result> {
    type Item = &'result ValueAggregate;

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

impl<'result> ExactSizeIterator for StreamIter<'result> {}

pub(crate) struct StreamSliceIter<'slice> {
    iter: Box<dyn Iterator<Item = &'slice [ValueAggregate]> + 'slice>,
    len: usize,
}

impl<'slice> Iterator for StreamSliceIter<'slice> {
    type Item = &'slice [ValueAggregate];

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
