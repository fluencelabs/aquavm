/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use super::utils::*;
use super::LambdaError;
use crate::execution_step::execution_context::stream_map_key::StreamMapKey;
use crate::execution_step::value_types::CanonStreamMap;
use crate::execution_step::CatchableError;
use crate::execution_step::ExecutionCtx;
use crate::execution_step::ExecutionResult;
use crate::execution_step::RcSecurityTetraplet;
use crate::lambda_to_execution_error;
use crate::ExecutionError;
use crate::JValue;
use crate::LambdaAST;
use crate::SecurityTetraplet;

use air_interpreter_value::JsonString;
use air_lambda_ast::Functor;
use air_lambda_parser::ValueAccessor;
use non_empty_vec::NonEmpty;

use std::rc::Rc;

pub(crate) struct LambdaResult {
    pub(crate) result: JValue,
    pub(crate) tetraplet_idx: Option<usize>,
}

pub(crate) struct MapLensResult {
    pub(crate) result: JValue,
    pub(crate) tetraplet: RcSecurityTetraplet,
}

pub(crate) fn select_by_lambda_from_stream<'value>(
    stream: impl ExactSizeIterator<Item = &'value JValue> + 'value,
    lambda: &LambdaAST<'_>,
    exec_ctx: &ExecutionCtx<'_>,
) -> ExecutionResult<LambdaResult> {
    match lambda {
        LambdaAST::ValuePath(value_path) => select_by_path_from_stream(stream, value_path, exec_ctx),
        LambdaAST::Functor(functor) => Ok(select_by_functor_from_stream(stream, functor)),
    }
}

pub(crate) fn select_by_lambda_from_canon_map(
    canon_map: &CanonStreamMap,
    lambda: &LambdaAST<'_>,
    exec_ctx: &ExecutionCtx<'_>,
) -> ExecutionResult<MapLensResult> {
    match lambda {
        LambdaAST::ValuePath(value_path) => select_by_path_from_canon_map(canon_map, value_path, lambda, exec_ctx),
        LambdaAST::Functor(functor) => Ok(select_by_functor_from_canon_map(canon_map, exec_ctx, functor)),
    }
}

pub(crate) fn select_by_lambda_from_scalar(
    value: &JValue,
    lambda: &LambdaAST<'_>,
    exec_ctx: &ExecutionCtx<'_>,
) -> ExecutionResult<JValue> {
    match lambda {
        LambdaAST::ValuePath(value_path) => select_by_path_from_scalar(value, value_path.iter(), exec_ctx),
        LambdaAST::Functor(functor) => select_by_functor_from_scalar(value, functor),
    }
}

fn select_by_path_from_stream<'value>(
    stream: impl ExactSizeIterator<Item = &'value JValue> + 'value,
    lambda: &NonEmpty<ValueAccessor<'_>>,
    exec_ctx: &ExecutionCtx<'_>,
) -> ExecutionResult<LambdaResult> {
    let stream_size = stream.len();
    let (idx, body) = split_to_idx(lambda, exec_ctx)?;

    let value = lambda_to_execution_error!(stream
        .peekable()
        .nth(idx)
        .ok_or(LambdaError::CanonStreamNotHaveEnoughValues { stream_size, idx }))?;

    let result = select_by_path_from_scalar(value, body.iter(), exec_ctx)?;
    let select_result = LambdaResult::new(result, idx);
    Ok(select_result)
}

fn select_by_path_from_canon_map_stream<'value>(
    stream: impl ExactSizeIterator<Item = (JValue, RcSecurityTetraplet)> + 'value,
    lambda: &NonEmpty<ValueAccessor<'_>>,
    exec_ctx: &ExecutionCtx<'_>,
) -> ExecutionResult<MapLensResult> {
    let stream_size = stream.len();
    let (idx, body) = split_to_idx(lambda, exec_ctx)?;

    let (value, tetraplet) = lambda_to_execution_error!(stream
        .peekable()
        .nth(idx)
        .ok_or(LambdaError::CanonStreamNotHaveEnoughValues { stream_size, idx }))?;

    let select_result = if body.is_empty() {
        // csm.$.key.[0] case
        MapLensResult::new(value, tetraplet)
    } else {
        // csm.$.key.[0].attribute case
        let result = select_by_path_from_scalar(&value, body.iter(), exec_ctx)?;

        let joined = body.iter().map(ToString::to_string).collect::<Vec<_>>().join(".");
        let lambda_suffix = format!(".{}", joined);
        let prefix_with_path = true;
        let updated_tetraplet = update_tetraplet_with_path(&tetraplet, &lambda_suffix, prefix_with_path);

        MapLensResult::new(result, updated_tetraplet)
    };
    Ok(select_result)
}

