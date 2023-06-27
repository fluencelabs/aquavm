/*
 * Copyright 2022 Fluence Labs Limited
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

use crate::execution_step::Generation;
use crate::execution_step::ValueAggregate;

use air_parser::AirPos;

pub(crate) struct StreamValueDescriptor<'stream_name> {
    pub value: ValueAggregate,
    pub name: &'stream_name str,
    pub generation: Generation,
    pub position: AirPos,
}

impl<'stream_name> StreamValueDescriptor<'stream_name> {
    pub fn new(value: ValueAggregate, name: &'stream_name str, generation: Generation, position: AirPos) -> Self {
        Self {
            value,
            name,
            generation,
            position,
        }
    }
}
