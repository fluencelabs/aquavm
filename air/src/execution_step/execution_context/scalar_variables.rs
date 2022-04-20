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

use crate::execution_step::boxed_value::ScalarRef;
use crate::execution_step::errors_prelude::*;
use crate::execution_step::ExecutionResult;
use crate::execution_step::FoldState;
use crate::execution_step::ValueAggregate;

use non_empty_vec::NonEmpty;

use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

// TODO: move this code snippet to documentation when it's ready

/// There are two scopes for variable scalars in AIR: global and local. A local scope
/// is a scope inside every fold block, other scope is a global. It means that scalar
/// in an upper fold block could be shadowed by a scalar with the same name in a lower
/// fold block, it works "as expected". Let's consider the following example:
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
/// Although there could be only one iterable value for a fold block, because of CRDT rules.
/// This struct is intended to provide abilities to work with scalars as it was described.
#[derive(Default)]
pub(crate) struct Scalars<'i> {
    // TODO: use Rc<String> to avoid copying
    /// Terminology used here (mainly to resolve concerns re difference between scalars and values):
    ///  - scalar is an AIR scalar, iterable and non iterable. A scalar is addressed by a name.
    ///  - value is concrete value assigned to scalar on certain depth
    ///  - scope is a variable scope where variable is visible. If we consider fold as a tree where
    ///     each next produces a new level, then scope is a level in this tree. Please note that it
    ///     includes variable defined after next instruction.
    ///  - depth is a count of seen scopes (or a depth in a tree met in the previous definition)
    ///
    /// Non iterable variables hash map could be recognized as a sparse matrix, where a row
    /// corresponds to a variable name and contains all its values were set with respect to a depth.
    /// A column corresponds to a depth and contains all values were set at current depth.
    ///
    /// This matrix follows these invariants:
    ///   - all rows are non empty
    ///   - global variables have 0 depth
    ///   - cells in a row are sorted by depth
    ///   - all depths in cell in one row are unique
    pub(crate) non_iterable_variables: HashMap<String, NonEmpty<SparseCell>>,

    /// This set contains depths were invalidated at the certain moment of script execution.
    /// They are needed for careful isolation of scopes produced by iterations in fold blocks,
    /// precisely to limit access of non iterable variables defined on one depths to ones
    /// defined on another.
    pub(crate) invalidated_depths: HashSet<usize>,

    pub(crate) iterable_variables: HashMap<String, FoldState<'i>>,

    /// Count of met scopes at the particular moment of execution.
    pub(crate) current_depth: usize,
}

#[derive(Debug)]
pub(crate) struct SparseCell {
    /// Scope depth where the value was set.
    pub(crate) depth: usize,
    pub(crate) value: Option<ValueAggregate>,
}

impl SparseCell {
    pub(crate) fn from_value(depth: usize, value: ValueAggregate) -> Self {
        Self {
            depth,
            value: Some(value),
        }
    }

    pub(crate) fn from_met_new(depth: usize) -> Self {
        Self { depth, value: None }
    }
}

impl<'i> Scalars<'i> {
    /// Returns true if there was a previous value for the provided key on the same
    /// fold block.
    pub(crate) fn set_value(&mut self, name: impl Into<String>, value: ValueAggregate) -> ExecutionResult<bool> {
        use std::collections::hash_map::Entry::{Occupied, Vacant};

        let name = name.into();
        let variable_could_be_set = self.variable_could_be_set(&name);
        match self.non_iterable_variables.entry(name) {
            Vacant(entry) => {
                let cell = SparseCell::from_value(self.current_depth, value);
                let cells = NonEmpty::new(cell);
                entry.insert(cells);

                Ok(false)
            }
            Occupied(entry) => {
                if !variable_could_be_set {
                    return Err(UncatchableError::ShadowingIsNotAllowed(entry.key().clone()).into());
                }

                let values = entry.into_mut();
                let last_cell = values.last_mut();
                if last_cell.depth == self.current_depth {
                    // just rewrite a value if fold level is the same
                    last_cell.value = Some(value);
                    Ok(true)
                } else {
                    let new_cell = SparseCell::from_value(self.current_depth, value);
                    values.push(new_cell);
                    Ok(false)
                }
            }
        }
    }

