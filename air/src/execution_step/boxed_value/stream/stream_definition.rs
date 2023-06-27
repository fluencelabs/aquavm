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

/// Streams are CRDT-like append only data structures. They are guaranteed to have the same order
/// of values on each peer.
#[derive(Debug, Default, Clone)]
pub struct Stream<T> {
    /// Values from previous data.
    previous_values: ValuesMatrix<T>,

    /// Values from current data.
    current_values: ValuesMatrix<T>,

    /// Values from call results executed on a current peer.
    new_values: ValuesMatrix<T>,
}

impl<T> Stream<T> {
    pub(crate) fn new() -> Self {
        Self {
            previous_values: ValuesMatrix::new(),
            current_values: ValuesMatrix::new(),
            new_values: ValuesMatrix::new(),
        }
    }

    pub(crate) fn from_new_value(value: T) -> Self {
        const FIRST_GENERATION: GenerationIdx = 0.into();
        Self {
            previous_values: ValuesMatrix::new(),
            current_values: ValuesMatrix::new(),
            new_values: ValuesMatrix::from_value(value, FIRST_GENERATION),
        }
    }

    pub(crate) fn add_value(&mut self, value: ValueAggregate, generation: Generation) {
        match generation {
            Generation::Previous(previous_gen) => self.previous_values.add_value_at_generation(value, previous_gen),
            Generation::Current(current_gen) => self.current_values.add_value_at_generation(value, current_gen),
            Generation::New => self.new_values.add_value_at_generation(value),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        let is_prev_empty = self.previous_values.values.iter().all(|v| v.is_empty());
        let is_curr_empty = self.current_values.values.iter().all(|v| v.is_empty());
        let is_new_empty = self.new_values.values.iter().all(|v| v.is_empty());

        is_prev_empty && is_curr_empty && is_new_empty
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &T> {
        let previous_iter = self.previous_values.values.iter().flat_map(|values| values.iter());
        let current_iter = self.current_values.values.iter().flat_map(|values| values.iter());
        let new_iter = self.new_values.values.iter().flat_map(|values| values.iter());

        previous_iter.chain(current_iter).chain(new_iter)
    }

    pub(crate) fn slice_iter(&self) -> impl Iterator<Item = &[T]> {
        let previous_iter = self.previous_values.iter();
        let current_iter = self.current_values.iter();
        let new_iter = self.new_values.iter();

        previous_iter.chain(current_iter).chain(new_iter)
    }

    /// Removes empty generations updating data and returns final generation count.
    pub(crate) fn compactify(mut self, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        self.previous_values.remove_empty_generations();
        self.current_values.remove_empty_generations();
        self.new_values.remove_empty_generations();

        let start_idx = 0.into();
        Self::update_generations(&self.previous_values, start_idx, trace_ctx)?;

        let start_idx = self.previous_values.values.len().into();
        Self::update_generations(&self.current_values, start_idx, trace_ctx)?;

        let start_idx = start_idx.checked_add(self.current_values.values.len().into()).unwrap();
        Self::update_generations(&self.new_values, start_idx, trace_ctx)?;

        Ok(())
    }

    fn update_generations(
        value_matrix: &ValuesMatrix<T>,
        start_idx: GenerationIdx,
        trace_ctx: &mut TraceHandler,
    ) -> ExecutionResult<()> {
        for (position, values) in value_matrix.values.iter().enumerate() {
            let generation = start_idx.checked_add(position.into()).unwrap();
            for value in values.iter() {
                trace_ctx
                    .update_generation(value.get_trace_pos(), generation)
                    .map_err(|e| ExecutionError::Uncatchable(UncatchableError::GenerationCompatificationError(e)))?;
            }
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Generation {
    Previous(GenerationIdx),
    Current(GenerationIdx),
    New,
}

impl Generation {
    #[cfg(test)]
    pub fn previous(generation_id: u32) -> Self {
        use std::convert::TryFrom;

        let generation_id = usize::try_from(generation_id).unwrap();
        let generation_idx = GenerationIdx::from(generation_id);
        Self::Previous(generation_idx)
    }

    #[cfg(test)]
    pub fn current(generation_id: u32) -> Self {
        use std::convert::TryFrom;

        let generation_id = usize::try_from(generation_id).unwrap();
        let generation_idx = GenerationIdx::from(generation_id);
        Self::Current(generation_idx)
    }

    pub fn new() -> Self {
        Self::New
    }
}

use std::fmt;

impl<T: fmt::Display> fmt::Display for Stream<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "previous values:\n{}", self.previous_values)?;
        writeln!(f, "current values:\n{}", self.current_values)?;
        writeln!(f, "new values:\n{}", self.new_values)
    }
}

impl fmt::Display for Generation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Generation::Nth(generation) => write!(f, "{}", generation),
            Generation::Last => write!(f, "Last"),
        }
    }
}
