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

use super::ExecutedState;
use crate::TracePos;

use serde::Deserialize;
use serde::Serialize;
use std::convert::TryInto;
use std::ops::Deref;
use std::ops::Index;
use std::ops::IndexMut;

pub type TraceLen = u32;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize)]
#[archive(check_bytes)]
pub struct ExecutionTrace(Vec<ExecutedState>);

impl ExecutionTrace {
    pub fn get(&self, index: TracePos) -> Option<&ExecutedState> {
        self.0.get(usize::from(index))
    }

    pub fn get_mut(&mut self, index: TracePos) -> Option<&mut ExecutedState> {
        self.0.get_mut(usize::from(index))
    }

    pub fn pop(&mut self) -> Option<ExecutedState> {
        self.0.pop()
    }

    pub fn push(&mut self, value: ExecutedState) {
        self.0.push(value);
    }

    pub fn trace_states_count(&self) -> TraceLen {
        self.0
            .len()
            .try_into()
            .expect("there is an overflow in trace_states_count().")
    }
}

impl Deref for ExecutionTrace {
    type Target = [ExecutedState];

    fn deref(&self) -> &[ExecutedState] {
        &self.0
    }
}

impl From<Vec<ExecutedState>> for ExecutionTrace {
    fn from(vec: Vec<ExecutedState>) -> Self {
        Self(vec)
    }
}

impl Index<TracePos> for ExecutionTrace {
    type Output = ExecutedState;

    fn index(&self, index: TracePos) -> &Self::Output {
        &self.deref()[usize::from(index)]
    }
}

impl IndexMut<TracePos> for ExecutionTrace {
    fn index_mut(&mut self, index: TracePos) -> &mut Self::Output {
        &mut self.0[usize::from(index)]
    }
}

impl PartialEq<Vec<ExecutedState>> for ExecutionTrace {
    fn eq(&self, other: &Vec<ExecutedState>) -> bool {
        &self.0 == other
    }
}

impl<'trace> IntoIterator for &'trace ExecutionTrace {
    type Item = &'trace ExecutedState;

    type IntoIter = <&'trace Vec<ExecutedState> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
