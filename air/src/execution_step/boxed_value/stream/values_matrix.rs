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

struct ValuesMatrix<T> {
    /// The first Vec represents generations, the second values in a generation. Generation is a set
    /// of values that interpreter obtained from one particle. It means that number of generation on
    /// a peer is equal to number of the interpreter runs in context of one particle. And each set of
    /// obtained values from a current_data that were not present in prev_data becomes a new generation.
    values: Vec<Vec<T>>,
}

impl<T> ValuesMatrix<T> {
    pub fn new() -> Self {
        Self { values: vec![] }
    }

    pub fn from_value(value: T, generation_idx: GenerationIdx) -> Self {
        let mut values = Self::new();
        values.add_value_at_generation(value, generation_idx);

        values
    }

    pub fn add_value_at_position(&mut self, value: T, generation_idx: GenerationIdx) {
        let generation_idx: usize = generation_idx.into();
        if self.values.len() < generation_idx {
            self.values.resize(generation_idx, Vec::new());
        }

        self.values[generation_idx].push(value);
    }

    /// Removes empty generations from current values.
    pub fn remove_empty_generations(&mut self) {
        self.values.retain(|generation| generation.is_empty())
    }
}

struct NewValuesMatrix<T>(ValuesMatrix<T>);

impl<T> NewValuesMatrix<T> {
    pub fn add_to_last_position(&mut self, value: T) {
        if self.values.is_empty() {
            self.values.push(vec![]);
        }

        let last_generation_idx = self.values.len() - 1;
        self.values[last_generation_idx].push(value);
    }

}

use std::fmt;

impl<T: fmt::Display> fmt::Display for ValuesMatrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.values.is_empty() {
            return write!(f, "[]");
        }

        writeln!(f, "[")?;
        for (id, generation_values) in self.values.iter().enumerate() {
            write!(f, " -- {id}: ")?;
            for value in generation_values.iter() {
                write!(f, "{value}, ")?;
            }
            writeln!(f)?;
        }

        write!(f, "]")
    }
}
