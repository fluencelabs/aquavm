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
mod state_handler;

use super::*;
use par_builder::ParBuilder;
use state_handler::CtxStateHandler;

/// Manages a par state, its state transitioning functions must be called in the following way:
///   from_left_started
///     -> left_completed(_with_error)
///     -> right_started
///     -> right_completed(_with_error)
#[derive(Debug, Default, Clone)]
pub(crate) struct ParFSM {
    prev_par: ParResult,
    current_par: ParResult,
    state_inserter: StateInserter,
    state_handler: CtxStateHandler,
    par_builder: ParBuilder,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum SubtreeType {
    Left,
    Right,
}

impl ParFSM {
    pub(crate) fn from_left_started(ingredients: MergerParResult, data_keeper: &mut DataKeeper) -> FSMResult<Self> {
        // default is a par with empty left and right subtrees
        let prev_par = ingredients.prev_par.unwrap_or_default();
        let current_par = ingredients.current_par.unwrap_or_default();

        let state_inserter = StateInserter::from_keeper(data_keeper);
        let state_updater = CtxStateHandler::prepare(prev_par, current_par, data_keeper)?;
        let par_builder = ParBuilder::from_keeper(data_keeper, &state_inserter);

        let par_fsm = Self {
            prev_par,
            current_par,
            state_inserter,
            state_handler: state_updater,
            par_builder,
        };

        par_fsm.prepare_sliders(data_keeper, SubtreeType::Left)?;

        Ok(par_fsm)
    }

    pub(crate) fn left_completed(&mut self, data_keeper: &mut DataKeeper) {
        self.par_builder.track(data_keeper, SubtreeType::Left);
        self.state_handler.handle_subtree_end(data_keeper, SubtreeType::Left);

        // all invariants were checked in the ctor
        let _ = self.prepare_sliders(data_keeper, SubtreeType::Right);
    }

    pub(crate) fn right_completed(mut self, data_keeper: &mut DataKeeper) {
        self.par_builder.track(data_keeper, SubtreeType::Right);
        let state = self.par_builder.build();
        self.state_inserter.insert(data_keeper, state);

        self.state_handler.handle_subtree_end(data_keeper, SubtreeType::Right);
    }

    fn prepare_sliders(&self, data_keeper: &mut DataKeeper, subtree_type: SubtreeType) -> FSMResult<()> {
        let (prev_len, current_len) = match subtree_type {
            SubtreeType::Left => (self.prev_par.left_subtree_size, self.current_par.left_subtree_size),
            SubtreeType::Right => (self.prev_par.right_subtree_size, self.current_par.right_subtree_size),
        };

        data_keeper.prev_slider_mut().set_subtrace_len(prev_len as _)?;
        data_keeper.current_slider_mut().set_subtrace_len(current_len as _)?;

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
