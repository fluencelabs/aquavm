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

use super::no_error;
use super::InstructionError;
use crate::execution_step::ErrorAffectable;
use crate::execution_step::RcSecurityTetraplet;
use crate::ExecutionError;
use crate::ToErrorCode;

pub(crate) struct ErrorDescriptor {
    error: InstructionError,
    error_can_be_set: bool,
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

    pub(crate) fn clear_error_object_if_needed(&mut self) {
        if self.error_can_be_set {
            self.error = no_error();
        }
    }

    pub(crate) fn set_original_execution_error(&mut self, e: &ExecutionError) {
        self.error.orig_catchable = match e {
            ExecutionError::Catchable(err) => Some(err.as_ref().clone()),
            _ => None,
        };
    }

    pub(crate) fn enable_error_setting(&mut self) {
        self.error_can_be_set = true;
    }

    pub(crate) fn disable_error_setting(&mut self) {
        self.error_can_be_set = false;
    }

    pub(crate) fn error_setting_is_enabled(&self) -> bool {
        self.error_can_be_set
    }
}

impl Default for ErrorDescriptor {
    fn default() -> Self {
        let error = no_error();
        let error_can_be_set = true;

        Self {
            error,
            error_can_be_set,
        }
    }
}
