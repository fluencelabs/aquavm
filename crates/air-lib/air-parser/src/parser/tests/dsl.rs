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

    Instruction::Call(Call {
        triplet,
        args,
        output,
    })
}

pub(super) fn seq<'i>(l: &'i Instruction<'i>, r: &'i Instruction<'i>) -> Instruction<'i> {
    Instruction::Seq(Seq(l, r))
}

pub(super) fn par<'i>(l: &'i Instruction<'i>, r: &'i Instruction<'i>) -> Instruction<'i> {
    Instruction::Par(Par(l, r))
}

pub(super) fn xor<'i>(l: &'i Instruction<'i>, r: &'i Instruction<'i>) -> Instruction<'i> {
    Instruction::Xor(Xor(l, r))
}

// pub(super) fn seqnn() -> Instruction<'static> {
//     seq(null(), null())
// }

pub(super) fn new<'i>(
    argument: NewArgument<'i>,
    instruction: &'i Instruction<'i>,
    span: Span,
) -> Instruction<'i> {
    Instruction::New(New {
        argument,
        instruction: (instruction),
        span,
    })
}

pub(super) fn never() -> Instruction<'static> {
    Instruction::Never(Never)
}

pub(super) fn null() -> Instruction<'static> {
    Instruction::Null(Null)
}

pub(super) fn fail_scalar(scalar: Scalar) -> Instruction<'_> {
    Instruction::Fail(Fail::Scalar(scalar))
}

pub(super) fn fail_scalar_wl(scalar: ScalarWithLambda) -> Instruction<'_> {
    Instruction::Fail(Fail::ScalarWithLambda(scalar))
}

pub(super) fn fail_literals(ret_code: i64, error_message: &str) -> Instruction<'_> {
    Instruction::Fail(Fail::Literal {
        ret_code,
        error_message,
    })
}

pub(super) fn fail_last_error() -> Instruction<'static> {
    Instruction::Fail(Fail::LastError)
}

pub(super) fn fail_error() -> Instruction<'static> {
    Instruction::Fail(Fail::Error)
}

pub(super) fn fold_scalar_variable<'i>(
    scalar: Scalar<'i>,
    iterator: Scalar<'i>,
    instruction: &'i Instruction<'i>,
    last_instruction: Option<&'i Instruction<'i>>,
    span: Span,
) -> Instruction<'i> {
    Instruction::FoldScalar(FoldScalar {
        iterable: FoldScalarIterable::Scalar(scalar),
        iterator,
        instruction,
        last_instruction,
        span,
    })
}

pub(super) fn fold_scalar_variable_wl<'i>(
    scalar: ScalarWithLambda<'i>,
    iterator: Scalar<'i>,
    instruction: &'i Instruction<'i>,
    last_instruction: Option<&'i Instruction<'i>>,
    span: Span,
) -> Instruction<'i> {
    Instruction::FoldScalar(FoldScalar {
        iterable: FoldScalarIterable::ScalarWithLambda(scalar),
        iterator,
        instruction,
        last_instruction,
        span,
    })
}

pub(super) fn fold_scalar_canon_stream<'i>(
    canon_stream: CanonStream<'i>,
    iterator: Scalar<'i>,
    instruction: &'i Instruction<'i>,
    last_instruction: Option<&'i Instruction<'i>>,
    span: Span,
) -> Instruction<'i> {
    Instruction::FoldScalar(FoldScalar {
        iterable: FoldScalarIterable::CanonStream(canon_stream),
        iterator,
        instruction,
        last_instruction,
        span,
    })
}

pub(super) fn fold_scalar_canon_stream_map<'i>(
    canon_stream_map: CanonStreamMap<'i>,
    iterator: Scalar<'i>,
    instruction: &'i Instruction<'i>,
    last_instruction: Option<&'i Instruction<'i>>,
    span: Span,
) -> Instruction<'i> {
    Instruction::FoldScalar(FoldScalar {
        iterable: FoldScalarIterable::CanonStreamMap(canon_stream_map),
        iterator,
        instruction,
        last_instruction,
        span,
    })
}

pub(super) fn fold_scalar_empty_array<'i>(
    iterator: Scalar<'i>,
    instruction: &'i Instruction<'i>,
    last_instruction: Option<&'i Instruction<'i>>,
    span: Span,
) -> Instruction<'i> {
    Instruction::FoldScalar(FoldScalar {
        iterable: FoldScalarIterable::EmptyArray,
        iterator,
        instruction,
        last_instruction,
        span,
    })
}

pub(super) fn fold_stream<'i>(
    iterable: Stream<'i>,
    iterator: Scalar<'i>,
    instruction: &'i Instruction<'i>,
    last_instruction: Option<&'i Instruction<'i>>,
    span: Span,
) -> Instruction<'i> {
    Instruction::FoldStream(FoldStream {
        iterable,
        iterator,
        instruction,
        last_instruction,
        span,
    })
}

pub(super) fn match_<'i>(
    left_value: ImmutableValue<'i>,
    right_value: ImmutableValue<'i>,
    instruction: &'i Instruction<'i>,
) -> Instruction<'i> {
    Instruction::Match(Match {
        left_value,
        right_value,
        instruction,
    })
}

pub(super) fn mismatch<'i>(
    left_value: ImmutableValue<'i>,
    right_value: ImmutableValue<'i>,
    instruction: &'i Instruction<'i>,
) -> Instruction<'i> {
    Instruction::MisMatch(MisMatch {
        left_value,
        right_value,
        instruction,
    })
}

pub(super) fn ap<'i>(argument: ApArgument<'i>, result: ApResult<'i>) -> Instruction<'i> {
    Instruction::Ap(Ap { argument, result })
}

pub(super) fn ap_with_map<'i>(
    key: StreamMapKeyClause<'i>,
    argument: ApArgument<'i>,
    result: StreamMap<'i>,
) -> Instruction<'i> {
    Instruction::ApMap(ApMap {
        key,
        value: argument,
        map: result,
    })
}

pub(super) fn canon<'i>(
    peer_pk: ResolvableToPeerIdVariable<'i>,
    stream: Stream<'i>,
    canon_stream: CanonStream<'i>,
) -> Instruction<'i> {
    Instruction::Canon(Canon {
        peer_id: peer_pk,
        stream,
        canon_stream,
    })
}

pub(super) fn canon_stream_map_scalar<'i>(
    peer_pk: ResolvableToPeerIdVariable<'i>,
    stream_map: StreamMap<'i>,
    scalar: Scalar<'i>,
) -> Instruction<'i> {
    Instruction::CanonStreamMapScalar(CanonStreamMapScalar {
        peer_id: peer_pk,
        stream_map,
        scalar,
    })
}

pub(super) fn canon_stream_map_canon_map<'i>(
    peer_pk: ResolvableToPeerIdVariable<'i>,
    stream_map: StreamMap<'i>,
    canon_stream_map: CanonStreamMap<'i>,
) -> Instruction<'i> {
    Instruction::CanonMap(CanonMap {
        peer_id: peer_pk,
        stream_map,
        canon_stream_map,
    })
}

pub(super) fn binary_instruction<'i, 'b>(
    name: &'i str,
) -> impl Fn(&'i Instruction<'i>, &'i Instruction<'i>) -> Instruction<'i> {
    match name {
        "xor" => xor,
        "par" => par,
        "seq" => seq,
        _ => unreachable!(),
    }
}
