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

use air_interpreter_data::verification;
use air_interpreter_data::InterpreterData;
use air_interpreter_signatures::FullSignatureStore;

// TODO replace with VerificationError
use crate::PreparationError;

#[tracing::instrument(skip_all)]
pub(crate) fn verify(
    prev_data: &InterpreterData,
    current_data: &InterpreterData,
) -> Result<FullSignatureStore, PreparationError> {
    current_data.cid_info.verify()?;

    let prev_data_verifier = verification::DataVerifier::new(prev_data);
    let current_data_verifier = verification::DataVerifier::new(current_data);
    // prev_data is always correct, check only current_data
    current_data_verifier.verify()?;

    let signature_store = prev_data_verifier.merge(current_data_verifier)?;
    Ok(signature_store)
}
