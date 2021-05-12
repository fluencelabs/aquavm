/*
 * Copyright 2020 Fluence Labs Limited
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

pub(super) fn is_air_alphanumeric(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_' || ch == '-'
}

pub(super) fn is_json_path_allowed_char(ch: char) -> bool {
    // we don't have spec for json path now, but some possible example could be found here
    // https://packagist.org/packages/softcreatr/jsonpath

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
