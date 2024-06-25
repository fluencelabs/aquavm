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

use crate::ast::*;
use std::rc::Rc;

pub(super) fn call<'i>(
    peer_pk: ResolvableToPeerIdVariable<'i>,
    service_id: ResolvableToStringVariable<'i>,
    function_name: ResolvableToStringVariable<'i>,
    args: Rc<Vec<ImmutableValue<'i>>>,
    output: CallOutputValue<'i>,
) -> Instruction<'i> {
    let triplet = Triplet {
        peer_id: peer_pk,
        service_id,
        function_name,
    };

    Instruction::Call(
        Call {
            triplet,
            args,
            output,
        }
        .into(),
    )
}

pub(super) fn seq<'i>(l: Instruction<'i>, r: Instruction<'i>) -> Instruction<'i> {
    Instruction::Seq(Seq(l, r).into())
}

pub(super) fn par<'i>(l: Instruction<'i>, r: Instruction<'i>) -> Instruction<'i> {
    Instruction::Par(Par(l, r).into())
}

pub(super) fn xor<'i>(l: Instruction<'i>, r: Instruction<'i>) -> Instruction<'i> {
    Instruction::Xor(Xor(l, r).into())
}

pub(super) fn seqnn() -> Instruction<'static> {
    seq(null(), null())
}

pub(super) fn new<'i>(
    argument: NewArgument<'i>,
    instruction: Instruction<'i>,
    span: Span,
) -> Instruction<'i> {
    Instruction::New(
        New {
            argument,
            instruction,
            span,
        }
        .into(),
    )
}

pub(super) fn never() -> Instruction<'static> {
    Instruction::Never(Never)
}

pub(super) fn null() -> Instruction<'static> {
    Instruction::Null(Null)
}

pub(super) fn fail_scalar(scalar: Scalar) -> Instruction<'_> {
    Instruction::Fail(Fail::Scalar(scalar).into())
}

pub(super) fn fail_scalar_wl(scalar: ScalarWithLambda) -> Instruction<'_> {
    Instruction::Fail(Fail::ScalarWithLambda(scalar).into())
}

pub(super) fn fail_literals(ret_code: i64, error_message: &str) -> Instruction<'_> {
    Instruction::Fail(
        Fail::Literal {
            ret_code,
            error_message,
        }
        .into(),
    )
}

pub(super) fn fail_last_error() -> Instruction<'static> {
    Instruction::Fail(Fail::LastError.into())
}

pub(super) fn fail_error() -> Instruction<'static> {
    Instruction::Fail(Fail::Error.into())
}

pub(super) fn fold_scalar_variable<'i>(
    scalar: Scalar<'i>,
    iterator: Scalar<'i>,
    instruction: Instruction<'i>,
    last_instruction: Option<Instruction<'i>>,
    span: Span,
) -> Instruction<'i> {
    Instruction::FoldScalar(
        FoldScalar {
            iterable: FoldScalarIterable::Scalar(scalar),
            iterator,
            instruction: Rc::new(instruction),
            last_instruction: last_instruction.map(Rc::new),
            span,
        }
        .into(),
    )
}

pub(super) fn fold_scalar_variable_wl<'i>(
    scalar: ScalarWithLambda<'i>,
    iterator: Scalar<'i>,
    instruction: Instruction<'i>,
    last_instruction: Option<Instruction<'i>>,
    span: Span,
) -> Instruction<'i> {
    Instruction::FoldScalar(
        FoldScalar {
            iterable: FoldScalarIterable::ScalarWithLambda(scalar),
            iterator,
            instruction: Rc::new(instruction),
            last_instruction: last_instruction.map(Rc::new),
            span,
        }
        .into(),
    )
}

pub(super) fn fold_scalar_canon_stream<'i>(
    canon_stream: CanonStream<'i>,
    iterator: Scalar<'i>,
    instruction: Instruction<'i>,
    last_instruction: Option<Instruction<'i>>,
    span: Span,
) -> Instruction<'i> {
    Instruction::FoldScalar(
        FoldScalar {
            iterable: FoldScalarIterable::CanonStream(canon_stream),
            iterator,
            instruction: Rc::new(instruction),
            last_instruction: last_instruction.map(Rc::new),
            span,
        }
        .into(),
    )
}

