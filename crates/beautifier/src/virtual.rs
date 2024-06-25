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

use air_parser::ast;

use core::fmt;
use std::fmt::Display;

/// A virtual `hopon` instruction.
pub(crate) struct HopOn<'i> {
    pub peer_id: ast::ResolvableToPeerIdVariable<'i>,
}

impl Display for HopOn<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hopon {}", self.peer_id)
    }
}

/// Try to parse the `new` instruction and its nested elements as a virtual `hopon` instruction.
///
/// For example:
/// ```clojure
/// (new #uniq1_name
///    (new $uniq2_name
///       (canon peer_id $uniq2_name #uniq1_name)))
/// ```
/// is parsed as a virtual instruction
/// ```clojure
/// (hopon peer_id)
/// ```
pub(crate) fn try_hopon<'i>(root_new: &ast::New<'i>) -> Option<HopOn<'i>> {
    let expected_stream_name = &root_new.argument;

    if let (ast::Instruction::New(nested_new), ast::NewArgument::Stream(stream_name)) =
        (&root_new.instruction, expected_stream_name)
    {
        let expected_nested_canon_name = &nested_new.argument;

        if let (ast::Instruction::Canon(canon), ast::NewArgument::CanonStream(nested_canon_name)) =
            (&nested_new.instruction, expected_nested_canon_name)
        {
            if canon.canon_stream.name == nested_canon_name.name
                && canon.stream.name == stream_name.name
                // this condition handles case that is never generated by an Aqua compiler, but
                // can be crafted manually
                //
                // see `hopon_shadowing` test for an example
                && !canon_shadows_peer_id(nested_canon_name.name, &canon.peer_id)
            {
                return Some(HopOn {
                    peer_id: canon.peer_id.clone(),
                });
            }
        }
    }

    None
}

fn canon_shadows_peer_id(canon_name: &str, peer_id: &ast::ResolvableToPeerIdVariable<'_>) -> bool {
    use ast::ResolvableToPeerIdVariable::*;
    match peer_id {
        InitPeerId => false,
        Literal(_) => false,
        Scalar(_) => false,
        ScalarWithLambda(_) => false,
        CanonStreamMapWithLambda(_) => false,
        CanonStreamWithLambda(canon_with_lambda) => canon_with_lambda.name == canon_name,
    }
}