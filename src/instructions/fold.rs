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

use super::ExecutionContext;
use super::Instruction;
use crate::AquamarineError;
use crate::Result;
use crate::SerdeValue;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::collections::LinkedList;
use std::rc::Rc;

/*
 (fold Iterable i
   (par
     (call fn [i] acc[])
     (next i)
   )
 )
*/

#[derive(Debug, Clone)]
pub(crate) struct FoldState {
    // TODO: make it store a ref to context value
    pub iterable: Vec<SerdeValue>,
    pub cursor: usize,
    pub instr_head: Rc<Instruction>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct Fold(String, String, Rc<Instruction>);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct Next(String);

impl super::ExecutableInstruction for Fold {
    fn execute(&self, ctx: &mut ExecutionContext) -> Result<()> {
        log::info!("fold {:?} is called with context {:?}", self, ctx);

        let iterable_name = &self.0;
        let iterable_variable_name = &self.1;
        let instr_head = self.2.clone();

        let iterable = ctx
            .data
            .get(iterable_name)
            .ok_or_else(|| AquamarineError::VariableNotFound(String::from(iterable_name)))?;

        let iterable = match iterable {
            SerdeValue::Array(json_array) => json_array.clone(),
            v => {
                return Err(AquamarineError::VariableIsNotArray(
                    v.clone(),
                    iterable_name.clone(),
                ))
            }
        };

        let fold_state = FoldState {
            iterable,
            cursor: 0,
            instr_head: instr_head.clone(),
        };

        ctx.folds.insert(iterable_variable_name.clone(), fold_state);

        instr_head.execute(ctx)?;

        ctx.folds.remove(iterable_variable_name);

        Ok(())
    }
}

impl super::ExecutableInstruction for Next {
    fn execute(&self, ctx: &mut ExecutionContext) -> Result<()> {
        log::info!("next {:?} is called with context {:?}", self, ctx);

        let iterable_variable_name = &self.0;
        let fold_state = ctx
            .folds
            .get_mut(iterable_variable_name)
            .ok_or_else(|| AquamarineError::FoldStateNotFound(iterable_variable_name.clone()))?;

        if fold_state.iterable.len() >= fold_state.cursor {
            // the only thing is needed here - is just to pass
            return Ok(());
        }

        fold_state.cursor = fold_state.cursor + 1;

        let next_instr = fold_state.instr_head.clone();
        next_instr.execute(ctx)?;

        // here it's need to getting fold state again because of borrow checker
        let fold_state = ctx
            .folds
            .get_mut(iterable_variable_name)
            .expect("fold state is deleted only after fold finishing");

        fold_state.cursor = fold_state.cursor - 1;

        Ok(())
    }
}
