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

use crate::air::ExecutionCtx;
use crate::AquamarineError;
use crate::JValue;
use crate::Result;

use air_parser::ast::{FunctionPart, InstructionValue, PeerPart};

use serde::Deserialize;
use serde::Serialize;

/// Triplet represents a location of the executable code in the network
/// It is build from `PeerPart` and `FunctionPart` of a `Call` instruction
pub(super) struct Triplet<'a, 'i> {
    pub(super) peer_pk: &'a InstructionValue<'i>,
    pub(super) service_id: &'a InstructionValue<'i>,
    pub(super) function_name: &'a InstructionValue<'i>,
}

/// ResolvedTriplet represents same location as `Triplet`, but with all
/// variables, literals and etc resolved into final `String`
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ResolvedTriplet {
    pub peer_pk: String,
    pub service_id: String,
    pub function_name: String,
}

impl<'a, 'i> Triplet<'a, 'i> {
    /// Build a `Triplet` from `Call`'s `PeerPart` and `FunctionPart`
    pub fn try_from(peer: &'a PeerPart<'i>, f: &'a FunctionPart<'i>) -> Result<Self> {
        use air_parser::ast::FunctionPart::*;
        use air_parser::ast::PeerPart::*;

        let (peer_pk, service_id, function_name) = match (peer, f) {
            (PeerPkWithServiceId(peer_pk, _peer_service_id), ServiceIdWithFuncName(service_id, func_name)) => {
                Ok((peer_pk, service_id, func_name))
            }
            (PeerPkWithServiceId(peer_pk, peer_service_id), FuncName(func_name)) => {
                Ok((peer_pk, peer_service_id, func_name))
            }
            (PeerPk(peer_pk), ServiceIdWithFuncName(service_id, func_name)) => Ok((peer_pk, service_id, func_name)),
            (PeerPk(_), FuncName(_)) => Err(AquamarineError::InstructionError(String::from(
                "call should have service id specified by peer part or function part",
            ))),
        }?;

        Ok(Self {
            peer_pk,
            service_id,
            function_name,
        })
    }

    /// Resolve variables, literals, etc in the `Triplet`, and build a `ResolvedTriplet`
    pub fn resolve(self, ctx: &ExecutionCtx<'i>) -> Result<ResolvedTriplet> {
        let Triplet {
            peer_pk,
            service_id,
            function_name,
        } = self;
        let peer_pk = resolve_to_string(peer_pk, ctx)?;
        let service_id = resolve_to_string(service_id, ctx)?;
        let function_name = resolve_to_string(function_name, ctx)?;

        Ok(ResolvedTriplet {
            peer_pk,
            service_id,
            function_name,
        })
    }
}

/// Resolve value to string by either resolving variable from `ExecutionCtx`, taking literal value, or etc
// TODO: return &str to avoid excess cloning
fn resolve_to_string<'i>(value: &InstructionValue<'i>, ctx: &ExecutionCtx<'i>) -> Result<String> {
    use crate::air::resolve::resolve_to_call_result;

    let resolved = match value {
        InstructionValue::CurrentPeerId => ctx.current_peer_id.clone(),
        InstructionValue::InitPeerId => ctx.init_peer_id.clone(),
        InstructionValue::Literal(value) => value.to_string(),
        InstructionValue::Variable(name) => {
            let resolved = resolve_to_call_result(name, ctx)?;
            let jvalue = resolved.into_jvalue();
            jvalue_to_string(jvalue)?
        }
        InstructionValue::JsonPath { variable, path } => {
            let resolved = resolve_to_call_result(variable, ctx)?;
            let resolved = resolved.apply_json_path(path)?;
            vec_to_string(resolved, path)?
        }
    };

    Ok(resolved)
}

fn jvalue_to_string(jvalue: JValue) -> Result<String> {
    use AquamarineError::IncompatibleJValueType;

    match jvalue {
        JValue::String(s) => Ok(s),
        _ => Err(IncompatibleJValueType(jvalue, "string")),
    }
}

fn vec_to_string(values: Vec<&JValue>, json_path: &str) -> Result<String> {
    if values.is_empty() {
        return Err(AquamarineError::VariableNotFound(json_path.to_string()));
    }

    if values.len() != 1 {
        return Err(AquamarineError::MultipleValuesInJsonPath(json_path.to_string()));
    }

    jvalue_to_string(values[0].clone())
}
