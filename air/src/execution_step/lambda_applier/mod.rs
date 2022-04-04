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

mod applier;
mod errors;
mod utils;

pub use errors::LambdaError;

pub(crate) type LambdaResult<T> = std::result::Result<T, LambdaError>;

use air_lambda_ast::ResolvedValueAccessor;

pub(crate) use applier::select_from_stream;

use std::rc::Rc;

#[macro_export]
macro_rules! lambda_to_execution_error {
    ($lambda_expr: expr) => {
        $lambda_expr.map_err(|lambda_error| {
            crate::execution_step::ExecutionError::Catchable(std::rc::Rc::new(
                crate::execution_step::CatchableError::LambdaApplierError(lambda_error),
            ))
        })
    };
}

use super::ExecutionResult;
use crate::execution_step::ExecutionCtx;
use air_lambda_ast::AIRLambda;
use air_lambda_ast::AIRLambdaAST;
use air_values::boxed_value::RcBoxedValue;

pub(crate) fn resolve_lambda<'ctx: 'lambda, 'lambda, 'i>(
    lambda: &'lambda AIRLambdaAST<'ctx>,
    exec_ctx: &'ctx ExecutionCtx<'i>,
) -> ExecutionResult<AIRLambda<'ctx>> {
    use air_lambda_ast::ValueAccessor;

    let resolved_lambda = lambda
        .into_iter()
        .map(|value| -> ExecutionResult<ResolvedValueAccessor<'_>> {
            match value {
                &ValueAccessor::ArrayAccess { idx } => Ok(ResolvedValueAccessor::ArrayAccess { idx }),
                &ValueAccessor::FieldAccessByName { field_name } => {
                    Ok(ResolvedValueAccessor::FieldAccess { field_name })
                }
                ValueAccessor::FieldAccessByScalar { scalar_name } => {
                    let scalar_ref = exec_ctx.scalars.get(scalar_name)?;
                    use air_values::scalar::ScalarRef::*;

                    let value = match scalar_ref {
                        Value(value_aggregate) => &value_aggregate.value,
                        IterableValue(fold_state) => {
                            // it's safe because iterable always point to valid value
                            let item = fold_state.iterable.peek().unwrap();
                            item.value
                        }
                    };
                    resolve_value_to_accessor(value)
                }
            }
        })
        .collect::<ExecutionResult<Vec<_>>>()?;

    Ok(resolved_lambda)
}

fn resolve_value_to_accessor(value: &RcBoxedValue) -> ExecutionResult<ResolvedValueAccessor<'_>> {
    match (value.as_str(), value.as_u64()) {
        // value can't be both string and number type at the same moment
        (Some(_), Some(_)) => todo!(),
        (Some(field_name), None) => Ok(ResolvedValueAccessor::FieldAccess { field_name }),
        (None, Some(idx)) => Ok(ResolvedValueAccessor::ArrayAccess { idx: idx as u32 }),
        (None, None) => Err(Rc::new(LambdaError::ScalarAccessorHasInvalidType {
            scalar_accessor: value.to_string(),
        })).into(),
    }
}