pub(super) fn fold_scalar_canon_stream_map<'i>(
    canon_stream_map: CanonStreamMap<'i>,
    iterator: Scalar<'i>,
    instruction: Instruction<'i>,
    last_instruction: Option<Instruction<'i>>,
    span: Span,
) -> Instruction<'i> {
    Instruction::FoldScalar(
        FoldScalar {
            iterable: FoldScalarIterable::CanonStreamMap(canon_stream_map),
            iterator,
            instruction: Rc::new(instruction),
            last_instruction: last_instruction.map(Rc::new),
            span,
        }
        .into(),
    )
}

pub(super) fn fold_scalar_empty_array<'i>(
    iterator: Scalar<'i>,
    instruction: Instruction<'i>,
    last_instruction: Option<Instruction<'i>>,
    span: Span,
) -> Instruction<'i> {
    Instruction::FoldScalar(
        FoldScalar {
            iterable: FoldScalarIterable::EmptyArray,
            iterator,
            instruction: Rc::new(instruction),
            last_instruction: last_instruction.map(Rc::new),
            span,
        }
        .into(),
    )
}

pub(super) fn fold_stream<'i>(
    iterable: Stream<'i>,
    iterator: Scalar<'i>,
    instruction: Instruction<'i>,
    last_instruction: Option<Instruction<'i>>,
    span: Span,
) -> Instruction<'i> {
    Instruction::FoldStream(
        FoldStream {
            iterable,
            iterator,
            instruction: Rc::new(instruction),
            last_instruction: last_instruction.map(Rc::new),
            span,
        }
        .into(),
    )
}

pub(super) fn match_<'i>(
    left_value: ImmutableValue<'i>,
    right_value: ImmutableValue<'i>,
    instruction: Instruction<'i>,
) -> Instruction<'i> {
    Instruction::Match(
        Match {
            left_value,
            right_value,
            instruction,
        }
        .into(),
    )
}

pub(super) fn mismatch<'i>(
    left_value: ImmutableValue<'i>,
    right_value: ImmutableValue<'i>,
    instruction: Instruction<'i>,
) -> Instruction<'i> {
    Instruction::MisMatch(
        MisMatch {
            left_value,
            right_value,
            instruction,
        }
        .into(),
    )
}

pub(super) fn ap<'i>(argument: ApArgument<'i>, result: ApResult<'i>) -> Instruction<'i> {
    Instruction::Ap(Ap { argument, result }.into())
}

pub(super) fn ap_with_map<'i>(
    key: StreamMapKeyClause<'i>,
    argument: ApArgument<'i>,
    result: StreamMap<'i>,
) -> Instruction<'i> {
    Instruction::ApMap(
        ApMap {
            key,
            value: argument,
            map: result,
        }
        .into(),
    )
}

pub(super) fn canon<'i>(
    peer_pk: ResolvableToPeerIdVariable<'i>,
    stream: Stream<'i>,
    canon_stream: CanonStream<'i>,
) -> Instruction<'i> {
    Instruction::Canon(
        Canon {
            peer_id: peer_pk,
            stream,
            canon_stream,
        }
        .into(),
    )
}

pub(super) fn canon_stream_map_scalar<'i>(
    peer_pk: ResolvableToPeerIdVariable<'i>,
    stream_map: StreamMap<'i>,
    scalar: Scalar<'i>,
) -> Instruction<'i> {
    Instruction::CanonStreamMapScalar(
        CanonStreamMapScalar {
            peer_id: peer_pk,
            stream_map,
            scalar,
        }
        .into(),
    )
}

pub(super) fn canon_stream_map_canon_map<'i>(
    peer_pk: ResolvableToPeerIdVariable<'i>,
    stream_map: StreamMap<'i>,
    canon_stream_map: CanonStreamMap<'i>,
) -> Instruction<'i> {
    Instruction::CanonMap(
        CanonMap {
            peer_id: peer_pk,
            stream_map,
            canon_stream_map,
        }
        .into(),
    )
}

pub(super) fn binary_instruction<'i, 'b>(
    name: &'i str,
) -> impl Fn(Instruction<'b>, Instruction<'b>) -> Instruction<'b> {
    match name {
        "xor" => xor,
        "par" => par,
        "seq" => seq,
        _ => unreachable!(),
    }
}
