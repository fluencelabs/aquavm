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

use crate::exec_err;
use crate::execution_step::boxed_value::Scalar;
use crate::execution_step::ExecutionError;
use crate::execution_step::ExecutionResult;
use crate::execution_step::FoldState;
use crate::execution_step::ResolvedCallResult;

use std::collections::HashMap;
use std::rc::Rc;

/// There are two scopes for scalars in AIR: global and local. A local scope is a scope
/// inside every fold block, other scope is a global. It means that scalar in an upper
/// fold block could be shadowed by a scalar with the same name in a lower fold block,
/// it works "as expected". Let's consider the following example:
/// (seq
///   (seq
///     (call ... local) ;; (1)
///     (fold iterable_1 iterator_1
///       (seq
///         (seq
///           (seq
///             (call ... local) ;; (2)
///             (fold iterable_2 iterator_2
///               (seq
///                 (seq
///                    (call ... local) ;; (3)
///                    (call ... [local]) ;; local set by (3) will be used
///                  )
///                  (next iterator_2)
///               )
///             )
///           )
///           (call ... [local]) ;; local set by (2) will be used
///         )
///         (next iterator_1)
///       )
///     )
///   )
///   (seq
///     (call ... [local]) ;; local set by (1) will be used
///     (call ... local) ;; error will be occurred because, it's impossible to set variable twice
///                      ;; in a global scope
///   )
/// )
///
/// This struct is intended to provide abilities to work with scalars as it was described.
#[derive(Default)]
pub(crate) struct Scalars<'i> {
    // this one is optimized for speed (not for memory), because it's unexpected
    // that a script could have a lot of inner folds.
    pub variables: HashMap<String, Vec<Option<Scalar<'i>>>>,
    pub fold_block_id: usize,
}

impl<'i> Scalars<'i> {
    pub(crate) fn set_jvalue(
        &mut self,
        name: impl Into<String>,
        call_result: ResolvedCallResult,
    ) -> ExecutionResult<()> {
        self.set(name, Scalar::JValueRef(call_result))
    }

    pub(crate) fn set_iterable(&mut self, name: impl Into<String>, fold_state: FoldState<'i>) -> ExecutionResult<()> {
        self.set(name, Scalar::JValueFoldCursor(fold_state))
    }

    pub(crate) fn set(&mut self, name: impl Into<String>, scalar: Scalar<'i>) -> ExecutionResult<()> {
        use std::collections::hash_map::Entry::{Occupied, Vacant};

        match self.variables.entry(name.into()) {
            Vacant(entry) => {
                let mut scalars = vec![None; self.fold_block_id];
                scalars.push(Some(scalar));
                entry.insert(scalars);
            }
            Occupied(entry) => {
                if !self.shadowing_allowed() {
                    return exec_err!(ExecutionError::MultipleVariablesFound(entry.key().clone()));
                }

                let scalars = entry.into_mut();
                scalars[self.fold_block_id] = Some(scalar);
            }
        }

        Ok(())
    }

    pub(crate) fn get(&'i self, name: &str) -> ExecutionResult<&'i Scalar<'i>> {
        let scalars = self
            .variables
            .get(name)
            .ok_or_else(|| Rc::new(ExecutionError::VariableNotFound(name.to_string())))?;

        last_not_none(scalars.iter(), self.fold_block_id)
            .ok_or_else(|| Rc::new(ExecutionError::VariableNotFound(name.to_string())))
    }

    pub(crate) fn meet_fold_begin(&mut self) {
        self.fold_block_id += 1;
    }

    pub(crate) fn meet_fold_end(&mut self) {
        self.fold_block_id -= 1;
    }

    fn shadowing_allowed(&self) -> bool {
        // shadowing is allowed only inside a fold block, 0 here means that execution flow
        // is in a global scope
        self.fold_block_id != 0
    }
}

// finds the last non none value on the interval 0..fold_block_id
fn last_not_none<'scalar, 'input>(
    scalars: impl Iterator<Item = &'scalar Option<Scalar<'input>>>,
    fold_block_id: usize,
) -> Option<&'scalar Scalar<'input>> {
    scalars
        .iter()
        .take(fold_block_id)
        .rev()
        .find_map(|scalar| scalar.as_ref().clone())
}

use std::fmt;

impl fmt::Display for Scalars {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (name, value) in self.variables.iter() {
            let last_value = last_not_none(value.iter(), self.fold_block_id);
            match last_value {
                Some(Scalar::JValueRef(last_value)) => writeln!(f, "{} => {}", name, last_value.result)?,
                _ => {}
            }
        }

        Ok(())
    }
}
