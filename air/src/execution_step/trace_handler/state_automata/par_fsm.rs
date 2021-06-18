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

mod size_tracker;
mod size_updater;

use super::*;
use size_tracker::SubtreeSizeTracker;
use size_updater::SubtreeSizeUpdater;

#[derive(Debug, Default)]
pub(crate) struct ParFSM {
    // position of stub par state in trace
    position: usize,
    prev_par: Option<ParResult>,
    current_par: Option<ParResult>,
    initial_subtree_sizes: SubtreeSizeUpdater,
    size_tracker: SubtreeSizeTracker,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum SubtreeType {
    Left,
    Right,
}

macro_rules! par_left {
    ($par:ident) => {
        $par.map(|p| p.0).unwrap_or_default()
    };
}

macro_rules! par_right {
    ($par:ident) => {
        $par.map(|p| p.1).unwrap_or_default()
    };
}

impl ParFSM {
    pub(crate) fn new(ingredients: MergerParResult, data_keeper: &mut DataKeeper) -> FSMResult<Self> {
        let position = data_keeper.result_trace.len();
        trace.push(ExecutedState::par(0, 0));
        let initial_subtree_sizes = SubtreeSizeUpdater::from_data_keeper(data_keeper, ingredients)?;

        let par_fsm = Self {
            position,
            prev_par: ingredients.prev_par,
            current_par: ingredients.current_par,
            initial_subtree_sizes,
            ..<_>::default()
        };

        par_fsm.prepare_data(data_keeper, SubtreeType::Left);
        Ok(par_fsm)
    }

    pub(crate) fn left_completed(&mut self, data_keeper: &mut DataKeeper) -> FSMResult<()> {
        self.check_intervals(data_keeper, SubtreeType::Left)?;
        self.size_tracker.update(data_keeper, SubtreeType::Left);
        self.prepare_data(data_keeper, SubtreeType::Right);

        Ok(())
    }

    pub(crate) fn right_completed(mut self, data_keeper: &mut DataKeeper) -> FSMResult<()> {
        self.check_intervals(data_keeper, SubtreeType::Right)?;
        self.size_tracker.update(data_keeper, SubtreeType::Right);

        let state = self.size_tracker.into_par();
        trace[self.position] = state;

        Ok(())
    }

    fn prepare_data(&self, data_keeper: &mut DataKeeper, subtree_type: SubtreeType) {
        let (prev_size, current_size) = match subtree_type {
            SubtreeType::Left => (par_left!(&self.prev_par), par_left!(&self.current_par)),
            SubtreeType::Right => (par_right!(&self.prev_par), par_right!(&self.current_par)),
        };

        data_keeper.prev_ctx.slider.set_interval_len(prev_size);
        data_keeper.prev_ctx.slider.set_interval_len(current_size);
    }

    fn check_intervals(&self, data_keeper: &DataKeeper, subtree_type: SubtreeType) -> FSMResult<()> {
        use StateFSMError::ParSubtreeNonExhausted as NonExhausted;

        let prev_len = data_keeper.prev_ctx.slider.interval_len();
        if prev_len != 0 {
            // unwrap is safe here because otherwise interval_len wouldn't be equal 0.
            return Err(NonExhausted(subtree_type, self.prev_par.unwrap(), prev_len));
        }

        let current_len = data_keeper.prev_ctx.slider.interval_len();
        if current_len != 0 {
            return Err(NonExhausted(subtree_type, self.current_par.unwrap(), current_len));
        }

        Ok(())
    }
}

use std::fmt;
use std::fmt::Formatter;

impl fmt::Display for SubtreeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubtreeType::Left => write!(f, "left"),
            SubtreeType::Right => write!(f, "right"),
        }
    }
}
