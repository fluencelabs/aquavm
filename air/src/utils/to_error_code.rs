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

pub trait ToErrorCode {
    fn to_error_code(&self) -> i64;
}

#[macro_export]
macro_rules! generate_to_error_code {
    ($self: expr, $error_type:ident, $start_id: expr) => {
        concat_idents::concat_idents!(error_start_id = $error_type, _, START_ID {
            concat_idents::concat_idents!(error_discriminant = $error_type, Discriminants { {
                #[allow(non_upper_case_globals)]
                const error_start_id: i64 = $start_id;

                let mut errors = error_discriminant::iter();
                let actual_error_type = error_discriminant::from($self);

                // unwrap is safe here because errors are guaranteed to contain all errors variants
                let enum_variant_position = errors.position(|et| et == actual_error_type).unwrap() as i64;
                error_start_id + enum_variant_position
                }
            })
        })
    }
}
