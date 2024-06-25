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

mod fold_lore_resolver;

use super::*;
pub use fold_lore_resolver::*;

#[derive(Debug, Default, Clone)]
pub struct MergerFoldResult {
    pub prev_fold_lore: ResolvedFold,
    pub current_fold_lore: ResolvedFold,
}

pub(crate) fn try_merge_next_state_as_fold(data_keeper: &mut DataKeeper) -> MergeResult<MergerFoldResult> {
    use ExecutedState::Fold;

    let prev_state = data_keeper.prev_slider_mut().next_state();
    let current_state = data_keeper.current_slider_mut().next_state();

    let fold_result = match (prev_state, current_state) {
        (Some(Fold(prev_fold)), Some(Fold(current_fold))) => {
            MergerFoldResult::from_fold_results(&prev_fold, &current_fold, data_keeper)
        }
        (None, Some(Fold(current_fold))) => {
            MergerFoldResult::from_fold_result(&current_fold, MergeCtxType::Current, data_keeper)
        }
        (Some(Fold(prev_fold)), None) => {
            MergerFoldResult::from_fold_result(&prev_fold, MergeCtxType::Previous, data_keeper)
        }
        (None, None) => return Ok(MergerFoldResult::default()),
        (prev_state, current_state) => return Err(MergeError::incompatible_states(prev_state, current_state, "fold")),
    }?;

    Ok(fold_result)
}

impl MergerFoldResult {
    fn from_fold_result(fold: &FoldResult, ctx_type: MergeCtxType, data_keeper: &DataKeeper) -> MergeResult<Self> {
        let (prev_fold_lore, current_fold_lore) = match ctx_type {
            MergeCtxType::Previous => {
                let fold_lore = resolve_fold_lore(fold, &data_keeper.prev_ctx)?;
                (fold_lore, <_>::default())
            }
            MergeCtxType::Current => {
                let fold_lore = resolve_fold_lore(fold, &data_keeper.current_ctx)?;
                (<_>::default(), fold_lore)
            }
        };

        let merge_result = Self {
            prev_fold_lore,
            current_fold_lore,
        };

        Ok(merge_result)
    }

    fn from_fold_results(
        prev_fold: &FoldResult,
        current_fold: &FoldResult,
        data_keeper: &DataKeeper,
    ) -> MergeResult<Self> {
        let prev_fold_lore = resolve_fold_lore(prev_fold, &data_keeper.prev_ctx)?;
        let current_fold_lore = resolve_fold_lore(current_fold, &data_keeper.current_ctx)?;

        let merge_result = Self {
            prev_fold_lore,
            current_fold_lore,
        };

        Ok(merge_result)
    }
}
