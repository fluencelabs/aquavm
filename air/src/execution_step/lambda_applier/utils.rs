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
        JValue::Object(values_map) => {
            values_map
                .get(field_name)
                .ok_or_else(|| LambdaError::JValueNotContainSuchField {
                    value: jvalue.clone(),
                    field_name: field_name.to_string(),
                })
        }
        _ => Err(LambdaError::FieldAccessorNotMatchValue {
            value: jvalue.clone(),
            field_name: field_name.to_string(),
        }),
    }
}
