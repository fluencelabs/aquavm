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
