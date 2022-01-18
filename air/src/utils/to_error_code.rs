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
