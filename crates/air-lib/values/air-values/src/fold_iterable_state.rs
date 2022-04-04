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

use super::Instruction;
use super::Iterable;
use super::IterableValue;
use super::RcSecurityTetraplet;
use super::ValueAggregate;

use air_parser::ast::Instruction;
use air_value::BoxedValue;

use std::rc::Rc;

pub struct FoldIterableState<'i> {
    pub iterable: IterableValue,
    pub iterable_type: IterableType,
    pub instr_head: Rc<Instruction<'i>>,
}

pub type IterableValue = Box<dyn for<'ctx> Iterable<'ctx, Item = FoldIterableState<'ctx>>>;

pub struct IterableItem<'ctx> {
    value: &'ctx dyn JValue,
    tetraplet: &'ctx RcSecurityTetraplet,
    position: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IterableType {
    Scalar,
    Stream(u32),
}

impl<'i, T> FoldIterableState<'i> {
    pub(crate) fn from_iterable(
        iterable: IterableValue,
        iterable_type: IterableType,
        instr_head: Rc<Instruction<'i>>,
    ) -> Self {
        Self {
            iterable,
            iterable_type,
            instr_head,
        }
    }
}

impl IterableItem<'_> {
    pub(crate) fn into_resolved_result(self) -> ValueAggregate {
        ValueAggregate::new(value, tetraplet, pos)
    }
}
