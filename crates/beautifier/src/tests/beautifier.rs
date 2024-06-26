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

#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]

use crate::{beautify_to_string, Beautifier};

#[test]
fn ap_with_literal() {
    let script = r#"(ap "some_string" $stream)"#;
    let output = beautify_to_string(script).unwrap();

    assert_eq!(
        output,
        r#"ap "some_string" $stream
"#
    );
}

#[test]
fn ap_with_number() {
    let script = "(ap -100 $stream)";
    let output = beautify_to_string(script).unwrap();

    assert_eq!(
        output,
        r#"ap -100 $stream
"#
    );
}

#[test]
fn ap_with_bool() {
    let script = "(ap true $stream)";
    let output = beautify_to_string(script).unwrap();

    assert_eq!(
        output,
        r#"ap true $stream
"#
    );
}

#[test]
fn ap_with_last_error() {
    let script = "(ap %last_error%.$.message! $stream)";
    let output = beautify_to_string(script).unwrap();

    assert_eq!(output, "ap %last_error%.$.message $stream\n");
}

#[test]
fn ap_with_empty_array() {
    let script = "(ap [] $stream)";
    let output = beautify_to_string(script).unwrap();

    assert_eq!(output, "ap [] $stream\n");
}

#[test]
fn ap_with_init_peer_id() {
    let script = "(ap %init_peer_id% $stream)";
    let output = beautify_to_string(script).unwrap();

    assert_eq!(output, "ap %init_peer_id% $stream\n");
}

#[test]
fn ap_with_timestamp() {
    let script = "(ap %timestamp% $stream)";
    let output = beautify_to_string(script).unwrap();

    assert_eq!(output, "ap %timestamp% $stream\n");
}

#[test]
fn ap_with_ttl() {
    let script = r#"
        (ap %ttl% $stream)
    "#;
    let output = beautify_to_string(script).unwrap();

    assert_eq!(output, "ap %ttl% $stream\n");
}

#[test]
fn seq() {
    let script = "(seq (null) (null))";
    let output = beautify_to_string(script).unwrap();

    assert_eq!(
        output,
        "null
null
"
    )
}

#[test]
fn seq_nested_pre() {
    let script = "(seq (seq (null) (null)) (null))";
    let output = beautify_to_string(script).unwrap();

    assert_eq!(
        output,
        "null
null
null
"
    );
}

#[test]
fn seq_nested_post() {
    let script = "(seq (null) (seq (null) (null)))";
    let output = beautify_to_string(script).unwrap();

    assert_eq!(
        output,
        "null
null
null
"
    );
}

#[test]
fn par() {
    let script = "(par (null) (null))";
    let output = beautify_to_string(script).unwrap();

    assert_eq!(
        output,
        "par:
    null
|
    null
"
    );
}

#[test]
fn match_() {
    let script = r#"(seq
  (seq
      (call "a" ("" "") [] a)
      (call "b" ("" "") [] b))
  (match a b (null)))"#;
    let output = beautify_to_string(script).unwrap();

    assert_eq!(
        output,
        r#"a <- call "a" ("", "") []
b <- call "b" ("", "") []
match a b:
    null
"#,
    );
}

#[test]
fn mismatch() {
    let script = r#"(seq
  (seq
      (call "a" ("" "") [] a)
      (call "b" ("" "") [] b))
  (mismatch a b (null)))"#;
    let output = beautify_to_string(script).unwrap();

    assert_eq!(
        output,
        r#"a <- call "a" ("", "") []
b <- call "b" ("", "") []
mismatch a b:
    null
"#,
    );
}

#[test]
fn fail_last_error() {
    let script = "(fail %last_error%)";
    let output = beautify_to_string(script).unwrap();

    assert_eq!(output, "fail %last_error%\n");
}

#[test]
fn fail_expr() {
    let script = "(fail var)";
    let output = beautify_to_string(script).unwrap();

    assert_eq!(output, "fail var\n");
}

#[test]
fn fail_common() {
    let script = r#"(fail 123 "Message")"#;
    let output = beautify_to_string(script).unwrap();

    assert_eq!(
        output,
        r#"fail 123 "Message"
"#
    );
}

#[test]
fn fold_scalar() {
    let script = r#"(seq (call "it" ("" "") [] var) (fold var i (null)))"#;
    let output = beautify_to_string(script).unwrap();

    assert_eq!(
        output,
        r#"var <- call "it" ("", "") []
fold var i:
    null
"#
    );
}

#[test]
fn fold_scalar_with_last_instruction() {
    let script = r#"(seq (call "it" ("" "") [] var) (fold var i (null) (never)))"#;
    let output = beautify_to_string(script).unwrap();

    assert_eq!(
        output,
        r#"var <- call "it" ("", "") []
fold var i:
    null
last:
    never
"#
    );
}

#[test]
fn fold_stream() {
    let script = r#"(seq (call "it" ("" "") [] $var) (fold $var i (null)))"#;
    let output = beautify_to_string(script).unwrap();

    assert_eq!(
        output,
        r#"$var <- call "it" ("", "") []
fold $var i:
    null
"#
    );
}

#[test]
fn fold_stream_with_last_instruction() {
    let script = r#"(seq (call "it" ("" "") [] $var) (fold $var i (never) (null)))"#;
    let output = beautify_to_string(script).unwrap();

    assert_eq!(
        output,
        r#"$var <- call "it" ("", "") []
fold $var i:
    never
last:
    null
"#
    );
}

