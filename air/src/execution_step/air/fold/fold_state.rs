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

use super::AValue;
use super::ExecutionCtx;
use super::ExecutionError;
use super::ExecutionResult;
use super::Instruction;
use super::IterableValue;
use super::ResolvedCallResult;
use super::TraceHandler;
use crate::exec_err;
use crate::execution_step::boxed_value::*;
use crate::log_instruction;

use air_parser::ast;

use std::collections::HashMap;
use std::rc::Rc;

pub(crate) struct FoldState<'i> {
    pub(crate) iterable: IterableValue,
    pub(crate) instr_head: Rc<Instruction<'i>>,
    // map of met variables inside this (not any inner) fold block with their initial values
    pub(crate) met_variables: HashMap<&'i str, ResolvedCallResult>,
    pub(crate) is_iterable_stream: bool,
}

impl<'i> FoldState<'i> {
    pub(crate) fn from_iterable(
        iterable: IterableValue,
        instr_head: Rc<Instruction<'i>>,
        is_iterable_stream: bool,
    ) -> Self {
        Self {
            iterable,
            instr_head,
            met_variables: HashMap::new(),
            is_iterable_stream,
        }
    }
}
