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

pub(super) fn apply_fold_lore_before(
    data_keeper: &mut DataKeeper,
    prev_fold_lore: &Option<ResolvedFoldSubTraceLore>,
    current_fold_lore: &Option<ResolvedFoldSubTraceLore>,
) -> FSMResult<()> {
    apply_fold_lore(
        data_keeper,
        prev_fold_lore,
        MergeCtxType::Previous,
        ByNextPosition::Before,
    )?;
    apply_fold_lore(
        data_keeper,
        current_fold_lore,
        MergeCtxType::Current,
        ByNextPosition::Before,
    )
}

pub(super) fn apply_fold_lore_after(
    data_keeper: &mut DataKeeper,
    prev_fold_lore: &Option<ResolvedFoldSubTraceLore>,
    current_fold_lore: &Option<ResolvedFoldSubTraceLore>,
) -> FSMResult<()> {
    apply_fold_lore(
        data_keeper,
        prev_fold_lore,
        MergeCtxType::Previous,
        ByNextPosition::After,
    )?;
    apply_fold_lore(
        data_keeper,
        current_fold_lore,
        MergeCtxType::Current,
        ByNextPosition::After,
    )
}

fn apply_fold_lore(
    data_keeper: &mut DataKeeper,
    fold_lore: &Option<ResolvedFoldSubTraceLore>,
    ctx_type: MergeCtxType,
    next_position: ByNextPosition,
) -> FSMResult<()> {
    let fold_lore = match fold_lore {
        Some(fold_lore) => fold_lore,
        None => return Ok(()),
    };

    let slider = match ctx_type {
        MergeCtxType::Previous => &data_keeper.prev_ctx.slider,
        MergeCtxType::Current => &data_keeper.current_ctx.slider,
    };

    match next_position {
        ByNextPosition::Before => {
            slider.set_interval_len(fold_lore.interval_len.before)?;
            slider.set_position(fold_lore.begin_pos.before)?;
        }
        ByNextPosition::After => {
            slider.set_interval_len(fold_lore.interval_len.after)?;
            slider.set_position(fold_lore.begin_pos.after)?;
        }
    }
    Ok(())
}

enum ByNextPosition {
    Before,
    After,
}
