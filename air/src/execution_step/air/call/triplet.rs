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

use super::ExecutionCtx;
use super::ExecutionResult;
use crate::execution_step::CatchableError;
use crate::JValue;

use air_parser::ast;
use polyplets::ResolvedTriplet;

/// Resolve variables, literals, etc in the `Triplet`, and build a `ResolvedTriplet`.
pub(crate) fn resolve<'i>(triplet: &ast::Triplet<'i>, ctx: &ExecutionCtx<'i>) -> ExecutionResult<ResolvedTriplet> {
    let ast::Triplet {
        peer_pk,
        service_id,
        function_name,
    } = triplet;

    let peer_pk = resolve_to_string(peer_pk, ctx)?;
    let service_id = resolve_to_string(service_id, ctx)?;
    let function_name = resolve_to_string(function_name, ctx)?;

    Ok(ResolvedTriplet {
        peer_pk,
        service_id,
        function_name,
    })
}

/// Resolve value to string by either resolving variable from `ExecutionCtx`, taking literal value, or etc.
// TODO: return Rc<String> to avoid excess cloning
// TODO: move this function into resolve in boxed value PR
pub(crate) fn resolve_to_string<'i>(
    value: &ast::CallInstrValue<'i>,
    ctx: &ExecutionCtx<'i>,
) -> ExecutionResult<String> {
    use crate::execution_step::resolver::resolve_ast_variable_wl;
    use ast::CallInstrValue::*;

    let resolved = match value {
        InitPeerId => ctx.run_parameters.init_peer_id.to_string(),
        Literal(value) => value.to_string(),
        Variable(variable) => {
            let (resolved, _) = resolve_ast_variable_wl(variable, ctx)?;
            try_jvalue_to_string(resolved, variable)?
        }
    };

    Ok(resolved)
}

fn try_jvalue_to_string(jvalue: JValue, variable: &ast::VariableWithLambda<'_>) -> ExecutionResult<String> {
    match jvalue {
        JValue::String(s) => Ok(s),
        _ => Err(CatchableError::IncompatibleJValueType {
            variable_name: variable.name().to_string(),
            actual_value: jvalue,
            expected_value_type: "string",
        }
        .into()),
    }
}
