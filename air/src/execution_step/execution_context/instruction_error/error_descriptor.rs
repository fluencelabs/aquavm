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

use std::rc::Rc;

use super::no_error;
use super::InstructionError;
use crate::execution_step::ErrorAffectable;
use crate::execution_step::RcSecurityTetraplet;
use crate::ExecutionError;
use crate::JValue;
use crate::ToErrorCode;

pub(crate) struct ErrorDescriptor {
    error: InstructionError,
}

impl ErrorDescriptor {
    pub(crate) fn try_to_set_error_from_exec_error(
        &mut self,
        error: &(impl ErrorAffectable + ToErrorCode + ToString),
        instruction: &str,
        peer_id_option: Option<&str>,
        tetraplet: Option<RcSecurityTetraplet>,
    ) {
        use super::get_instruction_error_from_exec_error;

        if !error.affects_error() {
            return;
        }

        self.error = get_instruction_error_from_exec_error(error, instruction, peer_id_option, tetraplet);
    }

    pub(crate) fn error(&self) -> &InstructionError {
        &self.error
    }

    pub(crate) fn clear_error_object(&mut self) {
        let orig_error_object = self.error.orig_error_object.clone();
        self.error = no_error();
        self.error.orig_error_object = orig_error_object;
    }

    pub(crate) fn set_original_execution_error(&mut self, e: &ExecutionError) {
        self.error.orig_catchable = match e {
            ExecutionError::Catchable(err) => Some(err.as_ref().clone()),
            _ => None,
        };
    }

    pub(crate) fn clear_original_error_object(&mut self) {
        self.error.orig_error_object = None;
    }

    pub(crate) fn original_error_object(&self) -> &Option<Rc<JValue>> {
        &self.error.orig_error_object
    }

    pub(crate) fn set_both_error_objects(&mut self, error_object: &Rc<JValue>) {
        self.error.error = error_object.clone();
        self.error.orig_error_object = Some(error_object.clone());
    }
}

impl Default for ErrorDescriptor {
    fn default() -> Self {
        let error = no_error();

        Self { error }
    }
}
