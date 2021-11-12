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

pub(super) fn seq<'a>(l: Instruction<'a>, r: Instruction<'a>) -> Instruction<'a> {
    Instruction::Seq(Seq(Box::new(l), Box::new(r)))
}

pub(super) fn par<'a>(l: Instruction<'a>, r: Instruction<'a>) -> Instruction<'a> {
    Instruction::Par(Par(Box::new(l), Box::new(r)))
}

pub(super) fn xor<'a>(l: Instruction<'a>, r: Instruction<'a>) -> Instruction<'a> {
    Instruction::Xor(Xor(Box::new(l), Box::new(r)))
}

pub(super) fn seqnn() -> Instruction<'static> {
    seq(null(), null())
}

pub(super) fn null() -> Instruction<'static> {
    Instruction::Null(Null)
}

pub(super) fn fold_scalar<'a>(
    iterable: ScalarWithLambda<'a>,
    iterator: &'a str,
    instruction: Instruction<'a>,
) -> Instruction<'a> {
    Instruction::FoldScalar(FoldScalar {
        iterable,
        iterator: Scalar::new(iterator),
        instruction: std::rc::Rc::new(instruction),
    })
}

pub(super) fn fold_stream<'a>(
    stream_name: &'a str,
    iterator: &'a str,
    instruction: Instruction<'a>,
) -> Instruction<'a> {
    Instruction::FoldStream(FoldStream {
        iterable: Stream::new(stream_name),
        iterator: Scalar::new(iterator),
        instruction: std::rc::Rc::new(instruction),
    })
}

pub(super) fn match_<'a>(
    left_value: Value<'a>,
    right_value: Value<'a>,
    instruction: Instruction<'a>,
) -> Instruction<'a> {
    Instruction::Match(Match {
        left_value,
        right_value,
        instruction: Box::new(instruction),
    })
}

pub(super) fn mismatch<'a>(
    left_value: Value<'a>,
    right_value: Value<'a>,
    instruction: Instruction<'a>,
) -> Instruction<'a> {
    Instruction::MisMatch(MisMatch {
        left_value,
        right_value,
        instruction: Box::new(instruction),
    })
}

pub(super) fn ap<'i>(argument: ApArgument<'i>, result: Variable<'i>) -> Instruction<'i> {
    Instruction::Ap(Ap { argument, result })
}

pub(super) fn binary_instruction<'a, 'b>(
    name: &'a str,
) -> impl Fn(Instruction<'b>, Instruction<'b>) -> Instruction<'b> {
    match name {
        "xor" => |l, r| xor(l, r),
        "par" => |l, r| par(l, r),
        "seq" => |l, r| seq(l, r),
        _ => unreachable!(),
    }
}
