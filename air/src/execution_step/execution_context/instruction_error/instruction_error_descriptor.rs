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

use air_interpreter_data::Provenance;

use super::instruction_error_definition::error_from_raw_fields;
use super::instruction_error_definition::error_from_raw_fields_no_peerid;
use super::no_error;
use super::InstructionError;
use crate::execution_step::InstructionErrorsEffector;
use crate::execution_step::RcSecurityTetraplet;
use crate::JValue;
use crate::ToErrorCode;

use std::rc::Rc;

pub(crate) struct InstructionErrorDescriptor {
    error: InstructionError,

    /// True, if last error could be set. This flag is used to distinguish
    /// whether an error is being bubbled up from the bottom or just encountered.
    /// This allows to write a simple code to handle bubbling error up.
    error_can_be_set: bool,
}

impl InstructionErrorDescriptor {
    pub(crate) fn try_to_set_last_error_from_exec_error(
        &mut self,
        error: &(impl InstructionErrorsEffector + ToErrorCode + ToString),
        instruction: &str,
        peer_id_option: Option<&str>,
        tetraplet: Option<RcSecurityTetraplet>,
    ) {
        // this check is optimization to prevent creation of an error object in case if error
        // couldn't be set
        if !self.error_can_be_set || !error.affects_last_error() {
            return;
        }

        self.set_from_error(error, instruction, peer_id_option, tetraplet)
    }

    pub(crate) fn try_to_set_error_from_exec_error(
        &mut self,
        error: &(impl InstructionErrorsEffector + ToErrorCode + ToString),
        instruction: &str,
        peer_id_option: Option<&str>,
        tetraplet: Option<RcSecurityTetraplet>,
    ) {
        // this check is optimization to prevent creation of an error object in case if error
        // couldn't be set
        if !self.error_can_be_set || !error.affects_error() {
            return;
        }

        self.set_from_error(error, instruction, peer_id_option, tetraplet);
    }

    pub(crate) fn set_from_error(
        &mut self,
        error: &(impl InstructionErrorsEffector + ToErrorCode + ToString),
        instruction: &str,
        peer_id_option: Option<&str>,
        tetraplet: Option<RcSecurityTetraplet>,
    ) {
        // it is not a call result, but generated from a limited set of unjoinable errors
        let provenance = Provenance::literal();

        self.set_from_ingredients(
            error.to_error_code(),
            &error.to_string(),
            instruction,
            peer_id_option,
            tetraplet,
            provenance,
        )
    }

    pub(crate) fn set_from_ingredients(
        &mut self,
        error_code: i64,
        error_message: &str,
        instruction: &str,
        peer_id_option: Option<&str>,
        tetraplet: Option<RcSecurityTetraplet>,
        provenance: Provenance,
    ) {
        let error_object = match peer_id_option {
            Some(peer_id) => error_from_raw_fields(error_code, error_message, instruction, peer_id),
            None => error_from_raw_fields_no_peerid(error_code, error_message, instruction),
        };
        self.set_from_error_object(Rc::new(error_object), tetraplet, provenance);
    }

    pub(crate) fn set_from_error_object(
        &mut self,
        error: Rc<JValue>,
        tetraplet: Option<RcSecurityTetraplet>,
        provenance: Provenance,
    ) {
        self.error = InstructionError {
            error,
            tetraplet,
            provenance,
        };
        self.error_can_be_set = false;
    }

    pub(crate) fn error(&self) -> &InstructionError {
        &self.error
    }

    pub(crate) fn meet_xor_right_branch(&mut self) {
        self.error_can_be_set = true;
    }

    pub(crate) fn meet_par_successed_end(&mut self) {
        self.error_can_be_set = true;
    }
}

impl Default for InstructionErrorDescriptor {
    fn default() -> Self {
        let error = no_error();

        Self {
            error,
            error_can_be_set: true,
        }
    }
}
