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

use super::*;

pub(super) fn apply_to_arg(
    argument: &ApArgument<'_>,
    exec_ctx: &ExecutionCtx<'_>,
    trace_ctx: &TraceHandler,
    should_touch_trace: bool,
) -> ExecutionResult<ResolvedCallResult> {
    let result = match argument {
        ApArgument::ScalarVariable(scalar_name) => apply_scalar(scalar_name, exec_ctx, trace_ctx, should_touch_trace)?,
        ApArgument::VariableWithLambda(vl) => apply_json_argument(vl, exec_ctx, trace_ctx)?,
        ApArgument::LastError(error_path) => apply_last_error(error_path, exec_ctx, trace_ctx)?,
        ApArgument::Literal(value) => apply_const(value.to_string(), exec_ctx, trace_ctx),
        ApArgument::Number(value) => apply_const(value, exec_ctx, trace_ctx),
        ApArgument::Boolean(value) => apply_const(*value, exec_ctx, trace_ctx),
        ApArgument::EmptyArray => apply_const(serde_json::json!([]), exec_ctx, trace_ctx),
    };

    Ok(result)
}

fn apply_scalar(
    scalar_name: &str,
    exec_ctx: &ExecutionCtx<'_>,
    trace_ctx: &TraceHandler,
    should_touch_trace: bool,
) -> ExecutionResult<ResolvedCallResult> {
    use crate::execution_step::ExecutionError::VariableNotFound;
    use crate::execution_step::Scalar;

    let scalar = exec_ctx.scalars.get(scalar_name)?;

    let mut result = match scalar {
        Scalar::JValueRef(result) => result.clone(),
        Scalar::JValueFoldCursor(iterator) => {
            let result = iterator.iterable.peek().expect(
                "peek always return elements inside fold,\
            this guaranteed by implementation of next and avoiding empty folds",
            );
            result.into_resolved_result()
        }
    };

    if should_touch_trace {
        result.trace_pos = trace_ctx.trace_pos();
    }

    Ok(result)
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
    vl: &VariableWithLambda<'_>,
    exec_ctx: &ExecutionCtx<'_>,
    trace_ctx: &TraceHandler,
) -> ExecutionResult<ResolvedCallResult> {
    let variable = Variable::from_ast(&vl.variable);
    let (jvalue, mut tetraplets) = apply_lambda(variable, &vl.lambda, exec_ctx)?;

    let tetraplet = tetraplets
        .pop()
        .unwrap_or_else(|| Rc::new(RefCell::new(SecurityTetraplet::default())));
    let result = ResolvedCallResult::new(Rc::new(jvalue), tetraplet, trace_ctx.trace_pos());

    Ok(result)
}
