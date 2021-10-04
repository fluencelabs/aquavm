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
        self.par_stack
            .last_mut()
            .ok_or(StateFSMError::ParQueueIsEmpty)
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