fn select_by_path_from_canon_map(
    canon_map: &CanonStreamMap,
    lambda: &NonEmpty<ValueAccessor<'_>>,
    original_lambda: &LambdaAST<'_>,
    exec_ctx: &ExecutionCtx<'_>,
) -> ExecutionResult<MapLensResult> {
    use crate::execution_step::value_types::CanonStream;

    let (prefix, body) = lambda.split_first();

    // HashMap<'map>::get(key: &'key K) forces key's lifetime 'key to be as good as 'map.
    // This variance-derived requirement forces StreamMapKey here.
    // See https://github.com/rust-lang/rust/issues/80389#issuecomment-752067798
    // for the details.
    let stream_map_key: StreamMapKey = match prefix {
        ValueAccessor::ArrayAccess { idx } => (*idx).into(),
        ValueAccessor::FieldAccessByName { field_name } => JsonString::from(*field_name).to_owned().into(),
        ValueAccessor::FieldAccessByScalar { scalar_name } => {
            let scalar = exec_ctx.scalars.get_value(scalar_name)?;
            lambda_to_execution_error!(try_scalar_ref_as_stream_map_key(scalar))?
        }
        ValueAccessor::Error => unreachable!("should not execute if parsing succeeded. QED."),
    };
    let canon_stream = canon_map.index(&stream_map_key);

    // There will be an empty canon stream if the key was not found.
    let result = match (NonEmpty::try_from(body.to_vec()), canon_stream) {
        (Ok(body_part), Some(canon_stream)) => {
            // csm.$.key... case

            let canon_stream_iter = canon_stream.iter().map(|v| (v.get_result().clone(), v.get_tetraplet()));
            select_by_path_from_canon_map_stream(canon_stream_iter, &body_part, exec_ctx)?
        }
        (Err(..), Some(canon_stream)) => {
            // csm.$.key case
            let prefix_with_path = false;
            let tetraplet = update_tetraplet_with_path(canon_map.tetraplet(), original_lambda, prefix_with_path);
            let value = canon_stream.as_jvalue();

            MapLensResult::new(value, tetraplet)
        }
        _ => {
            // csm.$.non_existing_key case
            let prefix_with_path = false;
            let tetraplet = update_tetraplet_with_path(canon_map.tetraplet(), original_lambda, prefix_with_path);
            let value = CanonStream::new(vec![], tetraplet.clone()).as_jvalue();

            MapLensResult::new(value, tetraplet)
        }
    };
    Ok(result)
}

fn split_to_idx<'lambda>(
    lambda: &'lambda NonEmpty<ValueAccessor<'_>>,
    exec_ctx: &ExecutionCtx<'_>,
) -> ExecutionResult<(usize, &'lambda [ValueAccessor<'lambda>])> {
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
    Ok((idx as usize, body))
}

// TODO put this functionality into SecurityTetraplet method.
fn update_tetraplet_with_path(
    original_tetraplet: &SecurityTetraplet,
    original_path: &impl ToString,
    prefix_with_path: bool,
) -> RcSecurityTetraplet {
    let lens_updated = if prefix_with_path {
        original_tetraplet.lens.to_string() + &original_path.to_string()
    } else {
        original_path.to_string()
    };

    SecurityTetraplet {
        lens: lens_updated,
        ..original_tetraplet.clone()
    }
    .into()
}

fn select_by_functor_from_stream<'value>(
    stream: impl ExactSizeIterator<Item = &'value JValue> + 'value,
    functor: &Functor,
) -> LambdaResult {
    match functor {
        Functor::Length => {
            let result = (stream.len()).into();
            LambdaResult::from_value(result)
        }
    }
}

fn select_by_functor_from_canon_map(
    canon_map: &CanonStreamMap,
    exec_ctx: &ExecutionCtx<'_>,
    functor: &Functor,
) -> MapLensResult {
    match functor {
        Functor::Length => {
            let result = (canon_map.len()).into();
            MapLensResult::with_functor(result, exec_ctx, functor)
        }
    }
}

fn select_by_path_from_scalar<'value, 'accessor>(
    mut value: &'value JValue,
    lambda: impl Iterator<Item = &'accessor ValueAccessor<'accessor>>,
    exec_ctx: &ExecutionCtx<'_>,
) -> ExecutionResult<JValue> {
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

    Ok(value.clone())
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
            Ok(length.into())
        }
    }
}

impl LambdaResult {
    fn new(result: JValue, tetraplet_idx: usize) -> Self {
        Self {
            result,
            tetraplet_idx: Some(tetraplet_idx),
        }
    }

    fn from_value(result: JValue) -> Self {
        Self {
            result,
            tetraplet_idx: None,
        }
    }
}

impl MapLensResult {
    fn new(result: JValue, tetraplet: RcSecurityTetraplet) -> Self {
        Self { result, tetraplet }
    }

    fn with_functor(result: JValue, exec_ctx: &ExecutionCtx<'_>, functor: &Functor) -> Self {
        let tetraplet = Rc::new(SecurityTetraplet::new(
            exec_ctx.run_parameters.current_peer_id.to_string(),
            "",
            "",
            functor.to_string(),
        ));
        Self { result, tetraplet }
    }
}
