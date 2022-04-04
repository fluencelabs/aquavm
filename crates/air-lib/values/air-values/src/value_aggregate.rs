/*
 * Copyright 2022 Fluence Labs Limited
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

use super::RcSecurityTetraplet;
use air_value::BoxedValue;

use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ValueAggregate {
    pub result: Rc<dyn BoxedValue>,
    pub tetraplet: RcSecurityTetraplet,
    pub trace_pos: usize,
}

impl ValueAggregate {
    pub(crate) fn new(result: Rc<dyn BoxedValue>, tetraplet: RcSecurityTetraplet, trace_pos: usize) -> Self {
        Self {
            result,
            tetraplet,
            trace_pos,
        }
    }
}

use std::fmt;
use std::fmt::Formatter;

impl fmt::Debug for ValueAggregate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
