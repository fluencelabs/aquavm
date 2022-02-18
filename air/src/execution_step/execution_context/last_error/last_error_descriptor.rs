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

use super::last_error_definition::error_from_raw_fields;
use super::LastError;
use crate::execution_step::LastErrorAffectable;
use crate::execution_step::RcSecurityTetraplet;
use crate::JValue;
use crate::ToErrorCode;

use std::rc::Rc;

pub(crate) struct LastErrorDescriptor {
    last_error: LastError,

    /// True, if last error could be set. This flag is used to distinguish
    /// whether an error is being bubbled up from the bottom or just encountered.
    /// This allows to write a simple code to handle bubbling error up.
    error_can_be_set: bool,
}

impl<'s> LastErrorDescriptor {
    pub(crate) fn try_to_set_from_error(
        &mut self,
        error: &(impl LastErrorAffectable + ToErrorCode + ToString),
        instruction: &str,
        peer_id: &str,
        tetraplet: Option<RcSecurityTetraplet>,
    ) -> bool {
        // this check is optimization to prevent creation of an error object in case if error
        // couldn't be set
        if !self.error_can_be_set || !error.affects_last_error() {
            return false;
        }

        self.set_from_ingredients(
            error.to_error_code(),
            &error.to_string(),
            instruction,
            peer_id,
            tetraplet,
        )
    }

    pub(crate) fn set_from_ingredients(
        &mut self,
        error_code: i64,
        error_message: &str,
        instruction: &str,
        peer_id: &str,
        tetraplet: Option<RcSecurityTetraplet>,
    ) -> bool {
        let error_object = error_from_raw_fields(error_code, error_message, instruction, peer_id);
        self.set_from_error_object(Rc::new(error_object), tetraplet);
        true
    }

    pub(crate) fn set_from_error_object(&mut self, error: Rc<JValue>, tetraplet: Option<RcSecurityTetraplet>) {
        self.last_error = LastError { error, tetraplet };
        self.error_can_be_set = false;
    }

    pub(crate) fn last_error(&self) -> &LastError {
        &self.last_error
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
        let last_error = LastError {
            error: Rc::new(JValue::Null),
            tetraplet: None,
        };

        Self {
            last_error,
            error_can_be_set: true,
        }
    }
}
