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

mod utils;

use super::call::call_result_setter::set_scalar_result;
use super::call::call_result_setter::set_stream_result;
use super::ExecutionCtx;
use super::ExecutionResult;
use super::TraceHandler;
use crate::execution_step::air::ResolvedCallResult;
use crate::execution_step::boxed_value::Variable;
use crate::execution_step::trace_handler::MergerApResult;
use crate::execution_step::utils::apply_json_path;
use crate::JValue;
use crate::SecurityTetraplet;
use utils::*;

use air_parser::ast::ApArgument;
use air_parser::ast::AstVariable;
use air_parser::ast::JsonPath;
use air_parser::ast::{Ap, LastErrorPath};

use std::cell::RefCell;
use std::rc::Rc;

impl<'i> super::ExecutableInstruction<'i> for Ap<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        let should_touch_trace = should_touch_trace(self);

        let merger_ap_result = if should_touch_trace {
            let merger_ap_result = trace_ctx.meet_ap_start()?;
            try_match_result_to_instr(&merger_ap_result, self)?;
            merger_ap_result
        } else {
            MergerApResult::Empty
        };

        let result = match &self.argument {
            ApArgument::ScalarVariable(scalar_name) => apply_scalar(scalar_name, exec_ctx)?,
            ApArgument::JsonPath(json_arg) => apply_json_argument(json_arg, exec_ctx, trace_ctx)?,
            ApArgument::LastError(error_path) => apply_last_error(error_path, exec_ctx, trace_ctx)?,
            ApArgument::Literal(value) => apply_const(value.to_string(), exec_ctx, trace_ctx),
            ApArgument::Number(value) => apply_const(value, exec_ctx, trace_ctx),
            ApArgument::Boolean(value) => apply_const(*value, exec_ctx, trace_ctx),
            ApArgument::EmptyArray => apply_const(serde_json::json!([]), exec_ctx, trace_ctx),
        };
        save_result(&self.result, &merger_ap_result, result, exec_ctx)?;

        if should_touch_trace {
            // if generations are empty, then this ap instruction operates only with scalars and data
            // shouldn't be updated
            let final_ap_result = to_ap_result(&merger_ap_result, self, exec_ctx);
            trace_ctx.meet_ap_end(final_ap_result);
        }

        Ok(())
    }
}

fn apply_scalar(scalar_name: &str, exec_ctx: &ExecutionCtx<'_>) -> ExecutionResult<ResolvedCallResult> {
    use super::ExecutionError;
    use crate::execution_step::Scalar;

    let scalar = exec_ctx
        .scalars
        .get(scalar_name)
        .ok_or_else(|| ExecutionError::VariableNotFound(scalar_name.to_string()))?;

    match scalar {
        Scalar::JValueRef(result) => Ok(result.clone()),
        Scalar::JValueFoldCursor(_) => crate::exec_err!(ExecutionError::ApArgumentIsIterable(scalar_name.to_string())),
    }
}

fn apply_const(value: impl Into<JValue>, exec_ctx: &ExecutionCtx<'_>, trace_ctx: &TraceHandler) -> ResolvedCallResult {
    let value = Rc::new(value.into());
    let tetraplet = SecurityTetraplet::literal_tetraplet(exec_ctx.init_peer_id.clone());
    let tetraplet = Rc::new(RefCell::new(tetraplet));

    ResolvedCallResult::new(value, tetraplet, trace_ctx.trace_pos())
}

fn apply_last_error(
    error_path: &LastErrorPath,
    exec_ctx: &ExecutionCtx<'_>,
    trace_ctx: &TraceHandler,
) -> ExecutionResult<ResolvedCallResult> {
    let (value, mut tetraplets) = crate::execution_step::utils::prepare_last_error(error_path, exec_ctx)?;
    let value = Rc::new(value);
    let tetraplet = tetraplets.remove(0);

    let result = ResolvedCallResult::new(value, tetraplet, trace_ctx.trace_pos());
    Ok(result)
}

fn apply_json_argument(
    json_arg: &JsonPath<'_>,
    exec_ctx: &ExecutionCtx<'_>,
    trace_ctx: &TraceHandler,
) -> ExecutionResult<ResolvedCallResult> {
    let variable = Variable::from_ast(&json_arg.variable);
    let (jvalue, mut tetraplets) = apply_json_path(variable, json_arg.path, json_arg.should_flatten, exec_ctx)?;

    let tetraplet = tetraplets
        .pop()
        .unwrap_or_else(|| Rc::new(RefCell::new(SecurityTetraplet::default())));
    let result = ResolvedCallResult::new(Rc::new(jvalue), tetraplet, trace_ctx.trace_pos());

    Ok(result)
}

fn save_result<'ctx>(
    ap_result_type: &AstVariable<'ctx>,
    merger_ap_result: &MergerApResult,
    result: ResolvedCallResult,
    exec_ctx: &mut ExecutionCtx<'ctx>,
) -> ExecutionResult<()> {
    match ap_result_type {
        AstVariable::Scalar(name) => set_scalar_result(result, name, exec_ctx),
        AstVariable::Stream(name) => {
            let generation = ap_result_to_generation(merger_ap_result);
            set_stream_result(result, generation, name.to_string(), exec_ctx).map(|_| ())
        }
    }
}

fn should_touch_trace(ap: &Ap<'_>) -> bool {
    match (&ap.argument, &ap.result) {
        (_, AstVariable::Stream(_)) => true,
        (ApArgument::JsonPath(json_path), _) => match &json_path.variable {
            AstVariable::Scalar(_) => false,
            AstVariable::Stream(_) => true,
        },
        _ => false,
    }
}
