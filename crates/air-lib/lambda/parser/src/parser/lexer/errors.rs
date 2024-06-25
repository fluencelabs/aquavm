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

use thiserror::Error as ThisError;

use std::num::ParseIntError;

#[derive(ThisError, Debug, Clone, PartialEq, Eq)]
pub enum LexerError {
    #[error("unexpected symbol for value accessor")]
    UnexpectedSymbol(usize, usize),

    #[error("{2}")]
    ParseIntError(usize, usize, #[source] ParseIntError),
}
