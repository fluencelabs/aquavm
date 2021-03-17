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

use super::ast::Call;
use super::ast::CallInstrArgValue;
use super::ast::CallOutputValue;
use super::ast::Fold;
use super::ast::FunctionPart;
use super::ast::Instruction;
use super::ast::IterableValue;
use super::ast::PeerPart;

use std::collections::HashSet;

#[derive(Debug, Default, Clone)]
pub struct VariableValidator<'i> {
    met_variables: HashSet<&'i str>,
    met_iterable: HashSet<&'i str>,
}

/*
   pub peer_part: PeerPart<'i>,
   pub function_part: FunctionPart<'i>,
   pub args: Rc<Vec<CallInstrArgValue<'i>>>,
*/

impl<'i> VariableValidator<'i> {
    pub(super) fn new() -> Self {
        <_>::default()
    }

    pub(super) fn check_call(&self, call: &Call) {}

    pub(super) fn check_fold(&self, fold: &Fold) {}

    pub(super) fn met_fold(&self, _fold: &'i Fold) {}

    pub(super) fn met_variable(&mut self, str: &'i str) {
        self.met_variables.insert(str);
    }

    pub(super) fn met_iterable(&mut self, str: &'i str) {
        self.met_iterable.insert(str);
    }

    fn met_call_output(&mut self, call_output: &'i CallOutputValue) {
        let variable_name = match call_output {
            CallOutputValue::Scalar(scalar) => scalar,
            CallOutputValue::Accumulator(acc) => acc,
            CallOutputValue::None => {
                return;
            }
        };

        self.met_variables.insert(variable_name);
    }

    fn met_iterable_(&mut self, iterable: &'i IterableValue) {
        let variable_name = match iterable {
            IterableValue::Variable(variable) => variable,
            IterableValue::JsonPath { variable, .. } => variable,
        };

        self.met_iterable.insert(variable_name);
    }
}
