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

use air_parser::ast::MatchableValue;

pub(crate) fn is_matchable_eq<'ctx>(
    left: &MatchableValue<'_>,
    right: &MatchableValue<'_>,
    exec_ctx: &'ctx ExecutionCtx<'_>,
) -> ExecutionResult<bool> {
    use MatchableValue::*;

    match (left, right) {
        (Literal(name), matchable) => compare_matchable_and_literal(matchable, name, exec_ctx),
        (matchable, Literal(name)) => compare_matchable_and_literal(matchable, name, exec_ctx),
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

fn compare_matchable_and_literal<'ctx>(
    matchable: &MatchableValue<'_>,
    string_literal: &str,
    exec_ctx: &'ctx ExecutionCtx<'_>,
) -> ExecutionResult<bool> {
    use std::borrow::Cow;
    use MatchableValue::*;

    fn compare_jvalue_and_literal(jvalue: Cow<'_, JValue>, string_literal: &str) -> bool {
        use std::ops::Deref;

        match jvalue.deref() {
            JValue::String(value) => value == string_literal,
            _ => false,
        }
    }

    match matchable {
        Literal(name) => Ok(name == &string_literal),
        Variable(name) => {
            let jvaluable = resolve_to_jvaluable(name, exec_ctx)?;
            let jvalue = jvaluable.as_jvalue();
            Ok(compare_jvalue_and_literal(jvalue, string_literal))
        }
        JsonPath { variable, path } => {
            let jvaluable = resolve_to_jvaluable(variable, exec_ctx)?;
            let jvalues = jvaluable.apply_json_path(path)?;
            if jvalues.len() != 1 {
                return Ok(false);
            }

            Ok(compare_jvalue_and_literal(Cow::Borrowed(jvalues[0]), string_literal))
        }
    }
}
