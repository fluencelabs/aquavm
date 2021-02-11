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

use crate::contexts::execution::ExecutionCtx;
use crate::execution::air::ExecutionResult;
use crate::execution::utils::resolve_to_jvaluable;
use crate::JValue;

use air_parser::ast;
use air_parser::ast::MatchableValue;

pub(crate) fn are_matchable_eq<'ctx>(
    left: &MatchableValue<'_>,
    right: &MatchableValue<'_>,
    exec_ctx: &'ctx ExecutionCtx<'_>,
) -> ExecutionResult<bool> {
    use MatchableValue::*;

    match (left, right) {
        (Literal(value), matchable) => compare_matchable(matchable, exec_ctx, make_string_comparator(value)),
        (matchable, Literal(value)) => compare_matchable(matchable, exec_ctx, make_string_comparator(value)),

        (Boolean(value), matchable) => compare_matchable(matchable, exec_ctx, make_bool_comparator(value)),
        (matchable, Boolean(value)) => compare_matchable(matchable, exec_ctx, make_bool_comparator(value)),

        (Number(value), matchable) => compare_matchable(matchable, exec_ctx, make_number_comparator(value)),
        (matchable, Number(value)) => compare_matchable(matchable, exec_ctx, make_number_comparator(value)),

        (Variable(left_name), Variable(right_name)) => {
            let left_jvaluable = resolve_to_jvaluable(left_name, exec_ctx)?;
            let left_value = left_jvaluable.as_jvalue();

            let right_jvaluable = resolve_to_jvaluable(right_name, exec_ctx)?;
            let right_value = right_jvaluable.as_jvalue();

            Ok(left_value == right_value)
        }
        (JsonPath { variable: lv, path: lp }, JsonPath { variable: rv, path: rp }) => {
            let left_jvaluable = resolve_to_jvaluable(lv, exec_ctx)?;
            let left_value = left_jvaluable.apply_json_path(lp)?;

            let right_jvaluable = resolve_to_jvaluable(rv, exec_ctx)?;
            let right_value = right_jvaluable.apply_json_path(rp)?;

            Ok(left_value == right_value)
        }
        _ => Ok(false),
    }
}

use std::borrow::Cow;
type Comparator<'a> = Box<dyn Fn(Cow<'_, JValue>) -> bool + 'a>;

fn compare_matchable<'ctx>(
    matchable: &MatchableValue<'_>,
    exec_ctx: &'ctx ExecutionCtx<'_>,
    comparator: Comparator<'ctx>,
) -> ExecutionResult<bool> {
    use MatchableValue::*;

    match matchable {
        Literal(_) => unreachable!("this is covered by patter in the caller"),
        Number(_) => unreachable!("this is covered by patter in the caller"),
        Boolean(bool) => {
            let jvalue = JValue::Bool(*bool);
            Ok(comparator(Cow::Owned(jvalue)))
        }
        Variable(name) => {
            let jvaluable = resolve_to_jvaluable(name, exec_ctx)?;
            let jvalue = jvaluable.as_jvalue();
            Ok(comparator(jvalue))
        }
        JsonPath { variable, path } => {
            let jvaluable = resolve_to_jvaluable(variable, exec_ctx)?;
            let jvalues = jvaluable.apply_json_path(path)?;
            if jvalues.len() != 1 {
                return Ok(false);
            }

            Ok(comparator(Cow::Borrowed(jvalues[0])))
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
    use serde_json::Number;
    use std::ops::Deref;

    let comparable_number: Number = match comparable_number {
        ast::Number::Int(value) => {
            let number: Number = (*value).into();
            number
        }
        ast::Number::Float(value) => {
            Number::from_f64(*value).expect("it is checked by the lexer that it's a finite float point")
        }
    };

    let comparable_jvalue = JValue::Number(comparable_number);

    Box::new(move |jvalue: Cow<'_, JValue>| -> bool { jvalue.deref() == &comparable_jvalue })
}
