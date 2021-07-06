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
use super::FoldState;
use super::TraceHandler;
use crate::exec_err;
use crate::log_instruction;

use air_parser::ast::Next;

impl<'i> super::ExecutableInstruction<'i> for Next<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(next, exec_ctx, trace_ctx);

        let iterator_name = self.0;
        let fold_state = try_get_fold_state(exec_ctx, iterator_name)?;

        if fold_state.is_iterable_stream {
            let next_state = fold_state.iterable.peek().unwrap();
            trace_ctx.meet_next(&next_state.as_value_and_pos())?;
        }

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

        // get this fold state the second time to bypass borrow checker
        let fold_state = try_get_fold_state(exec_ctx, iterator_name)?;
        if fold_state.is_iterable_stream {
            trace_ctx.meet_prev()?;
        }

        Ok(())
    }
}

fn try_get_fold_state<'i, 'ctx>(
    exec_ctx: &'ctx mut ExecutionCtx<'i>,
    iterator_name: &str,
) -> ExecutionResult<&'ctx mut FoldState<'i>> {
    use ExecutionError::FoldStateNotFound;
    use ExecutionError::IncompatibleAValueType;

    let avalue = exec_ctx
        .data_cache
        .get_mut(iterator_name)
        .ok_or_else(|| FoldStateNotFound(iterator_name.to_string()))?;

    match avalue {
        AValue::JValueFoldCursor(state) => Ok(state),
        v => {
            // it's not possible to use unreachable here
            // because at now next syntactically could be used without fold
            exec_err!(IncompatibleAValueType(
                format!("{}", v),
                String::from("JValueFoldCursor"),
            ))
        }
    }
}
