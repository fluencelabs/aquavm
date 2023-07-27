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

mod values_sparse_matrix;
use crate::execution_step::boxed_value::CanonStreamMapWithProvenance;
use crate::execution_step::boxed_value::CanonStreamWithProvenance;
use crate::execution_step::boxed_value::ScalarRef;
use crate::execution_step::errors_prelude::*;
use crate::execution_step::ExecutionResult;
use crate::execution_step::FoldState;
use crate::execution_step::ValueAggregate;
use values_sparse_matrix::ValuesSparseMatrix;

use std::collections::HashMap;

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
#[allow(dead_code)]
pub(crate) struct Scalars<'i> {
    // TODO: use Rc<String> to avoid copying
    /// Terminology used here (mainly to resolve concerns re difference between scalars and values):
    ///  - scalar is an AIR scalar, iterable and non iterable. A scalar is addressed by a name.
    ///  - value is concrete value assigned to scalar on certain depth
    ///  - scope is a variable scope where variable is visible. If we consider fold as a graph where
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
    pub(crate) non_iterable_variables: ValuesSparseMatrix<ValueAggregate>,

    pub(crate) canon_streams: ValuesSparseMatrix<CanonStreamWithProvenance>,

    pub(crate) canon_maps: ValuesSparseMatrix<CanonStreamMapWithProvenance<'i>>,

    pub(crate) iterable_variables: HashMap<String, FoldState<'i>>,
}

#[allow(dead_code)]
impl<'i> Scalars<'i> {
    pub fn new() -> Self {
        Self {
            non_iterable_variables: ValuesSparseMatrix::new(),
            canon_streams: ValuesSparseMatrix::new(),
            canon_maps: ValuesSparseMatrix::new(),
            iterable_variables: HashMap::new(),
        }
    }

    /// Returns true if there was a previous value for the provided key on the same
    /// fold block.
    pub(crate) fn set_scalar_value(&mut self, name: impl Into<String>, value: ValueAggregate) -> ExecutionResult<bool> {
        self.non_iterable_variables.set_value(name, value)
    }

    /// Returns true if there was a previous value for the provided key on the same
    /// fold block.
    pub(crate) fn set_canon_value(
        &mut self,
        name: impl Into<String>,
        value: CanonStreamWithProvenance,
    ) -> ExecutionResult<bool> {
        self.canon_streams.set_value(name, value)
    }

    /// Returns true if there was a previous value for the provided key on the same
    /// fold block.
    pub(crate) fn set_canon_map_value<'k: 'i>(
        &mut self,
        name: impl Into<String>,
        value: CanonStreamMapWithProvenance<'k>,
    ) -> ExecutionResult<bool> {
        self.canon_maps.set_value(name, value)
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

    pub(crate) fn get_non_iterable_scalar(&'i self, name: &str) -> ExecutionResult<Option<&'i ValueAggregate>> {
        self.non_iterable_variables.get_value(name)
    }

    pub(crate) fn get_iterable(&mut self, name: &str) -> ExecutionResult<&FoldState<'i>> {
        self.iterable_variables
            .get(name)
            .ok_or_else(|| UncatchableError::FoldStateNotFound(name.to_string()).into())
    }

    pub(crate) fn get_iterable_mut(&mut self, name: &str) -> ExecutionResult<&mut FoldState<'i>> {
        self.iterable_variables
            .get_mut(name)
            .ok_or_else(|| UncatchableError::FoldStateNotFound(name.to_string()).into())
    }

    pub(crate) fn get_canon_stream(&'i self, name: &str) -> ExecutionResult<&'i CanonStreamWithProvenance> {
        self.canon_streams
            .get_value(name)?
            .ok_or_else(|| CatchableError::VariableWasNotInitializedAfterNew(name.to_string()).into())
    }

    pub(crate) fn get_canon_map(&'i self, name: &str) -> ExecutionResult<&'i CanonStreamMapWithProvenance<'i>> {
        self.canon_maps
            .get_value(name)?
            .ok_or_else(|| CatchableError::VariableWasNotInitializedAfterNew(name.to_string()).into())
    }

    pub(crate) fn get_value(&'i self, name: &str) -> ExecutionResult<ScalarRef<'i>> {
        let value = self.get_non_iterable_scalar(name);
        let iterable_value_with_prov = self.iterable_variables.get(name);

        match (value, iterable_value_with_prov) {
            (Err(_), None) => Err(CatchableError::VariableNotFound(name.to_string()).into()),
            (Ok(None), _) => Err(CatchableError::VariableWasNotInitializedAfterNew(name.to_string()).into()),
            (Ok(Some(value)), None) => Ok(ScalarRef::Value(value)),
            (Err(_), Some(iterable_value)) => Ok(ScalarRef::IterableValue(iterable_value)),
            (Ok(_), Some(_)) => unreachable!("this is checked on the parsing stage"),
        }
    }

    pub(crate) fn variable_could_be_set(&self, variable_name: &str) -> bool {
        self.non_iterable_variables.variable_could_be_set(variable_name)
            || self.canon_streams.variable_could_be_set(variable_name)
    }

    pub(crate) fn meet_fold_start(&mut self) {
        self.non_iterable_variables.meet_fold_start();
        self.canon_streams.meet_fold_start();
    }

    // meet next before recursion
    pub(crate) fn meet_next_before(&mut self) {
        self.non_iterable_variables.meet_next_before();
        self.canon_streams.meet_next_before();
    }

    // meet next after recursion
    pub(crate) fn meet_next_after(&mut self) {
        self.non_iterable_variables.meet_next_after();
        self.canon_streams.meet_next_after();
    }

    pub(crate) fn meet_fold_end(&mut self) {
        self.non_iterable_variables.meet_fold_end();
        self.canon_streams.meet_fold_end();
    }

    pub(crate) fn meet_new_start_scalar(&mut self, scalar_name: String) {
        self.non_iterable_variables.meet_new_start(scalar_name);
    }

    pub(crate) fn meet_new_start_canon_stream(&mut self, canon_stream_name: String) {
        self.canon_streams.meet_new_start(canon_stream_name);
    }

    pub(crate) fn meet_new_end_scalar(&mut self, scalar_name: &str) -> ExecutionResult<()> {
        self.non_iterable_variables.meet_new_end(scalar_name)
    }

    pub(crate) fn meet_new_end_canon_stream(&mut self, canon_name: &str) -> ExecutionResult<()> {
        self.canon_streams.meet_new_end(canon_name)
    }
}

impl Default for Scalars<'_> {
    fn default() -> Self {
        Scalars::new()
    }
}

use std::fmt;

impl<'i> fmt::Display for Scalars<'i> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "scalars:\n{}", self.non_iterable_variables)?;
        writeln!(f, "canon_streams:\n{}", self.canon_streams)?;

        for (name, _) in self.iterable_variables.iter() {
            // it's impossible to print an iterable value for now
            writeln!(f, "{name} => iterable")?;
        }

        Ok(())
    }
}
