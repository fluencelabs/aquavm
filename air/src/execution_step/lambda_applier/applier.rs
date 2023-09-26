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

use air_lambda_ast::Functor;
use air_lambda_parser::ValueAccessor;
use non_empty_vec::NonEmpty;

use std::borrow::Cow;
use std::convert::TryFrom;
use std::ops::Deref;
use std::rc::Rc;

pub(crate) struct LambdaResult<'value> {
    pub(crate) result: Cow<'value, JValue>,
    pub(crate) tetraplet_idx: Option<usize>,
}

pub(crate) struct MapLensResult<'value> {
    pub(crate) result: Cow<'value, JValue>,
    pub(crate) tetraplet: RcSecurityTetraplet,
}

pub(crate) fn select_by_lambda_from_stream<'value>(
    stream: impl ExactSizeIterator<Item = &'value JValue> + 'value,
    lambda: &LambdaAST<'_>,
    exec_ctx: &ExecutionCtx<'_>,
) -> ExecutionResult<LambdaResult<'value>> {
    match lambda {
        LambdaAST::ValuePath(value_path) => select_by_path_from_stream(stream, value_path, exec_ctx),
        LambdaAST::Functor(functor) => Ok(select_by_functor_from_stream(stream, functor)),
    }
}

pub(crate) fn select_by_lambda_from_canon_map<'value>(
    canon_map: &'value CanonStreamMap<'_>,
    lambda: &LambdaAST<'_>,
    exec_ctx: &ExecutionCtx<'_>,
) -> ExecutionResult<MapLensResult<'value>> {
    match lambda {
        LambdaAST::ValuePath(value_path) => select_by_path_from_canon_map(canon_map, value_path, exec_ctx),
        LambdaAST::Functor(functor) => Ok(select_by_functor_from_canon_map(canon_map, exec_ctx, functor)),
    }
}

pub(crate) fn select_by_lambda_from_scalar<'value>(
    value: &'value JValue,
    lambda: &LambdaAST<'_>,
    exec_ctx: &ExecutionCtx<'_>,
) -> ExecutionResult<Cow<'value, JValue>> {
    match lambda {
        LambdaAST::ValuePath(value_path) => select_by_path_from_scalar(value, value_path.iter(), exec_ctx),
        LambdaAST::Functor(functor) => select_by_functor_from_scalar(value, functor).map(Cow::Owned),
    }
}

fn select_by_path_from_stream<'value>(
    stream: impl ExactSizeIterator<Item = &'value JValue> + 'value,
    lambda: &NonEmpty<ValueAccessor<'_>>,
    exec_ctx: &ExecutionCtx<'_>,
) -> ExecutionResult<LambdaResult<'value>> {
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
    let idx = idx as usize;
    let stream_size = stream.len();
    let value = lambda_to_execution_error!(stream
        .peekable()
        .nth(idx)
        .ok_or(LambdaError::CanonStreamNotHaveEnoughValues { stream_size, idx }))?;

    let result = select_by_path_from_scalar(value, body.iter(), exec_ctx)?;
    let select_result = LambdaResult::from_cow(result, idx);
    Ok(select_result)
}

fn select_by_path_from_stream_<'value>(
    stream: impl ExactSizeIterator<Item = (&'value JValue, RcSecurityTetraplet)> + 'value,
    lambda: &NonEmpty<ValueAccessor<'_>>,
    exec_ctx: &ExecutionCtx<'_>,
) -> ExecutionResult<MapLensResult<'value>> {
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
    let idx = idx as usize;
    let stream_size = stream.len();

    let (value, tetraplet) = lambda_to_execution_error!(stream
        .peekable()
        .nth(idx)
        .ok_or(LambdaError::CanonStreamNotHaveEnoughValues { stream_size, idx }))?;

    // 3d case take tetraplet from this VA and apply the full lens into it.
    let select_result = if body.is_empty() {
        let result = Cow::Borrowed(value);
        MapLensResult::from_cow(result, tetraplet)
    } else {
        let SecurityTetraplet {
            peer_pk,
            service_id,
            function_name,
            json_path,
        } = tetraplet.as_ref();

        let json_path_suffix = body.iter().fold("".to_string(), |acc, va| acc + &va.to_string());
        let json_path = json_path.to_string() + &prefix.to_string() + &json_path_suffix;

        let tetraplet = SecurityTetraplet::new(peer_pk, service_id, function_name, json_path).into();

        let result = select_by_path_from_scalar(value, body.iter(), exec_ctx)?;
        MapLensResult::from_cow(result, tetraplet)
    };
    Ok(select_result)
}

