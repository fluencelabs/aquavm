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
    peer_pk: CallInstrValue<'i>,
    service_id: CallInstrValue<'i>,
    function_name: CallInstrValue<'i>,
    args: Rc<Vec<Value<'i>>>,
    output: CallOutputValue<'i>,
) -> Instruction<'i> {
    let triplet = Triplet {
        peer_pk,
        service_id,
        function_name,
    };

    Instruction::Call(Call {
        triplet,
        args,
        output,
    })
}

pub(super) fn seq<'i>(l: Instruction<'i>, r: Instruction<'i>) -> Instruction<'i> {
    Instruction::Seq(Seq(Box::new(l), Box::new(r)))
}

pub(super) fn par<'i>(l: Instruction<'i>, r: Instruction<'i>) -> Instruction<'i> {
    Instruction::Par(Par(Box::new(l), Box::new(r)))
}

pub(super) fn xor<'i>(l: Instruction<'i>, r: Instruction<'i>) -> Instruction<'i> {
    Instruction::Xor(Xor(Box::new(l), Box::new(r)))
}

pub(super) fn seqnn() -> Instruction<'static> {
    seq(null(), null())
}

pub(super) fn new<'i>(
    variable: Variable<'i>,
    instruction: Instruction<'i>,
    span: Span,
) -> Instruction<'i> {
    Instruction::New(New {
        variable,
        instruction: Box::new(instruction),
        span,
    })
}

pub(super) fn null() -> Instruction<'static> {
    Instruction::Null(Null)
}

pub(super) fn fail_scalar(scalar: ScalarWithLambda) -> Instruction<'_> {
    Instruction::Fail(Fail::Scalar(scalar))
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

pub(super) fn fold_scalar_variable<'i>(
    scalar: ScalarWithLambda<'i>,
    iterator: Scalar<'i>,
    instruction: Instruction<'i>,
    span: Span,
) -> Instruction<'i> {
    Instruction::FoldScalar(FoldScalar {
        iterable: FoldScalarIterable::Scalar(scalar),
        iterator,
        instruction: Rc::new(instruction),
        span,
    })
}

pub(super) fn fold_scalar_empty_array<'i>(
    iterator: Scalar<'i>,
    instruction: Instruction<'i>,
    span: Span,
) -> Instruction<'i> {
    Instruction::FoldScalar(FoldScalar {
        iterable: FoldScalarIterable::EmptyArray,
        iterator,
        instruction: Rc::new(instruction),
        span,
    })
}

pub(super) fn fold_stream<'i>(
    iterable: Stream<'i>,
    iterator: Scalar<'i>,
    instruction: Instruction<'i>,
    span: Span,
) -> Instruction<'i> {
    Instruction::FoldStream(FoldStream {
        iterable,
        iterator,
        instruction: Rc::new(instruction),
        span,
    })
}

pub(super) fn match_<'i>(
    left_value: Value<'i>,
    right_value: Value<'i>,
    instruction: Instruction<'i>,
) -> Instruction<'i> {
    Instruction::Match(Match {
        left_value,
        right_value,
        instruction: Box::new(instruction),
    })
}

pub(super) fn mismatch<'i>(
    left_value: Value<'i>,
    right_value: Value<'i>,
    instruction: Instruction<'i>,
) -> Instruction<'i> {
    Instruction::MisMatch(MisMatch {
        left_value,
        right_value,
        instruction: Box::new(instruction),
    })
}

pub(super) fn ap<'i>(argument: ApArgument<'i>, result: Variable<'i>) -> Instruction<'i> {
    Instruction::Ap(Ap { argument, result })
}

pub(super) fn binary_instruction<'i, 'b>(
    name: &'i str,
) -> impl Fn(Instruction<'b>, Instruction<'b>) -> Instruction<'b> {
    match name {
        "xor" => |l, r| xor(l, r),
        "par" => |l, r| par(l, r),
        "seq" => |l, r| seq(l, r),
        _ => unreachable!(),
    }
}
