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

use super::Instruction;
use super::IterableValue;

use std::rc::Rc;

pub(crate) struct FoldState<'i> {
    pub(crate) iterable: IterableValue,
    pub(crate) iterable_type: IterableType,
    // true of iterator exhausted and reverse execution started
    pub(crate) back_iteration_started: bool,
    pub(crate) instr_head: Rc<Instruction<'i>>,
    pub(crate) last_instr_head: Option<Rc<Instruction<'i>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum IterableType {
    Scalar,
    Stream(u32),
}

impl<'i> FoldState<'i> {
    pub(crate) fn from_iterable(
        iterable: IterableValue,
        iterable_type: IterableType,
        instr_head: Rc<Instruction<'i>>,
        last_instr_head: Option<Rc<Instruction<'i>>>,
    ) -> Self {
        Self {
            iterable,
            iterable_type,
            back_iteration_started: false,
            instr_head,
            last_instr_head,
        }
    }
}
