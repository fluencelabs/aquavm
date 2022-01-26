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
use super::LambdaResult;
use crate::execution_step::ScalarRef;
use crate::JValue;

pub(super) fn try_jvalue_with_idx(jvalue: &JValue, idx: u32) -> LambdaResult<&JValue> {
    match jvalue {
        JValue::Array(values) => values
            .get(idx as usize)
            .ok_or_else(|| LambdaError::ValueNotContainSuchArrayIdx {
                value: jvalue.clone(),
                idx,
            }),
        _ => Err(LambdaError::ArrayAccessorNotMatchValue {
            value: jvalue.clone(),
            idx,
        }),
    }
}

pub(super) fn try_jvalue_with_field_name<'value>(
    jvalue: &'value JValue,
    field_name: &str,
) -> LambdaResult<&'value JValue> {
    match jvalue {
        JValue::Object(values_map) => values_map
            .get(field_name)
            .ok_or_else(|| LambdaError::ValueNotContainSuchField {
                value: jvalue.clone(),
                field_name: field_name.to_string(),
            }),
        _ => Err(LambdaError::FieldAccessorNotMatchValue {
            value: jvalue.clone(),
            field_name: field_name.to_string(),
        }),
    }
}

pub(super) fn select_by_scalar<'value, 'i>(
    value: &'value JValue,
    scalar_ref: ScalarRef<'i>,
) -> LambdaResult<&'value JValue> {
    use ScalarRef::*;

    match scalar_ref {
        Value(lambda_value) => select_by_jvalue(value, &lambda_value.result),
        IterableValue(fold_state) => {
            // it's safe because iterable always point to valid value
            let accessor = fold_state.iterable.peek().unwrap().into_resolved_result();
            select_by_jvalue(value, &accessor.result)
        }
    }
}

pub(super) fn try_scalar_ref_as_idx(scalar: ScalarRef<'_>) -> LambdaResult<u32> {
    match scalar {
        ScalarRef::Value(accessor) => try_jvalue_as_idx(&accessor.result),
        ScalarRef::IterableValue(accessor) => {
            // it's safe because iterable always point to valid value
            let accessor = accessor.iterable.peek().unwrap().into_resolved_result();
            try_jvalue_as_idx(&accessor.result)
        }
    }
}

fn select_by_jvalue<'value>(value: &'value JValue, accessor: &JValue) -> LambdaResult<&'value JValue> {
    match accessor {
        JValue::String(string_accessor) => try_jvalue_with_field_name(value, string_accessor),
        JValue::Number(number_accessor) => {
            let idx = try_number_to_u32(number_accessor)?;
            try_jvalue_with_idx(value, idx)
        }
        scalar_accessor => Err(LambdaError::ScalarAccessorHasInvalidType {
            scalar_accessor: scalar_accessor.clone(),
        }),
    }
}

fn try_jvalue_as_idx(jvalue: &JValue) -> LambdaResult<u32> {
    match jvalue {
        JValue::Number(number) => try_number_to_u32(number),
        scalar_accessor => Err(LambdaError::StreamAccessorHasInvalidType {
            scalar_accessor: scalar_accessor.clone(),
        }),
    }
}

fn try_number_to_u32(accessor: &serde_json::Number) -> LambdaResult<u32> {
    use std::convert::TryFrom;

    accessor
        .as_u64()
        .and_then(|v| u32::try_from(v).ok())
        .ok_or(LambdaError::IndexAccessNotU32 {
            accessor: accessor.clone(),
        })
}
