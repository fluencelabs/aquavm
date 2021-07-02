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

mod par_builder;
mod size_updater;

use super::*;
use par_builder::ParBuilder;
use size_updater::SubTraceSizeUpdater;

#[derive(Debug, Default, Clone)]
pub(crate) struct ParFSM {
    prev_par: Option<ParResult>,
    current_par: Option<ParResult>,
    state_inserter: StateInserter,
    size_updater: SubTraceSizeUpdater,
    par_builder: ParBuilder,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum SubtreeType {
    Left,
    Right,
}

macro_rules! par_left {
    ($par:expr) => {
        $par.map(|p| p.0).unwrap_or_default()
    };
}

macro_rules! par_right {
    ($par:expr) => {
        $par.map(|p| p.1).unwrap_or_default()
    };
}

impl ParFSM {
    pub(crate) fn new(ingredients: MergerParResult, data_keeper: &mut DataKeeper) -> FSMResult<Self> {
        let state_inserter = StateInserter::from_keeper(data_keeper);
        let size_updater = SubTraceSizeUpdater::from_data_keeper(data_keeper, ingredients)?;

        let par_fsm = Self {
            prev_par: ingredients.prev_par,
            current_par: ingredients.current_par,
            state_inserter,
            size_updater,
            ..<_>::default()
        };

        par_fsm.prepare_data(data_keeper, SubtreeType::Left)?;
        Ok(par_fsm)
    }

    pub(crate) fn left_completed(&mut self, data_keeper: &mut DataKeeper) -> FSMResult<()> {
        self.check_subtrace_lens(data_keeper, SubtreeType::Left)?;
        self.par_builder.track(data_keeper, SubtreeType::Left);
        self.prepare_data(data_keeper, SubtreeType::Right)?;

        Ok(())
    }

    pub(crate) fn right_completed(mut self, data_keeper: &mut DataKeeper) -> FSMResult<()> {
        self.check_subtrace_lens(data_keeper, SubtreeType::Right)?;
        self.par_builder.track(data_keeper, SubtreeType::Right);

        let state = self.par_builder.build();
        self.state_inserter.insert(data_keeper, state);
        self.size_updater.update(data_keeper)?;

        Ok(())
    }

    fn prepare_data(&self, data_keeper: &mut DataKeeper, subtree_type: SubtreeType) -> FSMResult<()> {
        let (prev_size, current_size) = match subtree_type {
            SubtreeType::Left => (par_left!(&self.prev_par), par_left!(&self.current_par)),
            SubtreeType::Right => (par_right!(&self.prev_par), par_right!(&self.current_par)),
        };

        data_keeper.prev_ctx.slider.set_subtrace_len(prev_size as usize)?;
        data_keeper.current_ctx.slider.set_subtrace_len(current_size as usize)?;

        Ok(())
    }

    /// Check that all values from interval were seen. Otherwise it's a error points out
    /// that a trace contains more values in a left or right subtree of this par.
    fn check_subtrace_lens(&self, data_keeper: &DataKeeper, subtree_type: SubtreeType) -> FSMResult<()> {
        use StateFSMError::ParSubtreeNonExhausted as NonExhausted;

        let prev_len = data_keeper.prev_ctx.slider.subtrace_len();
        if prev_len != 0 {
            // unwrap is safe here because otherwise subtrace_len wouldn't be equal 0.
            return Err(NonExhausted(subtree_type, self.prev_par.unwrap(), prev_len));
        }

        let current_len = data_keeper.current_ctx.slider.subtrace_len();
        if current_len != 0 {
            return Err(NonExhausted(subtree_type, self.current_par.unwrap(), current_len));
        }

        Ok(())
    }
}

use std::fmt;

impl fmt::Display for SubtreeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubtreeType::Left => write!(f, "left"),
            SubtreeType::Right => write!(f, "right"),
        }
    }
}
