/*
 * Copyright 2023 Fluence Labs Limited
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

use air_interpreter_data::Provenance;

use super::instruction_error_definition::error_from_raw_fields;
use super::instruction_error_definition::error_from_raw_fields_w_peerid;
use super::InstructionError;
use crate::execution_step::ErrorAffectable;
use crate::execution_step::RcSecurityTetraplet;
use crate::JValue;
use crate::ToErrorCode;

use std::rc::Rc;

pub(crate) fn get_instruction_error_from_exec_error(
    error: &(impl ErrorAffectable + ToErrorCode + ToString),
    instruction: &str,
    peer_id_option: Option<&str>,
    tetraplet: Option<RcSecurityTetraplet>,
) -> InstructionError {
    // it is not a call result, but generated from a limited set of unjoinable errors
    let provenance = Provenance::literal();

    get_instruction_error_from_ingredients(
        error.to_error_code(),
        &error.to_string(),
        instruction,
        peer_id_option,
        tetraplet,
        provenance,
    )
}

pub(crate) fn get_instruction_error_from_ingredients(
    error_code: i64,
    error_message: &str,
    instruction: &str,
    peer_id_option: Option<&str>,
    tetraplet: Option<RcSecurityTetraplet>,
    provenance: Provenance,
) -> InstructionError {
    let error_object = match peer_id_option {
        Some(peer_id) => error_from_raw_fields_w_peerid(error_code, error_message, instruction, peer_id),
        None => error_from_raw_fields(error_code, error_message, instruction),
    };
    get_instruction_error_from_error_object(Rc::new(error_object), tetraplet, provenance)
}

pub(crate) fn get_instruction_error_from_error_object(
    error: Rc<JValue>,
    tetraplet: Option<RcSecurityTetraplet>,
    provenance: Provenance,
) -> InstructionError {
    InstructionError {
        error,
        tetraplet,
        provenance,
    }
}
