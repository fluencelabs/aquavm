/*
 * Copyright 2024 Fluence Labs Limited
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

use air_interpreter_value::{JValue, Map};
// use serde_json::Number;

#[test]
fn test_deserialize_null() {
    let inp = "null";
    let val: JValue = serde_json::from_str(inp).unwrap();
    assert_eq!(val, JValue::Null);
}

#[test]
fn test_deserialize_bool_false() {
    let inp = "false";
    let val: JValue = serde_json::from_str(inp).unwrap();
    assert_eq!(val, JValue::Bool(false));
}

#[test]
fn test_deserialize_bool() {
    let inp = "true";
    let val: JValue = serde_json::from_str(inp).unwrap();
    assert_eq!(val, JValue::Bool(true));
}

#[test]
fn test_deserialize_i64() {
    let inp = "42";
    let val: JValue = serde_json::from_str(inp).unwrap();
    assert_eq!(val, JValue::Number(42.into()));
}

#[test]
fn test_deserialize_i64_2() {
    let inp = "-42";
    let val: JValue = serde_json::from_str(inp).unwrap();
    assert_eq!(val, JValue::Number((-42).into()));
}

// #[test]
// fn test_deserialize_f64() {
//     let inp = "-3140000000000000.0";
//     let val: JValue = serde_json::from_str(inp).unwrap();
//     assert_eq!(val, JValue::Number(Number::from_f64(-3.14e15).unwrap()));
// }

#[test]
fn test_deserialize_string_simple() {
    let inp = r#""simple string""#;
    let val: JValue = serde_json::from_str(inp).unwrap();
    assert_eq!(val, JValue::string("simple string"));
}

#[test]
fn test_deserialize_string_escaping() {
    let inp = r#""simple\" string""#;
    let val: JValue = serde_json::from_str(inp).unwrap();
    assert_eq!(val, JValue::string("simple\" string"));
}

#[test]
fn test_deserialize_array() {
    let inp = r#"[42,8,12]"#;
    let val: JValue = serde_json::from_str(inp).unwrap();
    assert_eq!(val, JValue::array_from_iter(vec![42, 8, 12].into_iter()));
}

#[test]
fn test_deserialize_object() {
    let inp = r#"{"b":18,"a":42}"#;
    let val: JValue = serde_json::from_str(inp).unwrap();

    let mut map = Map::new();
    map.insert("b".into(), 18.into());
    map.insert("a".into(), 42.into());

    let expected = JValue::object(map);
    assert_eq!(val, expected);
}

#[test]
fn test_deserialize_object_ordered() {
    let inp = r#"{"a":18,"b":42}"#;
    let val: JValue = serde_json::from_str(inp).unwrap();
    assert_eq!(val, JValue::object_from_pairs(vec![("b", 42), ("a", 18)]));
}
