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
