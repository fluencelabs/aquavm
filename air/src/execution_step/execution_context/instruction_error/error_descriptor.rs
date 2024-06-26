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

        // returns early if there is an error to bubble up or the error is Uncatchable.
        if !self.error_can_be_set || !error.affects_error() {
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
