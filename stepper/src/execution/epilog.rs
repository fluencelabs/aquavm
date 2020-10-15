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

use super::CALL_EVIDENCE_CTX_KEY;
use crate::call_evidence::CallEvidenceCtx;
use crate::AquaData;
use crate::AquamarineError::CallEvidenceSerializationError as CallSeError;
use crate::AquamarineError::DataSerializationError as DataSeError;
use crate::Result;

pub(super) fn make_result_data(mut data: AquaData, call_ctx: CallEvidenceCtx) -> Result<String> {
    use serde_json::{to_string, to_value};

    let serialized_call_ctx = to_value(call_ctx.new_path).map_err(CallSeError)?;
    data.insert(CALL_EVIDENCE_CTX_KEY.to_string(), serialized_call_ctx);

    let data = to_string(&data).map_err(DataSeError)?;

    Ok(data)
}
