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

use crate::contexts::execution::AValue;
use crate::contexts::execution::ExecutionCtx;
use crate::execution::boxed_value::JValuable;
use crate::execution::ExecutionError;
use crate::execution::ExecutionResult;
use crate::JValue;
use crate::SecurityTetraplet;

use air_parser::ast::CallInstrArgValue;

/// Resolve value to called function arguments.
pub(crate) fn resolve_to_args<'i>(
    value: &CallInstrArgValue<'i>,
    ctx: &ExecutionCtx<'i>,
) -> ExecutionResult<(JValue, Vec<SecurityTetraplet>)> {
    match value {
        CallInstrArgValue::InitPeerId => prepare_consts(ctx.init_peer_id.clone(), ctx),
        CallInstrArgValue::LastError => prepare_last_error(ctx),
        CallInstrArgValue::Literal(value) => prepare_consts(value.to_string(), ctx),
        CallInstrArgValue::Boolean(value) => prepare_consts(*value, ctx),
        CallInstrArgValue::Number(value) => prepare_consts(value, ctx),
        CallInstrArgValue::Variable(variable) => prepare_variable(variable, ctx),
        CallInstrArgValue::JsonPath {
            variable,
            path,
            should_flatten,
        } => prepare_json_path(variable, path, *should_flatten, ctx),
    }
}

#[allow(clippy::unnecessary_wraps)]
fn prepare_consts(arg: impl Into<JValue>, ctx: &ExecutionCtx<'_>) -> ExecutionResult<(JValue, Vec<SecurityTetraplet>)> {
    let jvalue = arg.into();
    let tetraplet = SecurityTetraplet::literal_tetraplet(ctx.init_peer_id.clone());

    Ok((jvalue, vec![tetraplet]))
}

#[allow(clippy::unnecessary_wraps)]
fn prepare_last_error(ctx: &ExecutionCtx<'_>) -> ExecutionResult<(JValue, Vec<SecurityTetraplet>)> {
    let result = match &ctx.last_error {
        Some(error) => {
            let serialized_error = error.serialize();
            let jvalue = JValue::String(serialized_error);
            let tetraplets = error
                .tetraplet
                .clone()
                .unwrap_or_else(|| SecurityTetraplet::literal_tetraplet(ctx.init_peer_id.clone()));

            (jvalue, vec![tetraplets])
        }
        None => {
            let jvalue = JValue::String(String::new());
            let tetraplets = vec![];

            (jvalue, tetraplets)
        }
    };

    Ok(result)
}

fn prepare_variable<'i>(
    variable: &Variable<'_>,
    ctx: &ExecutionCtx<'i>,
) -> ExecutionResult<(JValue, Vec<SecurityTetraplet>)> {
    let resolved = match variable {
        Variable::Scalar(name) => resolve_to_jvaluable(name, ctx)?,
        Variable::Stream(name) => {
            // return an empty stream for not found stream
            // here it ignores the join behaviour
            if ctx.data_cache.get(*name).is_none() {
                Box::new(())
            } else {
                resolve_to_jvaluable(name, ctx)?
            }
        }
    };

    let tetraplets = resolved.as_tetraplets();
    let jvalue = resolved.into_jvalue();

    Ok((jvalue, tetraplets))
}

fn prepare_json_path<'i>(
    variable: &Variable<'_>,
    json_path: &str,
    should_flatten: bool,
    ctx: &ExecutionCtx<'i>,
) -> ExecutionResult<(JValue, Vec<SecurityTetraplet>)> {
    let name = get_variable_name(variable);

    let resolved = resolve_to_jvaluable(name, ctx)?;
    let (jvalue, tetraplets) = resolved.apply_json_path_with_tetraplets(json_path)?;

    let jvalue = if should_flatten {
        if jvalue.len() != 1 {
            let jvalue = jvalue.into_iter().cloned().collect::<Vec<_>>();
            return crate::exec_err!(ExecutionError::FlatteningError(JValue::Array(jvalue)));
        }
        jvalue[0].clone()
    } else {
        let jvalue = jvalue.into_iter().cloned().collect::<Vec<_>>();
        JValue::Array(jvalue)
    };

    Ok((jvalue, tetraplets))
}

/// Constructs jvaluable result from `ExecutionCtx::data_cache` by name.
pub(crate) fn resolve_to_jvaluable<'name, 'i, 'ctx>(
    name: &'name str,
    ctx: &'ctx ExecutionCtx<'i>,
) -> ExecutionResult<Box<dyn JValuable + 'ctx>> {
    use ExecutionError::VariableNotFound;

    let value = ctx
        .data_cache
        .get(name)
        .ok_or_else(|| VariableNotFound(name.to_string()))?;

    match value {
        AValue::JValueRef(value) => Ok(Box::new(value.clone())),
        AValue::JValueStreamRef(stream) => Ok(Box::new(stream.borrow())),
        AValue::JValueFoldCursor(fold_state) => {
            let peeked_value = fold_state.iterable.peek().unwrap();
            Ok(Box::new(peeked_value))
        }
    }
}

use air_parser::ast::Variable;

pub(crate) fn get_variable_name<'a>(variable: &'a Variable<'_>) -> &'a str {
    match variable {
        Variable::Scalar(name) => name,
        Variable::Stream(name) => name,
    }
}
