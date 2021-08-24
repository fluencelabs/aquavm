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

use super::*;

impl<'i> Ap<'i> {
    pub fn new(argument: ApArgument<'i>, result: AstVariable<'i>) -> Self {
        Self { argument, result }
    }
}

impl<'i> JsonPath<'i> {
    pub fn new(variable: AstVariable<'i>, path: &'i str, should_flatten: bool) -> Self {
        Self {
            variable,
            path,
            should_flatten,
        }
    }
}
