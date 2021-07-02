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

use super::ExecutionTrace;
use super::KeeperError;
use super::KeeperResult;
use super::TraceSlider;

use air_interpreter_data::InterpreterData;
use air_interpreter_data::StreamGenerations;

use std::collections::HashMap;

/// Contains all necessary information about data.
#[derive(Debug, Default, PartialEq)]
pub(crate) struct MergeCtx {
    pub(crate) slider: TraceSlider,
    pub(crate) streams: StreamGenerations,
}

impl MergeCtx {
    #[allow(dead_code)]
    pub(crate) fn from_trace(trace: ExecutionTrace) -> Self {
        let slider = TraceSlider::new(trace);

        Self {
            slider,
            streams: HashMap::new(),
        }
    }

    pub(crate) fn from_data(data: InterpreterData) -> Self {
        let slider = TraceSlider::new(data.trace);

        Self {
            slider,
            streams: data.streams,
        }
    }

    pub(crate) fn stream_generation(&self, stream_name: &str) -> KeeperResult<u32> {
        self.streams
            .get(stream_name)
            .copied()
            .ok_or_else(|| KeeperError::NoSuchStream(stream_name.to_string()))
    }
}
