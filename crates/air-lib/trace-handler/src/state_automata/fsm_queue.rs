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

use super::FSMResult;
use super::FoldFSM;
use super::ParFSM;
use super::StateFSMError;

use std::collections::HashMap;

#[derive(Debug, Default)]
pub(crate) struct FSMKeeper {
    par_stack: Vec<ParFSM>,
    fold_map: HashMap<u32, FoldFSM>,
}

impl FSMKeeper {
    pub(crate) fn push_par(&mut self, par_fsm: ParFSM) {
        self.par_stack.push(par_fsm);
    }

    pub(crate) fn add_fold(&mut self, fold_id: u32, fold_fsm: FoldFSM) {
        self.fold_map.insert(fold_id, fold_fsm);
    }

    pub(crate) fn last_par(&mut self) -> FSMResult<&mut ParFSM> {
        self.par_stack.last_mut().ok_or(StateFSMError::ParQueueIsEmpty)
    }

    pub(crate) fn pop_par(&mut self) -> FSMResult<ParFSM> {
        self.par_stack.pop().ok_or(StateFSMError::ParQueueIsEmpty)
    }

    pub(crate) fn fold_mut(&mut self, fold_id: u32) -> FSMResult<&mut FoldFSM> {
        self.fold_map
            .get_mut(&fold_id)
            .ok_or(StateFSMError::FoldFSMNotFound(fold_id))
    }

    pub(crate) fn extract_fold(&mut self, fold_id: u32) -> FSMResult<FoldFSM> {
        self.fold_map
            .remove(&fold_id)
            .ok_or(StateFSMError::FoldFSMNotFound(fold_id))
    }
}
