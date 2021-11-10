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
use super::ExecutionError;
use super::ExecutionResult;
use crate::exec_err;
use crate::JValue;

use air_parser::ast::CallInstrValue;
use air_parser::ast::Triplet;
use polyplets::ResolvedTriplet;

/// Resolve variables, literals, etc in the `Triplet`, and build a `ResolvedTriplet`.
pub(crate) fn resolve<'i>(triplet: &Triplet<'i>, ctx: &ExecutionCtx<'i>) -> ExecutionResult<ResolvedTriplet> {
    let Triplet {
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
fn resolve_to_string<'i>(value: &CallInstrValue<'i>, ctx: &ExecutionCtx<'i>) -> ExecutionResult<String> {
    use crate::execution_step::utils::resolve_ast_variable_wl;

    let resolved = match value {
        CallInstrValue::InitPeerId => ctx.init_peer_id.clone(),
        CallInstrValue::Literal(value) => value.to_string(),
        CallInstrValue::Variable(variable) => {
            let (resolved, _) = resolve_ast_variable_wl(variable, ctx)?;
            jvalue_to_string(resolved)?
        }
    };

    Ok(resolved)
}

fn jvalue_to_string(jvalue: JValue) -> ExecutionResult<String> {
    use ExecutionError::IncompatibleJValueType;

    match jvalue {
        JValue::String(s) => Ok(s),
        _ => exec_err!(IncompatibleJValueType(jvalue, "string")),
    }
}
