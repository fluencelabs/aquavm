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

use super::RcSecurityTetraplets;
use super::Resolvable;
use crate::execution_step::execution_context::ExecutionCtx;
use crate::execution_step::lambda_applier::select_by_lambda_from_scalar;
use crate::execution_step::value_types::JValuable;
use crate::execution_step::ExecutionResult;
use crate::JValue;
use crate::SecurityTetraplet;

use air_interpreter_data::Provenance;
use air_lambda_ast::LambdaAST;
use air_parser::ast;

use air_parser::ast::InstructionErrorAST;
use std::rc::Rc;

/// Resolve value to called function arguments.
impl Resolvable for ast::ImmutableValue<'_> {
    fn resolve(&self, ctx: &ExecutionCtx<'_>) -> ExecutionResult<(JValue, RcSecurityTetraplets, Provenance)> {
        use ast::ImmutableValue::*;

        match self {
            InitPeerId => resolve_const(ctx.run_parameters.init_peer_id.as_ref(), ctx),
            Error(error_accessor) => error_accessor.resolve(ctx),
            LastError(error_accessor) => error_accessor.resolve(ctx),
            Literal(value) => resolve_const(value.to_string(), ctx),
            Timestamp => resolve_const(ctx.run_parameters.timestamp, ctx),
            TTL => resolve_const(ctx.run_parameters.ttl, ctx),
            Boolean(value) => resolve_const(*value, ctx),
            Number(value) => resolve_const(value, ctx),
            EmptyArray => resolve_const(vec![(); 0], ctx),
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

fn resolve_errors(
    instruction_error: &crate::InstructionError,
    lens: &Option<LambdaAST<'_>>,
    ctx: &ExecutionCtx<'_>,
) -> Result<(JValue, Vec<Rc<SecurityTetraplet>>, Provenance), crate::ExecutionError> {
    use crate::execution_step::InstructionError;

    let InstructionError {
        error,
        tetraplet,
        provenance,
        ..
    } = instruction_error;

    let jvalue = match lens {
        Some(error_accessor) => select_by_lambda_from_scalar(error, error_accessor, ctx)?,
        None => error.clone(),
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

impl<'lens> Resolvable for InstructionErrorAST<'lens> {
    fn resolve(&self, ctx: &ExecutionCtx<'_>) -> ExecutionResult<(JValue, RcSecurityTetraplets, Provenance)> {
        let instruction_error = ctx.error();
        resolve_errors(instruction_error, &self.lens, ctx)
    }
}

impl Resolvable for Option<LambdaAST<'_>> {
    fn resolve(&self, ctx: &ExecutionCtx<'_>) -> ExecutionResult<(JValue, RcSecurityTetraplets, Provenance)> {
        let instruction_error = ctx.last_error();
        resolve_errors(instruction_error, self, ctx)
    }
}

impl Resolvable for ast::Scalar<'_> {
    fn resolve(&self, ctx: &ExecutionCtx<'_>) -> ExecutionResult<(JValue, RcSecurityTetraplets, Provenance)> {
        let (value, provenance) = ctx.scalars.get_value(self.name)?.into_jvaluable();
        let tetraplets = value.as_tetraplets();
        Ok((value.as_jvalue(), tetraplets, provenance))
    }
}

impl Resolvable for ast::CanonStream<'_> {
    fn resolve(&self, ctx: &ExecutionCtx<'_>) -> ExecutionResult<(JValue, RcSecurityTetraplets, Provenance)> {
        let canon = ctx.scalars.get_canon_stream(self.name)?;
        let value: &dyn JValuable = &&canon.canon_stream;
        let tetraplets = value.as_tetraplets();
        Ok((value.as_jvalue(), tetraplets, Provenance::canon(canon.cid.clone())))
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
        Ok((value, vec![tetraplet], provenance))
    }
}

impl Resolvable for ast::CanonStreamWithLambda<'_> {
    fn resolve(&self, ctx: &ExecutionCtx<'_>) -> ExecutionResult<(JValue, RcSecurityTetraplets, Provenance)> {
        let canon = ctx.scalars.get_canon_stream(self.name)?;
        let value: &dyn JValuable = &&canon.canon_stream;
        let (value, tetraplet, provenance) =
            value.apply_lambda_with_tetraplets(&self.lambda, ctx, &Provenance::canon(canon.cid.clone()))?;
        let tetraplet = Rc::new(tetraplet);
        Ok((value, vec![tetraplet], provenance))
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

        Ok((value.as_jvalue(), tetraplets, provenance))
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

        Ok((value, vec![tetraplet], provenance))
    }
}
