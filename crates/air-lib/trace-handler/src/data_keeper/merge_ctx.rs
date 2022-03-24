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

use air_interpreter_data::GlobalStreamGens;
use air_interpreter_data::InterpreterData;

use std::collections::HashMap;

/// Contains all necessary information about data.
#[derive(Debug, Default, PartialEq)]
pub struct MergeCtx<VT> {
    pub slider: TraceSlider<VT>,
    pub streams: GlobalStreamGens,
}

impl<VT> MergeCtx<VT> {
    #[allow(dead_code)]
    pub(crate) fn from_trace(trace: ExecutionTrace<VT>) -> Self {
        let slider = TraceSlider::new(trace);

        Self {
            slider,
            streams: HashMap::new(),
        }
    }

    pub(crate) fn from_data(data: InterpreterData<VT>) -> Self {
        let slider = TraceSlider::new(data.trace);

        Self {
            slider,
            streams: data.global_streams,
        }
    }

    pub(crate) fn try_get_generation(&self, position: u32) -> KeeperResult<u32, VT> {
        use air_interpreter_data::*;

        let position = position as usize;
        let state = self
            .slider
            .state_at_position(position)
            .ok_or_else(|| KeeperError::NoElementAtPosition {
                position,
                trace_len: self.slider.trace_len(),
            })?;

        match state {
            ExecutedState::Call(CallResult::Executed(Value::Stream { generation, .. })) => Ok(*generation),
            // such Aps are always preceded by Fold where corresponding stream could be used,
            // so it's been already checked that res_generation is well-formed
            // and accessing 0th element is safe here
            ExecutedState::Ap(ap_result) => Ok(ap_result.res_generations[0]),
            state => Err(KeeperError::NoStreamState { state: state.clone() }),
        }
    }

    pub(crate) fn stream_generation(&self, stream_name: &str) -> Option<u32> {
        self.streams.get(stream_name).copied()
    }
}
