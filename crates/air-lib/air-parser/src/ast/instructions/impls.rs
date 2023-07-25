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

use super::*;

impl<'i> Ap<'i> {
    pub fn new(argument: ApArgument<'i>, result: ApResult<'i>) -> Self {
        Self { argument, result }
    }
}

impl<'i> ApMap<'i> {
    pub fn new(key: StreamMapKeyClause<'i>, value: ApArgument<'i>, map: StreamMap<'i>) -> Self {
        Self { key, value, map }
    }
}

impl<'i> Call<'i> {
    pub fn new(
        triplet: Triplet<'i>,
        args: Rc<Vec<ImmutableValue<'i>>>,
        output: CallOutputValue<'i>,
    ) -> Self {
        Self {
            triplet,
            args,
            output,
        }
    }
}

impl<'i> Canon<'i> {
    pub fn new(
        peer_id: ResolvableToPeerIdVariable<'i>,
        stream: Stream<'i>,
        canon_stream: CanonStream<'i>,
    ) -> Self {
        Self {
            peer_id,
            stream,
            canon_stream,
        }
    }
}

impl<'i> CanonMap<'i> {
    pub fn new(
        peer_id: ResolvableToPeerIdVariable<'i>,
        stream_map: StreamMap<'i>,
        canon_stream_map: CanonStreamMap<'i>,
    ) -> Self {
        Self {
            peer_id,
            stream_map,
            canon_stream_map,
        }
    }
}

impl<'i> CanonStreamMapScalar<'i> {
    pub fn new(
        peer_id: ResolvableToPeerIdVariable<'i>,
        stream_map: StreamMap<'i>,
        scalar: Scalar<'i>,
    ) -> Self {
        Self {
            peer_id,
            stream_map,
            scalar,
        }
    }
}

impl<'i> Seq<'i> {
    pub fn new(
        left_instruction: Box<Instruction<'i>>,
        right_instruction: Box<Instruction<'i>>,
    ) -> Self {
        Self(left_instruction, right_instruction)
    }
}

impl<'i> Par<'i> {
    pub fn new(
        left_instruction: Box<Instruction<'i>>,
        right_instruction: Box<Instruction<'i>>,
    ) -> Self {
        Self(left_instruction, right_instruction)
    }
}

impl<'i> Xor<'i> {
    pub fn new(
        left_instruction: Box<Instruction<'i>>,
        right_instruction: Box<Instruction<'i>>,
    ) -> Self {
        Self(left_instruction, right_instruction)
    }
}

impl<'i> Match<'i> {
    pub fn new(
        left_value: ImmutableValue<'i>,
        right_value: ImmutableValue<'i>,
        instruction: Box<Instruction<'i>>,
    ) -> Self {
        Self {
            left_value,
            right_value,
            instruction,
        }
    }
}

impl<'i> MisMatch<'i> {
    pub fn new(
        left_value: ImmutableValue<'i>,
        right_value: ImmutableValue<'i>,
        instruction: Box<Instruction<'i>>,
    ) -> Self {
        Self {
            left_value,
            right_value,
            instruction,
        }
    }
}

impl<'i> FoldScalar<'i> {
    pub fn new(
        iterable: FoldScalarIterable<'i>,
        iterator: Scalar<'i>,
        instruction: Instruction<'i>,
        last_instruction: Option<Instruction<'i>>,
        span: Span,
    ) -> Self {
        Self {
            iterable,
            iterator,
            instruction: Rc::new(instruction),
            last_instruction: last_instruction.map(Rc::new),
            span,
        }
    }
}

impl<'i> FoldStream<'i> {
    pub fn new(
        iterable: Stream<'i>,
        iterator: Scalar<'i>,
        instruction: Instruction<'i>,
        last_instruction: Option<Instruction<'i>>,
        span: Span,
    ) -> Self {
        Self {
            iterable,
            iterator,
            instruction: Rc::new(instruction),
            last_instruction: last_instruction.map(Rc::new),
            span,
        }
    }
}

impl<'i> FoldStreamMap<'i> {
    pub fn new(
        iterable: StreamMap<'i>,
        iterator: Scalar<'i>,
        instruction: Instruction<'i>,
        last_instruction: Option<Instruction<'i>>,
        span: Span,
    ) -> Self {
        Self {
            iterable,
            iterator,
            instruction: Rc::new(instruction),
            last_instruction: last_instruction.map(Rc::new),
            span,
        }
    }
}

impl<'i> FoldCanonStreamMap<'i> {
    pub fn new(
        iterable: CanonStreamMap<'i>,
        iterator: Scalar<'i>,
        instruction: Instruction<'i>,
        last_instruction: Option<Instruction<'i>>,
        span: Span,
    ) -> Self {
        Self {
            iterable,
            iterator,
            instruction: Rc::new(instruction),
            last_instruction: last_instruction.map(Rc::new),
            span,
        }
    }
}

impl<'i> Next<'i> {
    pub fn new(iterator: Scalar<'i>) -> Self {
        Self { iterator }
    }
}

impl<'i> New<'i> {
    #[allow(clippy::self_named_constructors)]
    pub fn new(argument: NewArgument<'i>, instruction: Box<Instruction<'i>>, span: Span) -> Self {
        Self {
            argument,
            instruction,
            span,
        }
    }
}
