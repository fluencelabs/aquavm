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

use crate::execution_step::LastErrorSettable;
use crate::execution_step::RSecurityTetraplet;
use crate::JValue;
use crate::ToErrorCode;

use serde::Deserialize;
use serde::Serialize;

use std::rc::Rc;

/// This struct is intended to track the last arisen error.
/// LastError is essentially a scalar value with support of lambda expressions.
/// The only differences from a scalar are
///  - it's accessed by %last_error% literal
///  - if it's unset before the usage, JValue::Null will be used without join behaviour
///  - it's a global scalar, meaning that fold and new scopes doesn't apply for it
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastError {
    /// Error object that represents the last occurred error.
    pub error: Rc<JValue>,

    /// Tetraplet that identify host where the error occurred.
    pub tetraplet: Option<RSecurityTetraplet>,
}

pub(crate) struct LastErrorDescriptor {
    last_error: LastError,

    /// True, if last error could be set. This flag is used to distinguish
    /// whether an error is being bubbled up from the bottom or just encountered.
    /// This allows to write a simple code to handle bubbling error up.
    error_could_be_set: bool,
}

impl<'s> LastErrorDescriptor {
    pub(crate) fn new() -> Self {
        let last_error = LastError {
            error: Rc::new(JValue::Null),
            tetraplet: None,
        };

        Self {
            last_error,
            error_could_be_set: true,
        }
    }

    pub(crate) fn try_to_set_from_ingredients(
        &mut self,
        error: &(impl ToString + LastErrorSettable + ToErrorCode),
        instruction: impl Into<String>,
        peer_id: impl Into<String>,
        tetraplet: Option<RSecurityTetraplet>,
    ) -> bool {
        // this check is optimization to prevent creation of an error object in case if error
        // couldn't be set
        if !self.error_could_be_set || !error.is_settable() {
            return false;
        }
        let error_object = serde_json::json!({
            "error_code": error.to_error_code(),
            "message": error.to_string(),
            "instruction": instruction.into(),
            "peer_id": peer_id.into(),
        });

        self.set_from_error_object(Rc::new(error_object), tetraplet);
        true
    }

    pub(crate) fn set_from_error_object(&mut self, error: Rc<JValue>, tetraplet: Option<RSecurityTetraplet>) {
        let last_error = LastError { error, tetraplet };
        self.last_error = last_error;
        self.error_could_be_set = false;
    }

    pub(crate) fn last_error(&self) -> &LastError {
        &self.last_error
    }

    pub(crate) fn meet_xor(&mut self) {
        self.error_could_be_set = true;
    }
}

impl Default for LastErrorDescriptor {
    fn default() -> Self {
        Self::new()
    }
}
