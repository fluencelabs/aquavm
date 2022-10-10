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

use super::StreamDescriptor;
use crate::execution_step::Stream;

use air_interpreter_data::GlobalStreamGens;

use std::collections::HashMap;

pub(super) fn prepare_global_streams(
    previous_global_streams: &GlobalStreamGens,
) -> HashMap<String, Vec<StreamDescriptor>> {
    previous_global_streams
        .iter()
        .map(|(stream_name, &prev_gens_count)| {
            let global_stream = Stream::from_generations_count(prev_gens_count as usize);
            let descriptor = StreamDescriptor::global(global_stream);
            (stream_name.to_string(), vec![descriptor])
        })
        .collect::<HashMap<_, _>>()
}
