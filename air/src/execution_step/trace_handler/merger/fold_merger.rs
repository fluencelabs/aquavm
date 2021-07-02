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

mod utils;

use super::*;
use crate::JValue;
pub(crate) use utils::*;

use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub(crate) struct MergerFoldResult {
    pub(crate) prev_fold_lore: ResolvedFoldLore,
    pub(crate) current_fold_lore: ResolvedFoldLore,
}

pub(crate) fn try_merge_next_state_as_fold(data_keeper: &mut DataKeeper) -> MergeResult<MergerFoldResult> {
    use ExecutedState::Fold;

    let prev_state = data_keeper.prev_ctx.slider.next_state();
    let current_state = data_keeper.current_ctx.slider.next_state();

    let fold_result = match (prev_state, current_state) {
        (Some(Fold(prev_fold)), Some(Fold(current_fold))) => {
            MergerFoldResult::from_fold_results(&prev_fold, &current_fold, data_keeper)
        }
        (None, Some(Fold(current_fold))) => {
            MergerFoldResult::from_fold_result(&current_fold, data_keeper, MergeCtxType::Current)
        }
        (Some(Fold(prev_fold)), None) => {
            MergerFoldResult::from_fold_result(&prev_fold, data_keeper, MergeCtxType::Previous)
        }
        (None, None) => return Ok(MergerFoldResult::default()),
        (prev_state, current_state) => return Err(MergeError::incompatible_states(prev_state, current_state, "fold")),
    }?;

    Ok(fold_result)
}

impl MergerFoldResult {
    pub(self) fn from_fold_result(
        fold: &FoldResult,
        data_keeper: &DataKeeper,
        ctx_type: MergeCtxType,
    ) -> MergeResult<Self> {
        let (prev_fold_lore, current_fold_lore) = match ctx_type {
            MergeCtxType::Previous => {
                let fold_lore = resolve_fold_lore(&data_keeper.prev_ctx.slider, fold)?;
                (fold_lore, <_>::default())
            }
            MergeCtxType::Current => {
                let fold_lore = resolve_fold_lore(&data_keeper.current_ctx.slider, fold)?;
                (<_>::default(), fold_lore)
            }
        };

        let merge_result = Self {
            prev_fold_lore,
            current_fold_lore,
        };

        Ok(merge_result)
    }

    pub(self) fn from_fold_results(
        prev_fold: &FoldResult,
        current_fold: &FoldResult,
        data_keeper: &DataKeeper,
    ) -> MergeResult<Self> {
        let prev_fold_lore = resolve_fold_lore(&data_keeper.prev_ctx.slider, prev_fold)?;
        let current_fold_lore = resolve_fold_lore(&data_keeper.current_ctx.slider, current_fold)?;

        let merge_result = Self {
            prev_fold_lore,
            current_fold_lore,
        };

        Ok(merge_result)
    }
}
