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

use super::*;

const EXPECTED_STATE_NAME: &str = "ap";

#[derive(Debug, Clone)]
pub enum MergerApResult {
    /// There is no corresponding state in a trace for this call.
    NotMet,

    /// There was a state in at least one of the contexts. If there were two states in
    /// both contexts, they were successfully merged.
    Met(MetApResult),
}

#[derive(Debug, Clone)]
pub struct MetApResult {
    pub generation: GenerationIdx,
    pub value_source: ValueSource,
}

pub(crate) fn try_merge_next_state_as_ap(data_keeper: &mut DataKeeper) -> MergeResult<MergerApResult> {
    use ExecutedState::Ap;
    use PreparationScheme::*;

    let prev_state = data_keeper.prev_slider_mut().next_state();
    let current_state = data_keeper.current_slider_mut().next_state();

    match (prev_state, current_state) {
        (Some(Ap(prev_ap)), Some(Ap(_))) => prepare_merge_result(prev_ap, Both, data_keeper),
        (Some(Ap(prev_ap)), None) => prepare_merge_result(prev_ap, Previous, data_keeper),
        // check that current state is Ap, but it's impossible to use it, because prev_data
        // could not have streams with such generations
        (None, Some(Ap(current_ap))) => prepare_merge_result(current_ap, Current, data_keeper),
        (None, None) => Ok(MergerApResult::NotMet),
        (prev_state, current_state) => Err(MergeError::incompatible_states(
            prev_state,
            current_state,
            EXPECTED_STATE_NAME,
        )),
    }
}

macro_rules! to_maybe_generation {
    ($ap_result:ident, $generations:expr, $error_ty:ident) => {
        match $generations.len() {
            1 => $generations[0],
            _ => {
                let ap_error = super::ApResultError::$error_ty($ap_result);
                return Err(super::MergeError::IncorrectApResult(ap_error));
            }
        }
    };
}

fn prepare_merge_result(
    ap_result: ApResult,
    scheme: PreparationScheme,
    data_keeper: &mut DataKeeper,
) -> MergeResult<MergerApResult> {
    prepare_positions_mapping(scheme, data_keeper);

    let generation = to_maybe_generation!(ap_result, &ap_result.res_generations, InvalidDstGenerations);
    let met_result = MetApResult::new(generation, scheme.into());
    let ap_result = MergerApResult::Met(met_result);

    Ok(ap_result)
}

impl MetApResult {
    pub(crate) fn new(generation: GenerationIdx, value_source: ValueSource) -> Self {
        Self {
            generation,
            value_source,
        }
    }
}
