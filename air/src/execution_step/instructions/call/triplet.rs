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

use super::ExecutionCtx;
use super::ExecutionResult;
use crate::execution_step::resolver::Resolvable;
use crate::execution_step::CatchableError;
use crate::JValue;

use air_parser::ast;
use polyplets::ResolvedTriplet;

/// Resolve variables, literals, etc in the `Triplet`, and build a `ResolvedTriplet`.
pub(crate) fn resolve<'i>(triplet: &ast::Triplet<'i>, ctx: &ExecutionCtx<'i>) -> ExecutionResult<ResolvedTriplet> {
    let ast::Triplet {
        peer_id: peer_pk,
        service_id,
        function_name,
    } = triplet;

    let peer_pk = resolve_peer_id_to_string(peer_pk, ctx)?;
    let service_id = resolve_to_string(service_id, ctx)?;
    let function_name = resolve_to_string(function_name, ctx)?;

    Ok(ResolvedTriplet {
        peer_pk,
        service_id,
        function_name,
    })
}

/// Resolve peer id to string by either resolving variable from `ExecutionCtx`, taking literal value, or etc.
// TODO: return Rc<String> to avoid excess cloning
// TODO: move this function into resolve in boxed value PR
pub(crate) fn resolve_peer_id_to_string<'i>(
    value: &ast::ResolvableToPeerIdVariable<'i>,
    exec_ctx: &ExecutionCtx<'i>,
) -> ExecutionResult<String> {
    use ast::ResolvableToPeerIdVariable::*;

    let ((jvalue, _, _), name) = match value {
        InitPeerId => return Ok(exec_ctx.run_parameters.init_peer_id.to_string()),
        Literal(value) => return Ok(value.to_string()),
        Scalar(scalar) => (scalar.resolve(exec_ctx)?, scalar.name),
        ScalarWithLambda(scalar) => (scalar.resolve(exec_ctx)?, scalar.name),
        CanonStreamWithLambda(canon_stream) => (canon_stream.resolve(exec_ctx)?, canon_stream.name),
        CanonStreamMapWithLambda(canon_stream_map) => (canon_stream_map.resolve(exec_ctx)?, canon_stream_map.name),
    };

    try_jvalue_to_string(jvalue, name)
}

/// Resolve value to string by either resolving variable from `ExecutionCtx`, taking literal value, or etc.
// TODO: return Rc<String> to avoid excess cloning
// TODO: move this function into resolve in boxed value PR
pub(crate) fn resolve_to_string<'i>(
    value: &ast::ResolvableToStringVariable<'i>,
    exec_ctx: &ExecutionCtx<'i>,
) -> ExecutionResult<String> {
    use ast::ResolvableToStringVariable::*;

    let ((jvalue, _, _), name) = match value {
        Literal(value) => return Ok(value.to_string()),
        Scalar(scalar) => (scalar.resolve(exec_ctx)?, scalar.name),
        ScalarWithLambda(scalar) => (scalar.resolve(exec_ctx)?, scalar.name),
        CanonStreamWithLambda(canon_stream) => (canon_stream.resolve(exec_ctx)?, canon_stream.name),
        CanonStreamMapWithLambda(canon_stream_map) => (canon_stream_map.resolve(exec_ctx)?, canon_stream_map.name),
    };

    try_jvalue_to_string(jvalue, name)
}

fn try_jvalue_to_string(jvalue: JValue, variable_name: impl Into<String>) -> ExecutionResult<String> {
    match jvalue {
        JValue::String(s) => Ok(s.to_string()),
        _ => Err(CatchableError::NonStringValueInTripletResolution {
            variable_name: variable_name.into(),
            actual_value: jvalue,
        }
        .into()),
    }
}
