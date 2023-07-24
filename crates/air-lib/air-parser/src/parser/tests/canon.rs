/*
 * Copyright 2022 Fluence Labs Limited
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

#[test]
fn canon_with_literal_peer_id() {
    let peer_id = "peer_id";
    let stream = "$stream";
    let canon_stream = "#canon_stream";
    let source_code = format!(
        r#"
        (canon "{peer_id}" {stream} {canon_stream})
    "#
    );

    let actual = parse(&source_code);
    let expected = canon(
        ResolvableToPeerIdVariable::Literal(peer_id),
        Stream::new(stream, 26.into()),
        CanonStream::new(canon_stream, 34.into()),
    );

    assert_eq!(actual, expected);
}

#[test]
fn canon_with_variable_peer_id() {
    let peer_id = "peer_id";
    let stream = "$stream";
    let canon_stream = "#canon_stream";
    let source_code = format!(
        r#"
        (canon {peer_id} {stream} {canon_stream})
    "#
    );

    let actual = parse(&source_code);
    let expected = canon(
        ResolvableToPeerIdVariable::Scalar(Scalar::new(peer_id, 16.into())),
        Stream::new(stream, 24.into()),
        CanonStream::new(canon_stream, 32.into()),
    );

    assert_eq!(actual, expected);
}

#[test]
fn canon_with_stream_map_to_scalar() {
    let peer_id = "peer_id";
    let stream_map = "%stream_map";
    let scalar = "scalar";
    let source_code = format!(
        r#"
        (canon {peer_id} {stream_map} {scalar})
    "#
    );

    let actual = parse(&source_code);
    let expected = canon_stream_map_scalar(
        ResolvableToPeerIdVariable::Scalar(Scalar::new(peer_id, 16.into())),
        StreamMap::new(stream_map, 24.into()),
        Scalar::new(scalar, 36.into()),
    );

    assert_eq!(actual, expected, "{:#?} {:#?}", actual, expected);
}

#[test]
fn canon_with_stream_map_to_canon_stream_map() {
    let peer_id = "peer_id";
    let stream_map = "%stream_map";
    let canon_stream_map = "#%canon_stream_map";
    let source_code = format!(
        r#"
        (canon {peer_id} {stream_map} {canon_stream_map})
    "#
    );

    let actual = parse(&source_code);

    let expected = canon_stream_map_canon_map(
        ResolvableToPeerIdVariable::Scalar(Scalar::new(peer_id, 16.into())),
        StreamMap::new(stream_map, 24.into()),
        CanonStreamMap::new(canon_stream_map, 36.into()),
    );

    assert_eq!(actual, expected, "{:#?} {:#?}", actual, expected);
}
