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
    pub fn new(left_instruction: Instruction<'i>, right_instruction: Instruction<'i>) -> Self {
        Self(left_instruction, right_instruction)
    }
}

impl<'i> Par<'i> {
    pub fn new(left_instruction: Instruction<'i>, right_instruction: Instruction<'i>) -> Self {
        Self(left_instruction, right_instruction)
    }
}

impl<'i> Xor<'i> {
    pub fn new(left_instruction: Instruction<'i>, right_instruction: Instruction<'i>) -> Self {
        Self(left_instruction, right_instruction)
    }
}

impl<'i> Match<'i> {
    pub fn new(
        left_value: ImmutableValue<'i>,
        right_value: ImmutableValue<'i>,
        instruction: Instruction<'i>,
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
        instruction: Instruction<'i>,
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

impl<'i> Next<'i> {
    pub fn new(iterator: Scalar<'i>) -> Self {
        Self { iterator }
    }
}

impl<'i> New<'i> {
    #[allow(clippy::self_named_constructors)]
    pub fn new(argument: NewArgument<'i>, instruction: Instruction<'i>, span: Span) -> Self {
        Self {
            argument,
            instruction,
            span,
        }
    }
}

impl<'i> Embed<'i> {
    pub fn new(
        args: Rc<Vec<ImmutableValue<'i>>>,
        script: &'i str,
        output: EmbedOutputValue<'i>,
    ) -> Self {
        Self {
            args,
            script,
            output,
        }
    }
}
