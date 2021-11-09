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

use crate::ast::FunctionPart;
use crate::ast::PeerPart;
use crate::ast::RawTriplet;

pub(crate) fn try_to_raw_triplet<'i>(
    peer: PeerPart<'i>,
    f: FunctionPart<'i>,
) -> Option<RawTriplet<'i>> {
    use FunctionPart::*;
    use PeerPart::*;

    let (peer_pk, service_id, function_name) = match (peer, f) {
        (
            PeerPkWithServiceId(peer_pk, _peer_service_id),
            ServiceIdWithFuncName(service_id, func_name),
        ) => (peer_pk, service_id, func_name),
        (PeerPkWithServiceId(peer_pk, peer_service_id), FuncName(func_name)) => {
            (peer_pk, peer_service_id, func_name)
        }
        (PeerPk(peer_pk), ServiceIdWithFuncName(service_id, func_name)) => {
            (peer_pk, service_id, func_name)
        }
        (PeerPk(_), FuncName(_)) => return None,
    };

    let raw_triplet = RawTriplet {
        peer_pk,
        service_id,
        function_name,
    };

    Some(raw_triplet)
}
