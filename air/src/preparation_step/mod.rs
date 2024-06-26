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

mod errors;
mod interpreter_versions;
mod preparation;
mod sizes_limits_check;

pub use errors::PreparationError;
pub use interpreter_versions::interpreter_version;
pub use interpreter_versions::min_supported_version;

pub(crate) use preparation::check_version_compatibility;
pub(crate) use preparation::parse_data;
pub(crate) use preparation::prepare;
pub(crate) use preparation::ParsedDataPair;
pub(crate) use preparation::PreparationDescriptor;
pub(crate) use sizes_limits_check::check_against_size_limits;
