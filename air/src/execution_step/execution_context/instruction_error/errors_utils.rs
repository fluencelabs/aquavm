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

use air_interpreter_data::Provenance;

use super::instruction_error_definition::error_from_raw_fields;
use super::instruction_error_definition::error_from_raw_fields_w_peerid;
use super::InstructionError;
use crate::execution_step::ErrorAffectable;
use crate::execution_step::RcSecurityTetraplet;
use crate::JValue;
use crate::ToErrorCode;

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
    get_instruction_error_from_error_object(error_object, tetraplet, provenance)
}

pub(crate) fn get_instruction_error_from_error_object(
    error: JValue,
    tetraplet: Option<RcSecurityTetraplet>,
    provenance: Provenance,
) -> InstructionError {
    let orig_catchable = None;
    InstructionError {
        error,
        tetraplet,
        provenance,
        orig_catchable,
    }
}
