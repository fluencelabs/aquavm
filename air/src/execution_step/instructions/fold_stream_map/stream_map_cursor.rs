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

use crate::execution_step::boxed_value::Generation;
use crate::execution_step::boxed_value::StreamMap;
use crate::execution_step::instructions::fold::construct_stream_map_iterable_values;
use crate::execution_step::instructions::fold::IterableValue;

pub(super) struct StreamMapCursor {
    last_seen_generation: GenerationIdx,
}

impl StreamMapCursor {
    pub(super) fn new() -> Self {
        Self {
            last_seen_generation: GenerationIdx::from(0),
        }
    }

    pub(super) fn construct_iterables(&mut self, stream_map: &StreamMap) -> Vec<IterableValue> {
        let iterables = construct_stream_map_iterable_values(
            stream_map,
            Generation::Nth(self.last_seen_generation),
            Generation::Last,
        );
        self.last_seen_generation = stream_map.last_non_empty_generation();

        iterables
    }
}
