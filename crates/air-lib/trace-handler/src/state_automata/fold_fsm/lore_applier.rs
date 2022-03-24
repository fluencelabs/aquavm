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
use ByNextPosition::*;
use MergeCtxType::*;

/// Adjusts sliders accordingly to a before fold lore state.
pub(super) fn apply_fold_lore_before<VT: Clone>(
    data_keeper: &mut DataKeeper<VT>,
    prev_fold_lore: &Option<ResolvedSubTraceDescs>,
    current_fold_lore: &Option<ResolvedSubTraceDescs>,
) -> FSMResult<(), VT> {
    apply_fold_lore(data_keeper, prev_fold_lore, Previous, Before)?;
    apply_fold_lore(data_keeper, current_fold_lore, Current, Before)
}

/// Adjusts sliders accordingly to an after fold lore state.
pub(super) fn apply_fold_lore_after<VT: Clone>(
    data_keeper: &mut DataKeeper<VT>,
    prev_fold_lore: &Option<ResolvedSubTraceDescs>,
    current_fold_lore: &Option<ResolvedSubTraceDescs>,
) -> FSMResult<(), VT> {
    apply_fold_lore(data_keeper, prev_fold_lore, Previous, After)?;
    apply_fold_lore(data_keeper, current_fold_lore, Current, After)
}

fn apply_fold_lore<VT: Clone>(
    data_keeper: &mut DataKeeper<VT>,
    fold_lore: &Option<ResolvedSubTraceDescs>,
    ctx_type: MergeCtxType,
    next_position: ByNextPosition,
) -> FSMResult<(), VT> {
    let fold_lore = match fold_lore {
        Some(fold_lore) => fold_lore,
        None => return Ok(()),
    };

    let slider = match ctx_type {
        Previous => data_keeper.prev_slider_mut(),
        Current => data_keeper.current_slider_mut(),
    };

    match next_position {
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
    }
    Ok(())
}
