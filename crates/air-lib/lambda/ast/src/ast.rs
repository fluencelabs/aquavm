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

mod traits;

use non_empty_vec::NonEmpty;
use serde::Deserialize;
use serde::Serialize;

pub type LambdaAST<'input> = NonEmpty<ValueAccessor<'input>>;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum ValueAccessor<'input> {
    // (.)?[$idx]
    ArrayAccess { idx: u32 },

    // .field
    FieldAccessByName { field_name: &'input str },

    // (.)?[field]
    FieldAccessByScalar { scalar_name: &'input str },

    // needed to allow parser catch all errors from a lambda expression without stopping
    // on the very first one. Although, this variant is guaranteed not to be present in a lambda.
    Error,
}
