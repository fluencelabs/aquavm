/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
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
