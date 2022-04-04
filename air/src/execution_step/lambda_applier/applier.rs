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

use super::LambdaError;
use crate::execution_step::ExecutionResult;
use crate::lambda_to_execution_error;

use air_lambda_ast::AIRLambda;
use air_lambda_ast::ResolvedValueAccessor;
use air_values::boxed_value::BoxedValue;

pub(crate) struct StreamSelectResult<'value> {
    pub(crate) result: &'value dyn BoxedValue,
    pub(crate) tetraplet_idx: usize,
}

pub(crate) fn select_from_stream<'value, 'i>(
    stream: impl ExactSizeIterator<Item = &'value dyn BoxedValue> + 'value,
    lambda: &AIRLambda<'_>,
) -> ExecutionResult<StreamSelectResult<'value>> {
    let prefix = lambda.take(1);
    let idx = match prefix {
        ResolvedValueAccessor::ArrayAccess { idx } => idx,
        ResolvedValueAccessor::FieldAccess { field_name } => {
            return lambda_to_execution_error!(Err(LambdaError::FieldAccessorAppliedToStream {
                field_name: field_name.to_string(),
            }));
        }
    };

    let stream_size = stream.len();
    let value = lambda_to_execution_error!(stream
        .peekable()
        .nth(idx as usize)
        .ok_or(LambdaError::StreamNotHaveEnoughValues { stream_size, idx }))?;

    let scalar_result = value.apply_lambda(lambda)?;
    let select_result = StreamSelectResult::new(scalar_result, idx);
    Ok(select_result)
}

/*
pub(crate) fn select_from_scalar<'value, 'accessor, 'i>(
    mut value: &'value dyn BoxedValue,
    lambda: impl Iterator<Item = &'accessor ResolvedValueAccessor<'accessor>>,
) -> ExecutionResult<&'value dyn BoxedValue> {
    use air_lambda_ast::ResolvedValueAccessor;

    for accessor in lambda {
        match accessor {
            ResolvedValueAccessor::ArrayAccess { idx } => {
                value = lambda_to_execution_error!(try_value_with_idx(value, *idx))?;
            }
            ResolvedValueAccessor::FieldAccess { field_name } => {
                value = lambda_to_execution_error!(try_jvalue_with_field_name(value, *field_name))?;
            }
        }
    }

    Ok(value)
}
 */

impl<'value> StreamSelectResult<'value> {
    pub(self) fn new(result: &'value dyn BoxedValue, tetraplet_idx: u32) -> Self {
        Self {
            result,
            tetraplet_idx: tetraplet_idx as usize,
        }
    }
}
