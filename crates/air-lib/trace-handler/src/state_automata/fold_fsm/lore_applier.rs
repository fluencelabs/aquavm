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
use ByNextPosition::*;
use MergeCtxType::*;

/// Adjusts sliders accordingly to a before fold lore state.
pub(super) fn apply_fold_lore_before(
    data_keeper: &mut DataKeeper,
    prev_fold_lore: &Option<ResolvedSubTraceDescs>,
    current_fold_lore: &Option<ResolvedSubTraceDescs>,
) -> FSMResult<()> {
    apply_fold_lore(data_keeper, prev_fold_lore, Previous, Before)?;
    apply_fold_lore(data_keeper, current_fold_lore, Current, Before)
}

/// Adjusts sliders accordingly to an after fold lore state.
pub(super) fn apply_fold_lore_after(
    data_keeper: &mut DataKeeper,
    prev_fold_lore: &Option<ResolvedSubTraceDescs>,
    current_fold_lore: &Option<ResolvedSubTraceDescs>,
) -> FSMResult<()> {
    apply_fold_lore(data_keeper, prev_fold_lore, Previous, After)?;
    apply_fold_lore(data_keeper, current_fold_lore, Current, After)
}

fn apply_fold_lore(
    data_keeper: &mut DataKeeper,
    fold_lore: &Option<ResolvedSubTraceDescs>,
    ctx_type: MergeCtxType,
    next_position: ByNextPosition,
) -> FSMResult<()> {
    let slider = match ctx_type {
        Previous => data_keeper.prev_slider_mut(),
        Current => data_keeper.current_slider_mut(),
    };

    match fold_lore {
        Some(fold_lore) => match next_position {
            Before => {
                slider.set_position_and_len(
                    fold_lore.before_subtrace.begin_pos as _,
                    fold_lore.before_subtrace.subtrace_len as _,
                )?;
            }
            After => {
                slider.set_position_and_len(
                    fold_lore.after_subtrace.begin_pos as _,
                    fold_lore.after_subtrace.subtrace_len as _,
                )?;
            }
        },
        None => {
            // substrace is empty
            slider.set_subtrace_len(0)?;
        }
    }

    Ok(())
}
