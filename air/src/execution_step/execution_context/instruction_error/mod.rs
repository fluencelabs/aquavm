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

mod error_descriptor;
mod errors;
mod errors_utils;
mod instruction_error_definition;
mod last_error_descriptor;

pub use errors::ErrorObjectError;
pub use instruction_error_definition::no_error;
pub use instruction_error_definition::no_error_object;
pub use instruction_error_definition::InstructionError;
pub use instruction_error_definition::ERROR_CODE_FIELD_NAME;
pub use instruction_error_definition::INSTRUCTION_FIELD_NAME;
pub use instruction_error_definition::MESSAGE_FIELD_NAME;
pub use instruction_error_definition::NO_ERROR_ERROR_CODE;
pub use instruction_error_definition::NO_ERROR_MESSAGE;

pub(crate) use error_descriptor::ErrorDescriptor;
pub(super) use errors_utils::*;
pub(crate) use instruction_error_definition::check_error_object;
pub(crate) use instruction_error_definition::error_from_raw_fields_w_peerid;
pub(crate) use last_error_descriptor::LastErrorDescriptor;
