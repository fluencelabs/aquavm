/*
 * Copyright 2020 Fluence Labs Limited
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

// mod trace_handler;
mod utils;

use super::ExecutionCtx;
use super::ExecutionError;
use super::ExecutionResult;
use super::ExecutionTraceCtx;
use super::Instruction;
use crate::contexts::execution::AValue;
use crate::contexts::execution::ResolvedCallResult;
use crate::exec_err;
use crate::execution::boxed_value::*;
use crate::log_instruction;

use air_parser::ast::Fold;
use air_parser::ast::Next;

use std::collections::HashMap;
use std::rc::Rc;

use utils::IterableValue;

pub(crate) struct FoldState<'i> {
    pub(crate) iterable: IterableValue,
    pub(crate) instr_head: Rc<Instruction<'i>>,
    // map of met variables inside this (not any inner) fold block with their initial values
    pub(crate) met_variables: HashMap<&'i str, ResolvedCallResult>,
}

impl<'i> FoldState<'i> {
    pub fn new(iterable: IterableValue, instr_head: Rc<Instruction<'i>>) -> Self {
        Self {
            iterable,
            instr_head,
            met_variables: HashMap::new(),
        }
    }
}

impl<'i> super::ExecutableInstruction<'i> for Fold<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut ExecutionTraceCtx) -> ExecutionResult<()> {
        use ExecutionError::MultipleFoldStates;

        log_instruction!(fold, exec_ctx, trace_ctx);

        let iterable = match utils::construct_iterable_value(&self.iterable, exec_ctx)? {
            Some(iterable) => iterable,
            None => return Ok(()),
        };

        let fold_state = FoldState::new(iterable, self.instruction.clone());

        let previous_value = exec_ctx
            .data_cache
            .insert(self.iterator.to_string(), AValue::JValueFoldCursor(fold_state));

        if previous_value.is_some() {
            return exec_err!(MultipleFoldStates(self.iterator.to_string()));
        }
        exec_ctx.met_folds.push_back(self.iterator);

        self.instruction.execute(exec_ctx, trace_ctx)?;

        cleanup_variables(exec_ctx, &self.iterator);

        Ok(())
    }
}

impl<'i> super::ExecutableInstruction<'i> for Next<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut ExecutionTraceCtx) -> ExecutionResult<()> {
        use ExecutionError::FoldStateNotFound;
        use ExecutionError::IncompatibleAValueType;

        log_instruction!(next, exec_ctx, trace_ctx);

        let iterator_name = self.0;
        let avalue = exec_ctx
            .data_cache
            .get_mut(iterator_name)
            .ok_or_else(|| FoldStateNotFound(iterator_name.to_string()))?;

        let fold_state = match avalue {
            AValue::JValueFoldCursor(state) => state,
            v => {
                // it's not possible to use unreachable here
                // because at now next syntactically could be used without fold
                return exec_err!(IncompatibleAValueType(
                    format!("{}", v),
                    String::from("JValueFoldCursor"),
                ));
            }
        };

        if !fold_state.iterable.next() {
            // just do nothing to exit
            return Ok(());
        }

        let next_instr = fold_state.instr_head.clone();
        next_instr.execute(exec_ctx, trace_ctx)?;

        // get the same fold state again because of borrow checker
        match exec_ctx.data_cache.get_mut(iterator_name) {
            // move iterator back to provide correct value for possible subtree after next
            // (for example for cases such as right fold)
            Some(AValue::JValueFoldCursor(fold_state)) => fold_state.iterable.prev(),
            _ => unreachable!("iterator value shouldn't changed inside fold"),
        };

        Ok(())
    }
}

fn cleanup_variables(exec_ctx: &mut ExecutionCtx<'_>, iterator: &str) {
    let fold_state = match exec_ctx.data_cache.remove(iterator) {
        Some(AValue::JValueFoldCursor(fold_state)) => fold_state,
        _ => unreachable!("fold cursor is changed only inside fold block"),
    };

    for (variable_name, _) in fold_state.met_variables {
        exec_ctx.data_cache.remove(variable_name);
    }
    exec_ctx.met_folds.pop_back();

    // TODO: fix 3 or more inner folds behaviour
    if let Some(fold_block_name) = exec_ctx.met_folds.back() {
        let fold_state = match exec_ctx.data_cache.get(*fold_block_name) {
            Some(AValue::JValueFoldCursor(fold_state)) => fold_state,
            _ => unreachable!("fold block data must be represented as fold cursor"),
        };

        let mut upper_fold_values = HashMap::new();
        for (variable_name, variable) in fold_state.met_variables.iter() {
            upper_fold_values.insert(variable_name.to_string(), AValue::JValueRef(variable.clone()));
        }

        exec_ctx.data_cache.extend(upper_fold_values);
    }
}
