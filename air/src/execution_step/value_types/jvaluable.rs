/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

mod canon_stream;
mod canon_stream_map;
mod cell_vec_resolved_call_result;
mod iterable_item;
mod resolved_call_result;

use super::iterable::IterableItem;
use super::ExecutionResult;
use super::ValueAggregate;
use crate::execution_step::lambda_applier::*;
use crate::execution_step::ExecutionCtx;
use crate::execution_step::RcSecurityTetraplets;
use crate::JValue;
use crate::LambdaAST;
use crate::SecurityTetraplet;

use air_interpreter_data::Provenance;

/// Represent a value that could be transform to a JValue with or without tetraplets.
pub(crate) trait JValuable {
    /// Applies lambda to the internal value, produces JValue.
    fn apply_lambda(&self, lambda: &LambdaAST<'_>, exec_ctx: &ExecutionCtx<'_>) -> ExecutionResult<JValue>;

    /// Applies lambda to the internal value, produces JValue with tetraplet.
    // TODO self should know about own provenance, but it will require
    // TODO implementing JValuable for different types than now,
    // TODO that's why current implementation passes root provenance explicitely
    fn apply_lambda_with_tetraplets(
        &self,
        lambda: &LambdaAST<'_>,
        exec_ctx: &ExecutionCtx<'_>,
        root_provenance: &Provenance,
    ) -> ExecutionResult<(JValue, SecurityTetraplet, Provenance)>;

    /// Return internal value as borrowed if it's possible, owned otherwise.
    fn as_jvalue(&self) -> JValue;

    /// Return tetraplets associating with internal value.
    fn as_tetraplets(&self) -> RcSecurityTetraplets;
}
