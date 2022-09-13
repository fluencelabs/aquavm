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

use super::utils::*;
use super::LambdaError;
use crate::execution_step::CatchableError;
use crate::execution_step::ExecutionCtx;
use crate::execution_step::ExecutionResult;
use crate::lambda_to_execution_error;
use crate::ExecutionError;
use crate::JValue;
use crate::LambdaAST;

use air_lambda_ast::Functor;
use air_lambda_parser::ValueAccessor;
use non_empty_vec::NonEmpty;

use std::borrow::Cow;
use std::rc::Rc;

pub(crate) struct StreamSelectResult<'value> {
    pub(crate) result: Cow<'value, JValue>,
    pub(crate) tetraplet_idx: Option<usize>,
}

pub(crate) fn select_by_lambda_from_stream<'value, 'i>(
    stream: impl ExactSizeIterator<Item = &'value JValue> + 'value,
    lambda: &LambdaAST<'_>,
    exec_ctx: &ExecutionCtx<'i>,
) -> ExecutionResult<StreamSelectResult<'value>> {
    match lambda {
        LambdaAST::ValuePath(value_path) => select_by_path_from_stream(stream, value_path, exec_ctx),
        LambdaAST::Functor(functor) => Ok(select_by_functor_from_stream(stream, functor)),
    }
}

pub(crate) fn select_by_lambda_from_scalar<'value, 'i>(
    value: &'value JValue,
    lambda: &LambdaAST<'_>,
    exec_ctx: &ExecutionCtx<'i>,
) -> ExecutionResult<Cow<'value, JValue>> {
    match lambda {
        LambdaAST::ValuePath(value_path) => select_by_path_from_scalar(value, value_path.iter(), exec_ctx),
        LambdaAST::Functor(functor) => select_by_functor_from_scalar(value, functor).map(Cow::Owned),
    }
}

fn select_by_path_from_stream<'value, 'i>(
    stream: impl ExactSizeIterator<Item = &'value JValue> + 'value,
    lambda: &NonEmpty<ValueAccessor<'_>>,
    exec_ctx: &ExecutionCtx<'i>,
) -> ExecutionResult<StreamSelectResult<'value>> {
    let (prefix, body) = lambda.split_first();
    let idx = match prefix {
        ValueAccessor::ArrayAccess { idx } => *idx,
        ValueAccessor::FieldAccessByName { field_name } => {
            return lambda_to_execution_error!(Err(LambdaError::FieldAccessorAppliedToStream {
                field_name: field_name.to_string(),
            }));
        }
        ValueAccessor::FieldAccessByScalar { scalar_name } => {
            let scalar = exec_ctx.scalars.get_value(scalar_name)?;
            lambda_to_execution_error!(try_scalar_ref_as_idx(scalar))?
        }
        ValueAccessor::Error => unreachable!("should not execute if parsing succeeded. QED."),
    };

    let stream_size = stream.len();
    let value = lambda_to_execution_error!(stream
        .peekable()
        .nth(idx as usize)
        .ok_or(LambdaError::StreamNotHaveEnoughValues { stream_size, idx }))?;

    let result = select_by_path_from_scalar(value, body.iter(), exec_ctx)?;
    let select_result = StreamSelectResult::from_cow(result, idx);
    Ok(select_result)
}

fn select_by_functor_from_stream<'value, 'i>(
    stream: impl ExactSizeIterator<Item = &'value JValue> + 'value,
    functor: &Functor,
) -> StreamSelectResult<'value> {
    match functor {
        Functor::Length => {
            let result = serde_json::json!(stream.len());
            StreamSelectResult::from_value(result)
        }
    }
}

fn select_by_path_from_scalar<'value, 'accessor, 'i>(
    mut value: &'value JValue,
    lambda: impl Iterator<Item = &'accessor ValueAccessor<'accessor>>,
    exec_ctx: &ExecutionCtx<'i>,
) -> ExecutionResult<Cow<'value, JValue>> {
    for accessor in lambda {
        match accessor {
            ValueAccessor::ArrayAccess { idx } => {
                value = lambda_to_execution_error!(try_jvalue_with_idx(value, *idx))?;
            }
            ValueAccessor::FieldAccessByName { field_name } => {
                value = lambda_to_execution_error!(try_jvalue_with_field_name(value, field_name))?;
            }
            ValueAccessor::FieldAccessByScalar { scalar_name } => {
                let scalar = exec_ctx.scalars.get_value(scalar_name)?;
                value = lambda_to_execution_error!(select_by_scalar(value, scalar))?;
            }
            ValueAccessor::Error => unreachable!("should not execute if parsing succeeded. QED."),
        }
    }

    Ok(Cow::Borrowed(value))
}

fn select_by_functor_from_scalar(value: &JValue, functor: &Functor) -> ExecutionResult<JValue> {
    match functor {
        Functor::Length => {
            let length = value
                .as_array()
                .ok_or_else(|| {
                    ExecutionError::Catchable(Rc::new(CatchableError::LengthFunctorAppliedToNotArray(value.clone())))
                })?
                .len();
            Ok(serde_json::json!(length))
        }
    }
}

impl<'value> StreamSelectResult<'value> {
    pub(self) fn from_cow(result: Cow<'value, JValue>, tetraplet_idx: u32) -> Self {
        Self {
            result,
            tetraplet_idx: Some(tetraplet_idx as usize),
        }
    }

    pub(self) fn from_value(result: JValue) -> Self {
        Self {
            result: Cow::Owned(result),
            tetraplet_idx: None,
        }
    }
}
