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
use super::LambdaResult;
use crate::JValue;
use crate::LambdaAST;

use air_lambda_parser::ValueAccessor;

pub(crate) struct StreamSelectResult<'value> {
    pub(crate) result: &'value JValue,
    pub(crate) tetraplet_idx: usize,
}

pub(crate) fn select_from_stream<'value>(
    stream: impl ExactSizeIterator<Item = &'value JValue> + 'value,
    lambda: &LambdaAST<'_>,
) -> LambdaResult<StreamSelectResult<'value>> {
    use ValueAccessor::*;

    let (prefix, body) = lambda.split_first();
    let idx = match prefix {
        ArrayAccess { idx } => *idx,
        FieldAccess { field_name } => {
            return Err(LambdaError::FieldAccessorAppliedToStream {
                field_name: field_name.to_string(),
            })
        }
        _ => unreachable!("should not execute if parsing succeeded. QED."),
    };

    let stream_size = stream.len();
    let mut stream = stream.peekable();
    for _ in 0..idx {
        let _ = stream.next();
    }

    let value = stream
        .peek()
        .ok_or(LambdaError::StreamNotHaveEnoughValues { stream_size, idx })?;

    let result = select(value, body.iter())?;
    let select_result = StreamSelectResult::new(result, idx);
    Ok(select_result)
}

pub(crate) fn select<'value, 'algebra>(
    mut value: &'value JValue,
    lambda: impl Iterator<Item = &'algebra ValueAccessor<'algebra>>,
) -> LambdaResult<&'value JValue> {
    for value_algebra in lambda {
        match value_algebra {
            ValueAccessor::ArrayAccess { idx } => {
                value = try_jvalue_with_idx(value, *idx)?;
            }
            ValueAccessor::FieldAccess { field_name } => {
                value = try_jvalue_with_field_name(value, *field_name)?;
            }
            ValueAccessor::Error => unreachable!("should not execute if parsing succeeded. QED."),
        }
    }

    Ok(value)
}

impl<'value> StreamSelectResult<'value> {
    pub(self) fn new(result: &'value JValue, tetraplet_idx: u32) -> Self {
        Self {
            result,
            tetraplet_idx: tetraplet_idx as usize,
        }
    }
}
