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

#[macro_export]
macro_rules! make_user_error(
    ($error_type:ident, $start_pos: ident, $token:expr, $end_pos: ident) => { {
        let error = $crate::parser::ParserError::$error_type($crate::parser::Span::new($start_pos, $end_pos));
        let error = lalrpop_util::ParseError::User { error };

        let dropped_tokens = vec![($start_pos, $token, $end_pos)];

        ErrorRecovery {
            error,
            dropped_tokens,
        }
    }}
);
