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

use super::ExecutionCtx;
use super::ExecutionResult;
use super::TraceHandler;
use crate::execution_step::utils::get_variable_name;
use crate::execution_step::utils::resolve_to_args;
use crate::execution_step::Generation;

use air_parser::ast::{Ap, CallInstrArgValue};

use crate::execution_step::air::ResolvedCallResult;
use std::rc::Rc;

impl<'i> super::ExecutableInstruction<'i> for Ap<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, _trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        let arg_value = CallInstrArgValue::JsonPath {
            variable: self.variable.clone(),
            path: self.path,
            should_flatten: self.should_flatten,
        };

        let (jvalue, tetraplet) = resolve_to_args(&arg_value, exec_ctx)?;

        let output = &self.output;
        let variable_name = get_variable_name(output);
        match exec_ctx.streams.get(variable_name) {
            Some(stream) => {
                let resolved_call = ResolvedCallResult::new(Rc::new(jvalue), tetraplet[0].triplet.clone(), 0);
                stream.borrow_mut().add_value(resolved_call, Generation::Last)?;
            }
            _ => unreachable!("return a error"),
        };

        Ok(())
    }
}