    pub(crate) fn set_iterable_value(
        &mut self,
        name: impl Into<String>,
        fold_state: FoldState<'i>,
    ) -> ExecutionResult<()> {
        use std::collections::hash_map::Entry::{Occupied, Vacant};

        match self.iterable_variables.entry(name.into()) {
            Vacant(entry) => {
                entry.insert(fold_state);
                Ok(())
            }
            Occupied(entry) => Err(UncatchableError::MultipleIterableValues(entry.key().clone()).into()),
        }
    }

    pub(crate) fn remove_iterable_value(&mut self, name: &str) {
        self.iterable_variables.remove(name);
    }

    pub(crate) fn get_non_iterable_value(&'i self, name: &str) -> ExecutionResult<Option<&'i ValueAggregate>> {
        self.non_iterable_variables
            .get(name)
            .and_then(|values| {
                let last_cell = values.last();
                let value_not_invalidated = !self.invalidated_depths.contains(&last_cell.depth);

                if value_not_invalidated {
                    Some(last_cell.value.as_ref())
                } else {
                    None
                }
            })
            .ok_or_else(|| ExecutionError::Catchable(Rc::new(CatchableError::VariableNotFound(name.to_string()))))
    }

    pub(crate) fn get_iterable_mut(&mut self, name: &str) -> ExecutionResult<&mut FoldState<'i>> {
        self.iterable_variables
            .get_mut(name)
            .ok_or_else(|| UncatchableError::FoldStateNotFound(name.to_string()).into())
    }

