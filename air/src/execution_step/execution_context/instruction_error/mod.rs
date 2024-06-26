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
