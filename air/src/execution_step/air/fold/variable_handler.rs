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

use super::ExecutionCtx;
use super::ExecutionResult;
use super::FoldState;
use super::Scalar;

use std::collections::HashMap;

pub(crate) struct VariableHandler<'i> {
    iterator: &'i str,
}

impl<'i> VariableHandler<'i> {
    pub(crate) fn init<'ctx: 'i>(
        exec_ctx: &mut ExecutionCtx<'ctx>,
        iterator: &'ctx str,
        fold_state: FoldState<'ctx>,
    ) -> ExecutionResult<Self> {
        Self::try_insert_fold_state(exec_ctx, iterator, fold_state)?;
        Self::meet_iterator(exec_ctx, iterator);

        let handler = Self { iterator };
        Ok(handler)
    }

    pub(crate) fn cleanup(self, exec_ctx: &mut ExecutionCtx<'_>) {
        let fold_state = match exec_ctx.scalars.remove(self.iterator) {
            Some(Scalar::JValueFoldCursor(fold_state)) => fold_state,
            _ => unreachable!("fold cursor is changed only inside fold block"),
        };

        for (variable_name, _) in fold_state.met_variables {
            exec_ctx.scalars.remove(variable_name);
        }
        exec_ctx.met_folds.pop_back();

        // TODO: fix 3 or more inner folds behaviour
        if let Some(fold_block_name) = exec_ctx.met_folds.back() {
            let fold_state = match exec_ctx.scalars.get(*fold_block_name) {
                Some(Scalar::JValueFoldCursor(fold_state)) => fold_state,
                _ => unreachable!("fold block data must be represented as fold cursor"),
            };

            let mut upper_fold_values = HashMap::new();
            for (variable_name, variable) in fold_state.met_variables.iter() {
                upper_fold_values.insert(variable_name.to_string(), Scalar::JValueRef(variable.clone()));
            }

            exec_ctx.scalars.extend(upper_fold_values);
        }
    }

    fn try_insert_fold_state<'ctx>(
        exec_ctx: &mut ExecutionCtx<'ctx>,
        iterator: &'ctx str,
        fold_state: FoldState<'ctx>,
    ) -> ExecutionResult<()> {
        use super::ExecutionError::MultipleFoldStates;

        let previous_value = exec_ctx
            .scalars
            .insert(iterator.to_string(), Scalar::JValueFoldCursor(fold_state));

        if previous_value.is_some() {
            return crate::exec_err!(MultipleFoldStates(iterator.to_string()));
        }

        Ok(())
    }

    fn meet_iterator<'ctx>(exec_ctx: &mut ExecutionCtx<'ctx>, iterator: &'ctx str) {
        exec_ctx.met_folds.push_back(iterator);
    }
}
