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

use crate::execution_step::CatchableError;
use crate::execution_step::ExecutionError;
use crate::execution_step::ExecutionResult;
use crate::execution_step::UncatchableError;

use non_empty_vec::NonEmpty;

use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

/// Depth of a global scope.
const GLOBAL_DEPTH: usize = 0;

pub(crate) struct ValuesSparseMatrix<T> {
    cells: HashMap<String, NonEmpty<SparseCell<T>>>,

    /// This set contains depths were invalidated at the certain moment of script execution.
    /// They are needed for careful isolation of scopes produced by iterations in fold blocks,
    /// precisely to limit access of non iterable variables defined on one depths to ones
    /// defined on another.
    allowed_depths: HashSet<usize>,

    /// Count of met scopes at the particular moment of execution.
    current_depth: usize,
}

impl<T> ValuesSparseMatrix<T> {
    pub(super) fn new() -> Self {
        let allowed_depths = maplit::hashset! { GLOBAL_DEPTH };

        Self {
            cells: HashMap::new(),
            allowed_depths,
            current_depth: GLOBAL_DEPTH,
        }
    }

    pub(super) fn set_value(&mut self, name: impl Into<String>, value: T) -> ExecutionResult<bool> {
        use std::collections::hash_map::Entry::{Occupied, Vacant};

        let name = name.into();
        let variable_could_be_set = self.variable_could_be_set(&name);
        match self.cells.entry(name) {
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

    pub(super) fn get_value(&self, name: &str) -> ExecutionResult<Option<&T>> {
        self.cells
            .get(name)
            .and_then(|values| {
                let last_cell = values.last();
                let depth_allowed = self.allowed_depths.contains(&last_cell.depth);

                if depth_allowed {
                    Some(last_cell.value.as_ref())
                } else {
                    None
                }
            })
            .ok_or_else(|| ExecutionError::Catchable(Rc::new(CatchableError::VariableNotFound(name.to_string()))))
    }

    pub(super) fn meet_fold_start(&mut self) {
        self.current_depth += 1;
        self.allowed_depths.insert(self.current_depth);
    }

    // meet next before recursion
    pub(super) fn meet_next_before(&mut self) {
        self.allowed_depths.remove(&self.current_depth);
        self.current_depth += 1;
        self.allowed_depths.insert(self.current_depth);
    }

    // meet next after recursion
    pub(super) fn meet_next_after(&mut self) {
        self.allowed_depths.remove(&self.current_depth);
        self.current_depth -= 1;
        self.allowed_depths.insert(self.current_depth);

        self.cleanup_obsolete_values();
    }

    pub(super) fn meet_fold_end(&mut self) {
        self.allowed_depths.remove(&self.current_depth);
        self.current_depth -= 1;
        self.cleanup_obsolete_values();
    }

    pub(super) fn meet_new_start(&mut self, scalar_name: String) {
        use std::collections::hash_map::Entry::{Occupied, Vacant};

        let new_cell = SparseCell::from_met_new(self.current_depth);
        match self.cells.entry(scalar_name) {
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

    pub(super) fn meet_new_end(&mut self, scalar_name: &str) -> ExecutionResult<()> {
        let current_depth = self.current_depth;
        let should_remove_values = self
            .cells
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
            self.cells.remove(scalar_name);
        }
        Ok(())
    }

    pub(super) fn variable_could_be_set(&self, variable_name: &str) -> bool {
        if self.shadowing_allowed() {
            return true;
        }

        match self.cells.get(variable_name) {
            Some(values) => values.last().value.is_none(),
            None => false,
        }
    }

    pub(super) fn shadowing_allowed(&self) -> bool {
        // shadowing is allowed only inside a fold block, 0 here means that execution flow
        // is in a global scope
        self.current_depth != 0
    }

    fn cleanup_obsolete_values(&mut self) {
        // TODO: it takes O(N) where N is a count of all scalars, but it could be optimized
        // by maintaining array of value indices that should be removed on each depth level
        let mut values_to_delete = Vec::new();
        for (name, values) in self.cells.iter_mut() {
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
            self.cells.remove(&value_name);
        }
    }
}

impl<T> Default for ValuesSparseMatrix<T> {
    fn default() -> Self {
        Self::new()
    }
}

fn is_global_value(value_depth: usize) -> bool {
    value_depth == GLOBAL_DEPTH
}

fn is_value_obsolete(value_depth: usize, current_scope_depth: usize) -> bool {
    value_depth > current_scope_depth
}

#[derive(Debug)]
pub(crate) struct SparseCell<T> {
    /// Scope depth where the value was set.
    pub(crate) depth: usize,
    pub(crate) value: Option<T>,
}

impl<T> SparseCell<T> {
    pub(crate) fn from_value(depth: usize, value: T) -> Self {
        Self {
            depth,
            value: Some(value),
        }
    }

    pub(crate) fn from_met_new(depth: usize) -> Self {
        Self { depth, value: None }
    }
}

use std::fmt;

impl<T> fmt::Display for SparseCell<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Some(value) => write!(f, "{value}"),
            None => write!(f, "none"),
        }
    }
}

impl<T> fmt::Display for ValuesSparseMatrix<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "current_depth: {}", self.current_depth)?;

        for (name, values) in self.cells.iter() {
            write!(f, "{name}: ")?;

            for value in values.iter() {
                write!(f, "{value:?} ")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::execution_step::{LiteralAggregate, ValueAggregate};

    use serde_json::json;

    use std::num::NonZeroUsize;
    use std::rc::Rc;

    #[test]
    fn test_local_cleanup() {
        let mut scalars = ValuesSparseMatrix::new();

        let value = json!(1u64);
        let rc_value = Rc::new(value);
        let value_aggregate = ValueAggregate::from_literal_result(LiteralAggregate::new(rc_value, "".into(), 1.into()));
        let value_1_name = "name_1";
        scalars.set_value(value_1_name, value_aggregate.clone()).unwrap();

        let value_2_name = "name_2";
        scalars.meet_fold_start();
        scalars.set_value(value_2_name, value_aggregate.clone()).unwrap();
        scalars.meet_fold_start();
        scalars.set_value(value_2_name, value_aggregate).unwrap();

        let expected_values_count = scalars.cells.get(value_2_name).unwrap().len();
        assert_eq!(expected_values_count, NonZeroUsize::new(2).unwrap());

        scalars.meet_fold_end();
        let expected_values_count = scalars.cells.get(value_2_name).unwrap().len();
        assert_eq!(expected_values_count, NonZeroUsize::new(1).unwrap());

        scalars.meet_fold_end();
        assert!(scalars.cells.get(value_2_name).is_none());

        let expected_values_count = scalars.cells.get(value_1_name).unwrap().len();
        assert_eq!(expected_values_count, NonZeroUsize::new(1).unwrap());
    }
}
