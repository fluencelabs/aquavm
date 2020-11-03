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

use super::utils::resolve_value;

use crate::air::ExecutionCtx;
use crate::{AquamarineError, Result};

use air_parser::ast::{FunctionPart, PeerPart, Value};

/// Triplet represents a location of the executable code in the network
/// It is build from `PeerPart` and `FunctionPart` of a `Call` instruction
pub(super) struct Triplet<'a, 'i> {
    pub(super) peer_pk: &'a Value<'i>,
    pub(super) service_id: &'a Value<'i>,
    pub(super) function_name: &'a Value<'i>,
}

/// ResolvedTriplet represents same location as `Triplet`, but with all
/// variables, literals and etc resolved into final `String`
pub(super) struct ResolvedTriplet {
    pub(super) peer_pk: String,
    pub(super) service_id: String,
    pub(super) function_name: String,
}

impl<'a, 'i> Triplet<'a, 'i> {
    /// Build a `Triplet` from `Call`'s `PeerPart` and `FunctionPart`
    pub fn try_from(peer: &'a PeerPart<'i>, f: &'a FunctionPart<'i>) -> Result<Self> {
        use air_parser::ast::FunctionPart::*;
        use air_parser::ast::PeerPart::*;

        let (peer_pk, service_id, function_name) = match (peer, f) {
            (PeerPkWithServiceId(peer_pk, peer_service_id), ServiceIdWithFuncName(_service_id, func_name)) => {
                Ok((peer_pk, peer_service_id, func_name))
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
    pub fn resolve(self, ctx: &'a ExecutionCtx<'i>) -> Result<ResolvedTriplet> {
        let Triplet {
            peer_pk,
            service_id,
            function_name,
        } = self;
        let peer_pk = resolve_value(peer_pk, ctx)?.as_ref().to_string();
        let service_id = resolve_value(service_id, ctx)?.as_ref().to_string();
        let function_name = resolve_value(function_name, ctx)?.as_ref().to_string();

        Ok(ResolvedTriplet {
            peer_pk,
            service_id,
            function_name,
        })
    }
}
