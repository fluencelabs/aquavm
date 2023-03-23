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

pub(super) fn merge_global_streams(
    previous_global_streams: GlobalStreamGens,
    current_global_streams: GlobalStreamGens,
) -> HashMap<String, Vec<StreamDescriptor>> {
    let mut global_streams = previous_global_streams
        .iter()
        .map(|(stream_name, &prev_gens_count)| {
            let current_gens_count = current_global_streams.get(stream_name).cloned().unwrap_or_default();
            let global_stream = Stream::from_generations_count(prev_gens_count, current_gens_count);
            let descriptor = StreamDescriptor::global(global_stream);
            (stream_name.to_string(), vec![descriptor])
        })
        .collect::<HashMap<_, _>>();

    for (stream_name, current_gens_count) in current_global_streams {
        if previous_global_streams.contains_key(&stream_name) {
            continue;
        }

        let global_stream = Stream::from_generations_count(0, current_gens_count);
        let descriptor = StreamDescriptor::global(global_stream);
        global_streams.insert(stream_name, vec![descriptor]);
    }

    global_streams
}
