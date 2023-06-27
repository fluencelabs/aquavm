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

pub struct RecursiveStream<'stream, T> {
    stream: &'stream Stream<T>,
    last_new_generation: GenerationIdx,
}

pub(crate) enum IterationResult {
    Stop,
    Continue,
}

impl<'stream, T> RecursiveStream<'stream, T> {
    pub fn new(stream: &'stream Stream<T>) -> Self {
        let last_new_generation = stream.new_values.values.len();
        let last_new_generation = last_new_generation.into();

        Self {
            stream,
            last_new_generation,
        }
    }

    pub fn iteration_start_met(&mut self) {
        self.stream.new_values.values.push(vec![]);

        let last_new_generation = self.stream.new_values.values.len();
        self.last_new_generation = last_new_generation.into();
    }

    pub fn iteration_end_met(&mut self) -> IterationResult {
        if self.stream.new_values.values[self.last_new_generation].is_empty() {
            self.stream.new_values.values.pop();
            return IterationResult::Stop;
        }

        IterationResult::Continue
    }
}
