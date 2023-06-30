/*
 * Copyright 2023 Fluence Labs Limited
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

use super::RcSecurityTetraplets;
use super::Resolvable;
use crate::execution_step::execution_context::ExecutionCtx;
use crate::execution_step::lambda_applier::select_by_lambda_from_scalar;
use crate::execution_step::value_types::JValuable;
use crate::execution_step::ExecutionResult;
use crate::JValue;
use crate::LambdaAST;
use crate::SecurityTetraplet;

use air_interpreter_data::Provenance;
use air_parser::ast;

use serde_json::json;
use std::rc::Rc;

/// Resolve value to called function arguments.
impl Resolvable for ast::ImmutableValue<'_> {
    fn resolve(&self, ctx: &ExecutionCtx<'_>) -> ExecutionResult<(JValue, RcSecurityTetraplets, Provenance)> {
        use ast::ImmutableValue::*;

        match self {
            InitPeerId => resolve_const(ctx.run_parameters.init_peer_id.as_ref(), ctx),
            LastError(error_accessor) => error_accessor.resolve(ctx),
            Literal(value) => resolve_const(value.to_string(), ctx),
            Timestamp => resolve_const(ctx.run_parameters.timestamp, ctx),
            TTL => resolve_const(ctx.run_parameters.ttl, ctx),
            Boolean(value) => resolve_const(*value, ctx),
            Number(value) => resolve_const(value, ctx),
            EmptyArray => resolve_const(json!([]), ctx),
            Variable(variable) => variable.resolve(ctx),
            VariableWithLambda(variable) => variable.resolve(ctx),
        }
    }
}

pub(crate) fn resolve_const(
    arg: impl Into<JValue>,
    ctx: &ExecutionCtx<'_>,
) -> ExecutionResult<(JValue, RcSecurityTetraplets, Provenance)> {
    let jvalue = arg.into();
    let tetraplet = SecurityTetraplet::literal_tetraplet(ctx.run_parameters.init_peer_id.as_ref());
    let tetraplet = Rc::new(tetraplet);

    Ok((jvalue, vec![tetraplet], Provenance::literal()))
}

impl Resolvable for Option<LambdaAST<'_>> {
    fn resolve(&self, ctx: &ExecutionCtx<'_>) -> ExecutionResult<(JValue, RcSecurityTetraplets, Provenance)> {
        use crate::LastError;

        let LastError {
            error,
            tetraplet,
            provenance,
        } = ctx.last_error();

        let jvalue = match self {
            Some(error_accessor) => select_by_lambda_from_scalar(error.as_ref(), error_accessor, ctx)?.into_owned(),
            None => error.as_ref().clone(),
        };

        let tetraplets = match tetraplet {
            Some(tetraplet) => vec![tetraplet.clone()],
            None => {
                let tetraplet = SecurityTetraplet::literal_tetraplet(ctx.run_parameters.init_peer_id.as_ref());
                let tetraplet = Rc::new(tetraplet);
                vec![tetraplet]
            }
        };

        Ok((jvalue, tetraplets, provenance.clone()))
    }
}

impl Resolvable for ast::Scalar<'_> {
    fn resolve(&self, ctx: &ExecutionCtx<'_>) -> ExecutionResult<(JValue, RcSecurityTetraplets, Provenance)> {
        let (value, provenance) = ctx.scalars.get_value(self.name)?.into_jvaluable();
        let tetraplets = value.as_tetraplets();
        Ok((value.into_jvalue(), tetraplets, provenance))
    }
}

impl Resolvable for ast::CanonStream<'_> {
    fn resolve(&self, ctx: &ExecutionCtx<'_>) -> ExecutionResult<(JValue, RcSecurityTetraplets, Provenance)> {
        let canon = ctx.scalars.get_canon_stream(self.name)?;
        let value: &dyn JValuable = &&canon.canon_stream;
        let tetraplets = value.as_tetraplets();
        Ok((
            value.as_jvalue().into_owned(),
            tetraplets,
            Provenance::canon(canon.cid.clone()),
        ))
    }
}

impl Resolvable for ast::ImmutableVariable<'_> {
    fn resolve(&self, ctx: &ExecutionCtx<'_>) -> ExecutionResult<(JValue, RcSecurityTetraplets, Provenance)> {
        match self {
            Self::Scalar(scalar) => scalar.resolve(ctx),
            Self::CanonStream(canon_stream) => canon_stream.resolve(ctx),
            Self::CanonStreamMap(canon_stream_map) => canon_stream_map.resolve(ctx),
        }
    }
}

impl Resolvable for ast::ScalarWithLambda<'_> {
    fn resolve(&self, ctx: &ExecutionCtx<'_>) -> ExecutionResult<(JValue, RcSecurityTetraplets, Provenance)> {
        let (value, root_provenance) = ctx.scalars.get_value(self.name)?.into_jvaluable();
        let (value, tetraplet, provenance) = value.apply_lambda_with_tetraplets(&self.lambda, ctx, &root_provenance)?;
        let tetraplet = Rc::new(tetraplet);
        Ok((value.into_owned(), vec![tetraplet], provenance))
    }
}

impl Resolvable for ast::CanonStreamWithLambda<'_> {
    fn resolve(&self, ctx: &ExecutionCtx<'_>) -> ExecutionResult<(JValue, RcSecurityTetraplets, Provenance)> {
        let canon = ctx.scalars.get_canon_stream(self.name)?;
        let value: &dyn JValuable = &&canon.canon_stream;
        let (value, tetraplet, provenance) =
            value.apply_lambda_with_tetraplets(&self.lambda, ctx, &Provenance::canon(canon.cid.clone()))?;
        let tetraplet = Rc::new(tetraplet);
        Ok((value.into_owned(), vec![tetraplet], provenance))
    }
}

impl Resolvable for ast::ImmutableVariableWithLambda<'_> {
    fn resolve(&self, ctx: &ExecutionCtx<'_>) -> ExecutionResult<(JValue, RcSecurityTetraplets, Provenance)> {
        match self {
            Self::Scalar(scalar) => scalar.resolve(ctx),
            Self::CanonStream(canon_stream) => canon_stream.resolve(ctx),
            Self::CanonStreamMap(canon_stream_map) => canon_stream_map.resolve(ctx),
        }
    }
}

impl Resolvable for ast::StreamMapKeyClause<'_> {
    fn resolve(&self, ctx: &ExecutionCtx<'_>) -> ExecutionResult<(JValue, RcSecurityTetraplets, Provenance)> {
        match self {
            ast::StreamMapKeyClause::Literal(value) => resolve_const(value.to_string(), ctx),
            ast::StreamMapKeyClause::Int(value) => resolve_const(*value, ctx),
            ast::StreamMapKeyClause::Scalar(scalar) => scalar.resolve(ctx),
            ast::StreamMapKeyClause::ScalarWithLambda(scalar_with_lambda) => scalar_with_lambda.resolve(ctx),
            ast::StreamMapKeyClause::CanonStreamWithLambda(canon_with_lambda) => canon_with_lambda.resolve(ctx),
        }
    }
}

impl Resolvable for ast::CanonStreamMap<'_> {
    fn resolve(&self, ctx: &ExecutionCtx<'_>) -> ExecutionResult<(JValue, RcSecurityTetraplets, Provenance)> {
        let canon_stream_map_name = self.name;
        let canon_stream_map_with_prov = ctx.scalars.get_canon_map(canon_stream_map_name)?;
        let canon_stream_map = &canon_stream_map_with_prov.canon_stream_map;
        let value: &dyn JValuable = &canon_stream_map;
        let tetraplets = value.as_tetraplets();
        let provenance = Provenance::canon(canon_stream_map_with_prov.cid.clone());

        Ok((value.as_jvalue().into_owned(), tetraplets, provenance))
    }
}

impl Resolvable for ast::CanonStreamMapWithLambda<'_> {
    fn resolve(&self, ctx: &ExecutionCtx<'_>) -> ExecutionResult<(JValue, RcSecurityTetraplets, Provenance)> {
        let canon_stream_map_name = self.name;
        let canon_stream_map_with_prov = ctx.scalars.get_canon_map(canon_stream_map_name)?;
        let canon_stream_map = &canon_stream_map_with_prov.canon_stream_map;
        let root_provenance = Provenance::canon(canon_stream_map_with_prov.cid.clone());
        let (value, tetraplet, provenance) =
            canon_stream_map.apply_lambda_with_tetraplets(&self.lambda, ctx, &root_provenance)?;

        let tetraplet = Rc::new(tetraplet);

        Ok((value.into_owned(), vec![tetraplet], provenance))
    }
}
