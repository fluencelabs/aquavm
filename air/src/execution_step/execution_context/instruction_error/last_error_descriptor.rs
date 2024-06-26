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

use super::no_error;
use super::InstructionError;
use crate::execution_step::ErrorAffectable;
use crate::execution_step::RcSecurityTetraplet;
use crate::JValue;
use crate::ToErrorCode;

pub(crate) struct LastErrorDescriptor {
    error: InstructionError,

    /// True, if last error could be set. This flag is used to distinguish
    /// whether an error is being bubbled up from the bottom or just encountered.
    /// This allows to write a simple code to handle bubbling error up.
    error_can_be_set: bool,
}

impl LastErrorDescriptor {
    pub(crate) fn try_to_set_last_error_from_exec_error(
        &mut self,
        error: &(impl ErrorAffectable + ToErrorCode + ToString),
        instruction: &str,
        peer_id_option: Option<&str>,
        tetraplet: Option<RcSecurityTetraplet>,
    ) {
        use super::get_instruction_error_from_exec_error;

        // This check is an optimization to prevent creation of an error object in case if error
        // must not be set.
        if !self.error_can_be_set || !error.affects_last_error() {
            return;
        }

        self.error = get_instruction_error_from_exec_error(error, instruction, peer_id_option, tetraplet);
        self.error_can_be_set = false;
    }

    pub(crate) fn set_from_error_object(
        &mut self,
        error: JValue,
        tetraplet: Option<RcSecurityTetraplet>,
        provenance: Provenance,
    ) {
        use super::get_instruction_error_from_error_object;

        self.error = get_instruction_error_from_error_object(error, tetraplet, provenance);
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

impl Default for LastErrorDescriptor {
    fn default() -> Self {
        let error = no_error();

        Self {
            error,
            error_can_be_set: true,
        }
    }
}
