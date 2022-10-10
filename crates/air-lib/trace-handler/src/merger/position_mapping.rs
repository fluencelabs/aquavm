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

#[derive(Debug, Copy, Clone)]
pub enum PreparationScheme {
    Previous,
    Current,
    Both,
}

/// Prepares new_to_old_pos mapping in data keeper to keep track of value sources.
pub(super) fn prepare_positions_mapping(scheme: PreparationScheme, data_keeper: &mut DataKeeper) {
    use PreparationScheme::*;

    let new_pos = data_keeper.result_trace_next_pos();

    // it's safe to sub 1 from positions here iff scheme was set correctly
    match scheme {
        Previous => {
            let prev_pos = data_keeper.prev_slider().position() - 1;
            data_keeper.new_to_prev_pos.insert(new_pos, prev_pos);
        }
        Current => {
            let current_pos = data_keeper.current_slider().position() - 1;
            data_keeper.new_to_current_pos.insert(new_pos, current_pos);
        }
        Both => {
            let prev_pos = data_keeper.prev_slider().position() - 1;
            let current_pos = data_keeper.current_slider().position() - 1;
            data_keeper.new_to_prev_pos.insert(new_pos, prev_pos);
            data_keeper.new_to_current_pos.insert(new_pos, current_pos);
        }
    };
}
