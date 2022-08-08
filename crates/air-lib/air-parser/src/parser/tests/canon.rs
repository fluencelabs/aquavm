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

use super::dsl::*;
use super::parse;
use crate::ast::*;

use fstrings::f;
use fstrings::format_args_f;

#[test]
fn canon_with_literal_peer_id() {
    let peer_id = "peer_id";
    let stream = "$stream";
    let canon_stream = "#canon_stream";
    let source_code = f!(r#"
        (canon "{peer_id}" {stream} {canon_stream})
    "#);

    let actual = parse(&source_code);
    let expected = canon(
        CallInstrValue::Literal(peer_id),
        Stream::new(stream, 26),
        CanonStream::new(canon_stream, 34),
    );

    assert_eq!(actual, expected);
}

#[test]
fn canon_with_variable_peer_id() {
    let peer_id = "peer_id";
    let stream = "$stream";
    let canon_stream = "#canon_stream";
    let source_code = f!(r#"
        (canon {peer_id} {stream} {canon_stream})
    "#);

    let actual = parse(&source_code);
    let expected = canon(
        CallInstrValue::Variable(VariableWithLambda::scalar(peer_id, 16)),
        Stream::new(stream, 24),
        CanonStream::new(canon_stream, 32),
    );

    assert_eq!(actual, expected);
}
