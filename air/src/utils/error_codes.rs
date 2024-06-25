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

/// This consts are used as start ids of corresponding errors.
pub(crate) const PREPARATION_ERROR_START_ID: i64 = 1;
pub(crate) const CATCHABLE_ERRORS_START_ID: i64 = 10000;
pub(crate) const UNCATCHABLE_ERRORS_START_ID: i64 = 20000;
pub(crate) const FAREWELL_ERRORS_START_ID: i64 = 30000;
