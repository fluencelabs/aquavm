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

pub(crate) trait ToErrorCode {
    fn to_error_code(&self) -> i64;
}

/*
use concat_idents::concat_idents;

#[macro_export]
macro_rules! generate_to_error_code {
    ($error_type:ident, $start_id: ident) => {
        const PREPARATION_ERRORS_START_ID: u32 = $start_id;

        let mut errors = PreparationErrorDiscriminants::iter();
        let actual_error_type = PreparationErrorDiscriminants::from(self);

        // unwrap is safe here because errors are guaranteed to contain all errors variants
        let enum_variant_position = errors.position(|et| et == actual_error_type).unwrap() as i64;
        PREPARATION_ERRORS_START_ID + enum_variant_position
    }
}
 */