#[test]
fn call_var() {
    let script = "(call \"{0}\" (\"a\" \"b\") [\"stream_1\" \"stream_2\"] streamvar)";
    let output = beautify_to_string(script).unwrap();

    assert_eq!(
        output,
        "streamvar <- call \"{0}\" (\"a\", \"b\") [\"stream_1\", \"stream_2\"]\n"
    );
}

#[test]
fn call_novar() {
    let script = r#"(call "{0}" ("a" "b") ["stream_1" "stream_2"])"#;
    let output = beautify_to_string(script).unwrap();

    assert_eq!(
        output,
        r#"call "{0}" ("a", "b") ["stream_1", "stream_2"]
"#
    );
}

#[test]
fn call_noargs() {
    let script = r#"(call "{0}" ("a" "b") [])"#;
    let output = beautify_to_string(script).unwrap();

    assert_eq!(
        output,
        r#"call "{0}" ("a", "b") []
"#
    );
}

#[test]
fn next() {
    let script = r#"(seq (call "{0}" ("a" "b") ["stream_1"] j) (fold j i (next i)))"#;
    let output = beautify_to_string(script).unwrap();

    assert_eq!(
        output,
        r#"j <- call "{0}" ("a", "b") ["stream_1"]
fold j i:
    next i
"#
    );
}

#[test]
fn new() {
    let script = "(new var (seq (null) (null)))";
    let output = beautify_to_string(script).unwrap();

    assert_eq!(
        output,
        "new var:
    null
    null
"
    );
}

#[test]
fn null() {
    let script = "(null)";
    let output = beautify_to_string(script).unwrap();

    assert_eq!(output, "null\n");
}

#[test]
fn custom_indent_step() {
    let mut output = vec![];
    let mut beautifier = Beautifier::new_with_indent(&mut output, 2);
    let script = "(new var1 (new var (seq (null) (null))))";
    beautifier.beautify(script).unwrap();

    assert_eq!(
        String::from_utf8(output).unwrap(),
        "new var1:
  new var:
    null
    null
"
    );
}

#[test]
fn deeply_nested() {
    let script = include_str!("deeply_nested.air");
    let output = beautify_to_string(script).unwrap();
    let expected = include_str!("deeply_nested_expected.txt");
    assert_eq!(output, expected);
}

#[test]
fn fail_error() {
    let script = r#"(fail :error:)"#;
    let output = beautify_to_string(script).unwrap();
    assert_eq!(output, "fail :error:\n");
}

#[test]
fn hopon_on() {
    let script = r#"(new $ephemeral (new #ephemeral (canon "relay" $ephemeral #ephemeral)))"#;

    let mut output = vec![];
    let mut beautifier = Beautifier::new(&mut output).enable_all_patterns();
    beautifier.beautify(script).unwrap();

    assert_eq!(String::from_utf8(output).unwrap(), "hopon \"relay\"\n");
}

#[test]
fn hopon_off() {
    let script = r#"(new $ephemeral (new #ephemeral (canon "relay" $ephemeral #ephemeral)))"#;

    let mut output = vec![];
    let mut beautifier = Beautifier::new(&mut output);
    beautifier.beautify(script).unwrap();

    assert_eq!(
        String::from_utf8(output).unwrap(),
        concat!(
            "new $ephemeral:\n",
            "    new #ephemeral:\n",
            "        canon \"relay\" $ephemeral #ephemeral\n"
        ),
    );
}

#[test]
fn hopon_canon_mismatch() {
    let script = r#"(new $ephemeral (new #can (canon "relay" $ephemeral #ephemeral)))"#;

    let mut output = vec![];
    let mut beautifier = Beautifier::new(&mut output).enable_all_patterns();
    beautifier.beautify(script).unwrap();

    assert_eq!(
        String::from_utf8(output).unwrap(),
        concat!(
            "new $ephemeral:\n",
            "    new #can:\n",
            "        canon \"relay\" $ephemeral #ephemeral\n"
        ),
    );
}

#[test]
fn hopon_stream_mismatch() {
    let script = r#"(new $stream (new #ephemeral (canon "relay" $ephemeral #ephemeral)))"#;

    let mut output = vec![];
    let mut beautifier = Beautifier::new(&mut output).enable_all_patterns();
    beautifier.beautify(script).unwrap();

    assert_eq!(
        String::from_utf8(output).unwrap(),
        concat!(
            "new $stream:\n",
            "    new #ephemeral:\n",
            "        canon \"relay\" $ephemeral #ephemeral\n"
        ),
    );
}

#[test]
fn hopon_nested() {
    let script =
        r#"(new $other (new $ephemeral (new #ephemeral (canon "relay" $ephemeral #ephemeral))) )"#;

    let mut output = vec![];
    let mut beautifier = Beautifier::new(&mut output).enable_all_patterns();
    beautifier.beautify(script).unwrap();

    assert_eq!(
        String::from_utf8(output).unwrap(),
        "new $other:\n    hopon \"relay\"\n",
    );
}

// this is bug that should be eventually fixed: it uses top-level #can
// instead of the nested one which disappeared
//
// the compiler doesn't generate such code, but it can be crafted manually
#[test]
fn hopon_shadowing() {
    let script = r#"(new #can (new $ephemeral (new #can (canon #can.$.[0] $ephemeral #can))) )"#;

    let mut output = vec![];
    let mut beautifier = Beautifier::new(&mut output).enable_all_patterns();
    beautifier.beautify(script).unwrap();

    assert_eq!(
        String::from_utf8(output).unwrap(),
        concat!(
            "new #can:\n",
            "    new $ephemeral:\n",
            "        new #can:\n",
            "            canon #can.$.[0] $ephemeral #can\n"
        ),
    );
}
