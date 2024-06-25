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
