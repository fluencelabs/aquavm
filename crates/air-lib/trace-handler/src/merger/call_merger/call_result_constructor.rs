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

use super::*;

pub(super) enum PrepareScheme {
    Previous,
    Current,
    Both,
}

pub(super) fn prepare_call_result(
    value: CallResult,
    scheme: PrepareScheme,
    data_keeper: &mut DataKeeper,
) -> MergerCallResult {
    let prev_pos = match scheme {
        PrepareScheme::Previous | PrepareScheme::Both => Some(data_keeper.prev_slider().position() - 1),
        PrepareScheme::Current => None,
    };

    let current_pos = match scheme {
        PrepareScheme::Current | PrepareScheme::Both => Some(data_keeper.current_slider().position() - 1),
        PrepareScheme::Previous => None,
    };

    let data_positions = DataPositions { prev_pos, current_pos };

    let trace_pos = data_keeper.result_states_count();
    data_keeper.new_to_old_pos.insert(trace_pos, data_positions);

    MergerCallResult::CallResult { value, trace_pos }
}
