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

#[derive(Clone, Debug)]
pub(crate) enum StateFSM {
    Par(ParFSM),
    Fold(FoldFSM),
}

#[derive(Debug, Default)]
pub(crate) struct FSMQueue {
    states: Vec<StateFSM>,
}

impl FSMQueue {
    pub(crate) fn push_fsm(&mut self, fsm: StateFSM) {
        self.states.push(fsm);
    }

    pub(crate) fn last_as_mut_par(&mut self) -> FSMResult<&mut ParFSM> {
        match self.states.last_mut().ok_or(StateFSMError::QueueIsEmpty("par"))? {
            StateFSM::Par(par) => Ok(par),
            fold @ StateFSM::Fold(_) => Err(StateFSMError::IncompatibleFSM("par", fold.clone())),
        }
    }

    pub(crate) fn last_as_mut_fold(&mut self) -> FSMResult<&mut FoldFSM> {
        match self.states.last_mut().ok_or(StateFSMError::QueueIsEmpty("fold"))? {
            par @ StateFSM::Par(_) => Err(StateFSMError::IncompatibleFSM("fold", par.clone())),
            StateFSM::Fold(fold) => Ok(fold),
        }
    }

    pub(crate) fn pop_as_par(&mut self) -> FSMResult<ParFSM> {
        match self.states.pop().ok_or(StateFSMError::QueueIsEmpty("par"))? {
            StateFSM::Par(par) => Ok(par),
            fold @ StateFSM::Fold(_) => Err(StateFSMError::IncompatibleFSM("par", fold)),
        }
    }

    pub(crate) fn pop_as_fold(&mut self) -> FSMResult<FoldFSM> {
        match self.states.pop().ok_or(StateFSMError::QueueIsEmpty("fold"))? {
            par @ StateFSM::Par(_) => Err(StateFSMError::IncompatibleFSM("fold", par)),
            StateFSM::Fold(fold) => Ok(fold),
        }
    }

    pub(crate) fn pop(&mut self) -> Option<StateFSM> {
        self.states.pop()
    }
}