    pub(crate) fn get_value(&'i self, name: &str) -> ExecutionResult<ScalarRef<'i>> {
        let value = self.get_non_iterable_value(name);
        let iterable_value = self.iterable_variables.get(name);

        match (value, iterable_value) {
            (Err(_), None) => Err(CatchableError::VariableNotFound(name.to_string()).into()),
            (Ok(None), _) => Err(CatchableError::VariableWasNotInitializedAfterNew(name.to_string()).into()),
            (Ok(Some(value)), None) => Ok(ScalarRef::Value(value)),
            (Err(_), Some(iterable_value)) => Ok(ScalarRef::IterableValue(iterable_value)),
            (Ok(_), Some(_)) => unreachable!("this is checked on the parsing stage"),
        }
    }

    pub(crate) fn meet_fold_start(&mut self) {
        self.current_depth += 1;
    }

    // meet next before recursion
    pub(crate) fn meet_next_before(&mut self) {
        self.invalidated_depths.insert(self.current_depth);
        self.current_depth += 1;
    }

    // meet next after recursion
    pub(crate) fn meet_next_after(&mut self) {
        self.current_depth -= 1;
        self.invalidated_depths.remove(&self.current_depth);
        self.cleanup_obsolete_values();
    }

    pub(crate) fn meet_fold_end(&mut self) {
        self.current_depth -= 1;
        self.cleanup_obsolete_values();
    }

    pub(crate) fn meet_new_start(&mut self, scalar_name: &str) {
        use std::collections::hash_map::Entry::{Occupied, Vacant};

        let new_cell = SparseCell::from_met_new(self.current_depth);
        match self.non_iterable_variables.entry(scalar_name.to_string()) {
            Vacant(entry) => {
                let ne_vec = NonEmpty::new(new_cell);
                entry.insert(ne_vec);
            }
            Occupied(entry) => {
                let entry = entry.into_mut();
                entry.push(new_cell);
            }
        }
    }

    pub(crate) fn meet_new_end(&mut self, scalar_name: &str) -> ExecutionResult<()> {
        let current_depth = self.current_depth;
        let should_remove_values = self
            .non_iterable_variables
            .get_mut(scalar_name)
            .and_then(|values| {
                // carefully check that we're popping up an appropriate value,
                // returning None means an error here
                match values.pop() {
                    Some(value) if value.depth == current_depth => Some(false),
                    Some(_) => None,
                    // None means that the value was last in a row
                    None if values.last().depth == current_depth => Some(true),
                    None => None,
                }
            })
            .ok_or_else(|| UncatchableError::ScalarsStateCorrupted {
                scalar_name: scalar_name.to_string(),
                depth: self.current_depth,
            })
            .map_err(Into::<ExecutionError>::into)?;

        if should_remove_values {
            self.non_iterable_variables.remove(scalar_name);
        }
        Ok(())
    }

    pub(crate) fn variable_could_be_set(&self, variable_name: &str) -> bool {
        if self.shadowing_allowed() {
            return true;
        }

        match self.non_iterable_variables.get(variable_name) {
            Some(values) => values.last().value.is_none(),
            None => false,
        }
    }

    pub(crate) fn shadowing_allowed(&self) -> bool {
        // shadowing is allowed only inside a fold block, 0 here means that execution flow
        // is in a global scope
        self.current_depth != 0
    }

    fn cleanup_obsolete_values(&mut self) {
        // TODO: it takes O(N) where N is a count of all scalars, but it could be optimized
        // by maintaining array of value indices that should be removed on each depth level
        let mut values_to_delete = Vec::new();
        for (name, values) in self.non_iterable_variables.iter_mut() {
            let value_depth = values.last().depth;
            if !is_global_value(value_depth) && is_value_obsolete(value_depth, self.current_depth) {
                // it can't be empty, so it returns None if it contains 1 element
                if values.pop().is_none() {
                    // TODO: optimize this cloning in next PR
                    values_to_delete.push(name.to_string());
                }
            }
        }

        for value_name in values_to_delete {
            self.non_iterable_variables.remove(&value_name);
        }
    }
}

fn is_global_value(current_scope_depth: usize) -> bool {
    current_scope_depth == 0
}

fn is_value_obsolete(value_depth: usize, current_scope_depth: usize) -> bool {
    value_depth > current_scope_depth
}

use std::fmt;

impl<'i> fmt::Display for Scalars<'i> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "fold_block_id: {}", self.current_depth)?;

        for (name, _) in self.non_iterable_variables.iter() {
            let value = self.get_non_iterable_value(name);
            if let Ok(Some(last_value)) = value {
                writeln!(f, "{} => {}", name, last_value.result)?;
            }
        }

        for (name, _) in self.iterable_variables.iter() {
            // it's impossible to print an iterable value for now
            writeln!(f, "{} => iterable", name)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use polyplets::SecurityTetraplet;

    use serde_json::json;

    use std::num::NonZeroUsize;
    use std::rc::Rc;

    #[test]
    fn test_local_cleanup() {
        let mut scalars = Scalars::default();

        let tetraplet = SecurityTetraplet::default();
        let rc_tetraplet = Rc::new(tetraplet);
        let value = json!(1u64);
        let rc_value = Rc::new(value);
        let value_aggregate = ValueAggregate::new(rc_value, rc_tetraplet, 1);
        let value_1_name = "name_1";
        scalars.set_value(value_1_name, value_aggregate.clone()).unwrap();

        let value_2_name = "name_2";
        scalars.meet_fold_start();
        scalars.set_value(value_2_name, value_aggregate.clone()).unwrap();
        scalars.meet_fold_start();
        scalars.set_value(value_2_name, value_aggregate.clone()).unwrap();

        let expected_values_count = scalars.non_iterable_variables.get(value_2_name).unwrap().len();
        assert_eq!(expected_values_count, NonZeroUsize::new(2).unwrap());

        scalars.meet_fold_end();
        let expected_values_count = scalars.non_iterable_variables.get(value_2_name).unwrap().len();
        assert_eq!(expected_values_count, NonZeroUsize::new(1).unwrap());

        scalars.meet_fold_end();
        assert!(scalars.non_iterable_variables.get(value_2_name).is_none());

        let expected_values_count = scalars.non_iterable_variables.get(value_1_name).unwrap().len();
        assert_eq!(expected_values_count, NonZeroUsize::new(1).unwrap());
    }
}
