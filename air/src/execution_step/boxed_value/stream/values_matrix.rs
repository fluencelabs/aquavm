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

use air_interpreter_data::GenerationIdx;

use typed_index_collections::TiVec;

/// It is intended to store values according to generations which stream has. Each generation could
/// contain several values, generation are ordered, meaning that values in elder generations are
/// handled by AIR instructions (such as fold) later. Also, there is an order between values in one
/// generation. And being placed by generations values could be considered as a matrix.
///
/// This matrix is used for values in previous and current data.
pub(crate) struct ValuesMatrix<T> {
    /// The first Vec represents generations, the second values in a generation. Generation is a set
    /// of values that interpreter obtained from one particle. It means that number of generation on
    /// a peer is equal to number of the interpreter runs in context of one particle.
    values: TiVec<GenerationIdx, Vec<T>>,
}

impl<T> ValuesMatrix<T> {
    pub fn new() -> Self {
        Self { values: TiVec::new() }
    }

    pub fn from_value(value: T, generation_idx: GenerationIdx) -> Self {
        let mut values = Self::new();
        values.add_value_to_generation(value, generation_idx);

        values
    }

    pub fn add_value_to_generation(&mut self, value: T, generation_idx: GenerationIdx) {
        if generation_idx >= self.values.len() {
            self.values.resize(generation_idx.into(), Vec::new());
        }

        self.values[generation_idx].push(value);
    }

    pub fn remove_empty_generations(&mut self) {
        self.values.retain(|generation| generation.is_empty())
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn len(&self) -> GenerationIdx {
        self.values.len().into()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.values.iter().flat_map(|generation| generation.iter())
    }

    pub fn slice_iter(&self) -> impl Iterator<Item = &[T]> {
        self.values.iter().filter(|generation| !generation.is_empty())
    }
}

/// It's intended to handle new values from call results.
pub(crate) struct NewValuesMatrix<T>(ValuesMatrix<T>);

impl<T> NewValuesMatrix<T> {
    pub fn from_value(value: T) -> Self {
        let values = TiVec::from(vec![value]);
        let values_matrix = ValuesMatrix { values };

        Self(values_matrix)
    }

    pub fn add_to_last_generation(&mut self, value: T) {
        let last_generation_idx = self.last_generation_idx();
        self.0.add_value_to_generation(value, last_generation_idx);
    }

    pub fn add_new_generation(&mut self) {
        self.0.values.push(vec![]);
    }

    pub fn last_generation(&self) -> &[T] {
        let last_generation_idx = self.last_generation_idx();
        &self.0.values[last_generation_idx]
    }

    pub fn remove_last_generation(&mut self) {
        self.0.values.pop()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }

    pub fn slice_iter(&self) -> impl Iterator<Item = &[T]> {
        self.0.slice_iter()
    }

    pub fn last_generation_idx(&self) -> GenerationIdx {
        if self.0.values.is_empty() {
            return 0.into();
        }

        self.0.values.len().into()
    }
}

use std::fmt;

impl<T: fmt::Display> fmt::Display for ValuesMatrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.values.is_empty() {
            return write!(f, "[]");
        }

        writeln!(f, "[")?;
        for (idx, generation_values) in self.values.iter_enumerated() {
            write!(f, " -- {idx}: ")?;
            for value in generation_values.iter() {
                write!(f, "{value}, ")?;
            }
            writeln!(f)?;
        }

        write!(f, "]")
    }
}

impl<T: fmt::Display> fmt::Display for NewValuesMatrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
