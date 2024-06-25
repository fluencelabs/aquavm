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

use air_interpreter_data::InterpreterData;
use air_interpreter_signatures::SignatureStore;

// TODO replace with VerificationError
use crate::PreparationError;

#[cfg(feature = "check_signatures")]
#[tracing::instrument(skip_all)]
pub(crate) fn verify(
    prev_data: &InterpreterData,
    current_data: &InterpreterData,
    salt: &str,
) -> Result<SignatureStore, PreparationError> {
    use air_interpreter_data::verification;

    current_data.cid_info.verify()?;

    let prev_data_verifier = verification::DataVerifier::new(prev_data, salt)?;
    let current_data_verifier = verification::DataVerifier::new(current_data, salt)?;
    // prev_data is always correct, check only current_data
    current_data_verifier.verify()?;

    let signature_store = prev_data_verifier.merge(current_data_verifier)?;
    Ok(signature_store)
}

#[cfg(not(feature = "check_signatures"))]
#[tracing::instrument(skip_all)]
pub(crate) fn verify(
    _prev_data: &InterpreterData,
    _current_data: &InterpreterData,
    _salt: &str,
) -> Result<SignatureStore, PreparationError> {
    Ok(<_>::default())
}