fn select_by_path_from_canon_map<'value>(
    canon_map: &'value CanonStreamMap<'_>,
    lambda: &NonEmpty<ValueAccessor<'_>>,
    exec_ctx: &ExecutionCtx<'_>,
) -> ExecutionResult<MapLensResult<'value>> {
    use crate::execution_step::value_types::CanonStream;

    let (prefix, body) = lambda.split_first();

    // HashMap<'map>::get(key: &'key K) forces key's lifetime 'key to be as good as 'map.
    // This variance-derived requirement forces StreamMapKey<'static> here.
    // See https://github.com/rust-lang/rust/issues/80389#issuecomment-752067798
    // for the details.
    let stream_map_key: StreamMapKey<'_> = match prefix {
        ValueAccessor::ArrayAccess { idx } => (*idx).into(),
        ValueAccessor::FieldAccessByName { field_name } => (*field_name).to_owned().into(),
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
            let canon_stream_iter = canon_stream.iter().map(|v| (v.get_result().deref(), v.get_tetraplet()));
            select_by_path_from_stream_(canon_stream_iter, &body_part, exec_ctx)?
        }
        (Err(..), Some(canon_stream)) => {
            let value = Cow::Owned(canon_stream.as_jvalue());
            MapLensResult::from_cow(value, canon_map.tetraplet().clone())
        }
        _ => {
            let SecurityTetraplet {
                peer_pk,
                service_id,
                function_name,
                json_path,
            } = canon_map.tetraplet().as_ref();
            let json_path = json_path.to_string() + &prefix.to_string();

            let tetraplet: RcSecurityTetraplet =
                SecurityTetraplet::new(peer_pk, service_id, function_name, json_path).into();
            let value = CanonStream::new(vec![], tetraplet.clone()).as_jvalue();
            let value = Cow::Owned(value);
            MapLensResult::from_cow(value, tetraplet)
        }
    };
    Ok(result)
}

fn select_by_functor_from_stream<'value>(
    stream: impl ExactSizeIterator<Item = &'value JValue> + 'value,
    functor: &Functor,
) -> LambdaResult<'value> {
    match functor {
        Functor::Length => {
            let result = serde_json::json!(stream.len());
            LambdaResult::from_value(result)
        }
    }
}

fn select_by_functor_from_canon_map<'value>(
    canon_map: &CanonStreamMap<'_>,
    exec_ctx: &ExecutionCtx<'_>,
    functor: &Functor,
) -> MapLensResult<'value> {
    match functor {
        Functor::Length => {
            let result = serde_json::json!(canon_map.len());
            MapLensResult::from_value(result, exec_ctx, functor)
        }
    }
}

fn select_by_path_from_scalar<'value, 'accessor>(
    mut value: &'value JValue,
    lambda: impl Iterator<Item = &'accessor ValueAccessor<'accessor>>,
    exec_ctx: &ExecutionCtx<'_>,
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

impl<'value> LambdaResult<'value> {
    fn from_cow(result: Cow<'value, JValue>, tetraplet_idx: usize) -> Self {
        Self {
            result,
            tetraplet_idx: Some(tetraplet_idx),
        }
    }

    fn from_value(result: JValue) -> Self {
        Self {
            result: Cow::Owned(result),
            tetraplet_idx: None,
        }
    }
}

impl<'value> MapLensResult<'value> {
    fn from_cow(result: Cow<'value, JValue>, tetraplet: RcSecurityTetraplet) -> Self {
        Self { result, tetraplet }
    }

    fn from_value(result: JValue, exec_ctx: &ExecutionCtx<'_>, functor: &Functor) -> Self {
        let tetraplet = Rc::new(SecurityTetraplet::new(
            exec_ctx.run_parameters.current_peer_id.to_string(),
            "",
            "",
            functor.to_string(),
        ));
        Self {
            result: Cow::Owned(result),
            tetraplet,
        }
    }
}
