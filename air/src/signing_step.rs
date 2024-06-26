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

use crate::ExecutionError;

use air_interpreter_signatures::KeyPair;
use air_interpreter_signatures::PeerCidTracker;
use air_interpreter_signatures::SignatureStore;

#[cfg(feature = "gen_signatures")]
#[tracing::instrument(skip_all)]
pub(crate) fn sign_produced_cids(
    signature_tracker: &mut PeerCidTracker,
    signature_store: &mut SignatureStore,
    salt: &str,
    keypair: &KeyPair,
) -> Result<(), ExecutionError> {
    use crate::UncatchableError;

    let signature = signature_tracker
        .gen_signature(salt, keypair)
        .map_err(UncatchableError::SigningError)?;
    let public_key = keypair.public();
    signature_store.put(public_key, signature);
    Ok(())
}

#[cfg(not(feature = "gen_signatures"))]
#[tracing::instrument(skip_all)]
pub(crate) fn sign_produced_cids(
    _signature_tracker: &mut PeerCidTracker,
    _signature_store: &mut SignatureStore,
    _salt: &str,
    _keypair: &KeyPair,
) -> Result<(), ExecutionError> {
    Ok(())
}
