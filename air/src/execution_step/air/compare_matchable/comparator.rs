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

use crate::execution_step::air::ExecutionResult;
use crate::execution_step::execution_context::ExecutionCtx;
use crate::execution_step::utils::resolve_ast_variable_wl;
use crate::JValue;

use air_parser::ast;
use air_parser::ast::AIRValue;

pub(crate) fn are_matchable_eq<'ctx>(
    left: &AIRValue<'_>,
    right: &AIRValue<'_>,
    exec_ctx: &'ctx ExecutionCtx<'_>,
) -> ExecutionResult<bool> {
    use AIRValue::*;

    match (left, right) {
        (InitPeerId, InitPeerId) => Ok(true),
        (InitPeerId, matchable) | (matchable, InitPeerId) => compare_matchable(
            matchable,
            exec_ctx,
            make_string_comparator(exec_ctx.init_peer_id.as_str()),
        ),

        (LastError(left_error), LastError(right_error)) => Ok(left_error == right_error),
        (LastError(error), matchable) | (matchable, LastError(error)) => {
            compare_matchable(matchable, exec_ctx, make_string_comparator(&error.to_string()))
        }

        (Literal(left_name), Literal(right_name)) => Ok(left_name == right_name),
        (Literal(value), matchable) | (matchable, Literal(value)) => {
            compare_matchable(matchable, exec_ctx, make_string_comparator(value))
        }

        (Boolean(left_boolean), Boolean(right_boolean)) => Ok(left_boolean == right_boolean),
        (Boolean(value), matchable) | (matchable, Boolean(value)) => {
            compare_matchable(matchable, exec_ctx, make_bool_comparator(value))
        }

        (Number(left_number), Number(right_number)) => Ok(left_number == right_number),
        (Number(value), matchable) | (matchable, Number(value)) => {
            compare_matchable(matchable, exec_ctx, make_number_comparator(value))
        }

        (Variable(left_variable), Variable(right_variable)) => {
            let (left_value, _) = resolve_ast_variable_wl(left_variable, exec_ctx)?;
            let (right_value, _) = resolve_ast_variable_wl(right_variable, exec_ctx)?;

            Ok(left_value == right_value)
        }
        _ => Ok(false),
    }
}

use std::borrow::Cow;
type Comparator<'a> = Box<dyn Fn(Cow<'_, JValue>) -> bool + 'a>;

fn compare_matchable<'ctx>(
    matchable: &AIRValue<'_>,
    exec_ctx: &'ctx ExecutionCtx<'_>,
    comparator: Comparator<'ctx>,
) -> ExecutionResult<bool> {
    use AIRValue::*;

    match matchable {
        InitPeerId => {
            let init_peer_id = exec_ctx.init_peer_id.clone();
            let jvalue = init_peer_id.into();
            Ok(comparator(Cow::Owned(jvalue)))
        }
        LastError(error_path) => {
            let error_path = error_path.to_string();
            let jvalue = error_path.into();
            Ok(comparator(Cow::Owned(jvalue)))
        }
        Literal(str) => {
            let jvalue = str.to_string().into();
            Ok(comparator(Cow::Owned(jvalue)))
        }
        Number(number) => {
            let jvalue = number.into();
            Ok(comparator(Cow::Owned(jvalue)))
        }
        Boolean(bool) => {
            let jvalue = (*bool).into();
            Ok(comparator(Cow::Owned(jvalue)))
        }
        EmptyArray => {
            let jvalue = JValue::Array(vec![]);
            Ok(comparator(Cow::Owned(jvalue)))
        }
        Variable(variable) => {
            let (jvalue, _) = resolve_ast_variable_wl(variable, exec_ctx)?;
            Ok(comparator(Cow::Owned(jvalue)))
        }
    }
}

fn make_string_comparator(comparable_string: &str) -> Comparator<'_> {
    use std::ops::Deref;

    Box::new(move |jvalue: Cow<'_, JValue>| -> bool {
        match jvalue.deref() {
            JValue::String(value) => value == comparable_string,
            _ => false,
        }
    })
}

fn make_bool_comparator(comparable_bool: &bool) -> Comparator<'_> {
    use std::ops::Deref;

    let comparable_bool = *comparable_bool;
    Box::new(move |jvalue: Cow<'_, JValue>| -> bool {
        match jvalue.deref() {
            JValue::Bool(jvalue) => jvalue == &comparable_bool,
            _ => false,
        }
    })
}

fn make_number_comparator(comparable_number: &ast::Number) -> Comparator<'_> {
    use std::ops::Deref;

    let comparable_jvalue: JValue = comparable_number.into();
    Box::new(move |jvalue: Cow<'_, JValue>| -> bool { jvalue.deref() == &comparable_jvalue })
}
