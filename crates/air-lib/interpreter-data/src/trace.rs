/*
 * Copyright 2022 Fluence Labs Limited
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
