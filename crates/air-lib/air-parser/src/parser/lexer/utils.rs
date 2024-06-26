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

pub(super) fn is_air_alphanumeric(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_' || ch == '-'
}

pub(super) fn is_lens_allowed_char(ch: char) -> bool {
    // good old switch faster here than hash set
    match ch {
        '$' => true,
        '@' => true,
        '[' => true,
        ']' => true,
        '(' => true,
        ')' => true,
        ':' => true,
        '?' => true,
        '.' => true,
        '*' => true,
        ',' => true,
        '"' => true,
        '\'' => true,
        ch => is_air_alphanumeric(ch),
    }
}
