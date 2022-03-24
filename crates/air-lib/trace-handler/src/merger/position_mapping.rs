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

use super::DataKeeper;
use super::DataPositions;

pub(super) enum PreparationScheme {
    Previous,
    Current,
    Both,
}

/// Prepares new_to_old_pos mapping in data keeper to keep track of value sources.
pub(super) fn prepare_positions_mapping<VT>(scheme: PreparationScheme, data_keeper: &mut DataKeeper<VT>) {
    use PreparationScheme::*;

    // it's safe to sub 1 from positions iff scheme was set correctly
    let prev_pos = match scheme {
        Previous | Both => Some(data_keeper.prev_slider().position() - 1),
        Current => None,
    };

    let current_pos = match scheme {
        Current | Both => Some(data_keeper.current_slider().position() - 1),
        Previous => None,
    };

    let data_positions = DataPositions { prev_pos, current_pos };

    let trace_pos = data_keeper.result_states_count();
    data_keeper.new_to_old_pos.insert(trace_pos, data_positions);
}
