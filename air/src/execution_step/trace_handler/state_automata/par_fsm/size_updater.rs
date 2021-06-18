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

#[derive(Debug, Default)]
pub(super) struct SubtreeSizeUpdater {
    pub(self) prev_size: usize,
    pub(self) current_size: usize,
}

impl SubtreeSizeUpdater {
    pub(super) fn from_data_keeper(data_keeper: &DataKeeper, ingredients: MergerParResult) -> FSMResult<Self> {
        let prev_subtree_size = data_keeper.prev_ctx.slider.interval_len();
        let prev_size = Self::compute_new_size(prev_subtree_size, ingredients.prev_par)?;

        let current_subtree_size = data_keeper.current_ctx.slider.interval_len();
        let current_size = Self::compute_new_size(current_subtree_size, ingredients.current_par)?;

        let updater = Self {
            prev_size,
            current_size,
        };

        Ok(updater)
    }

    pub(super) fn update(self, data_keeper: &mut DataKeeper) {
        data_keeper.prev_ctx.slider.set_interval_len(self.prev_size);
        data_keeper.current_ctx.slider.set_interval_len(self.current_size);
    }

    fn compute_new_size(initial_size: usize, par_result: Option<ParResult>) -> FSMResult<usize> {
        let par_size = par_result
            .map(|p| p.size().ok_or_else(|| StateFSMError::ParLenOverflow(p.clone())))
            .transpose()?
            .unwrap_or_default();

        let new_size = initial_size
            .checked_sub(par_size)
            // unwrap is safe here, because underflow could be caused only if par is Some
            .ok_or_else(|| StateFSMError::ParSubtreeUnderflow(par_result.unwrap(), initial_size))?;

        Ok(new_size)
    }
}
