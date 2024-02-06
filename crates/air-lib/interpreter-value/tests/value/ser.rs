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

use air_interpreter_value::JValue;
// use serde_json::Number;

#[test]
fn test_serialize_null() {
    let val = JValue::Null;
    let res = serde_json::to_string(&val).unwrap();
    assert_eq!(res, "null");
}

#[test]
fn test_serialize_bool_false() {
    let val = JValue::Bool(false);
    let res = serde_json::to_string(&val).unwrap();
    assert_eq!(res, "false");
}

#[test]
fn test_serialize_bool() {
    let val = JValue::Bool(true);
    let res = serde_json::to_string(&val).unwrap();
    assert_eq!(res, "true");
}

#[test]
fn test_serialize_i64() {
    let val = JValue::Number(42.into());
    let res = serde_json::to_string(&val).unwrap();
    assert_eq!(res, "42");
}

#[test]
fn test_serialize_i64_2() {
    let val = JValue::Number((-42).into());
    let res = serde_json::to_string(&val).unwrap();
    assert_eq!(res, "-42");
}

// #[test]
// fn test_serialize_f64() {
//     let val = JValue::Number(Number::from_f64(-3.14e15).unwrap());
//     let res = serde_json::to_string(&val).unwrap();
//     assert_eq!(res, "-3140000000000000.0");
// }

#[test]
fn test_serialize_string_simple() {
    let val = JValue::string("simple string");
    let res = serde_json::to_string(&val).unwrap();
    assert_eq!(res, r#""simple string""#);
}

#[test]
fn test_serialize_string_escaping() {
    let val = JValue::string("simple\" string");
    let res = serde_json::to_string(&val).unwrap();
    assert_eq!(res, r#""simple\" string""#);
}

#[test]
fn test_serialize_array() {
    let val = JValue::array_from_iter(vec![42, 8, 12].into_iter());
    let res = serde_json::to_string(&val).unwrap();
    assert_eq!(res, r#"[42,8,12]"#);
}

#[test]
#[cfg(not(feature = "preserve_order"))]
fn test_serialize_object() {
    let val = JValue::object(maplit::btreemap! {
        "b".into() => 42.into(),
        "a".into() => 18.into(),
    });
    let res = serde_json::to_string(&val).unwrap();
    assert_eq!(res, r#"{"a":18,"b":42}"#);
}

#[test]
#[cfg(feature = "preserve_order")]
fn test_serialize_object() {
    use air_interpreter_value::Map;

    let mut map = Map::new();
    map.insert("b".into(), 18.into());
    map.insert("a".into(), 42.into());

    let val = JValue::object(map);
    let res = serde_json::to_string(&val).unwrap();
    assert_eq!(res, r#"{"b":18,"a":42}"#);
}

#[test]
#[cfg(not(feature = "preserve_order"))]
fn test_serialize_object_ordered() {
    let val = JValue::object_from_pairs(vec![("b", 42), ("a", 18)]);
    let res = serde_json::to_string(&val).unwrap();
    assert_eq!(res, r#"{"a":18,"b":42}"#);
}

#[test]
#[cfg(feature = "preserve_order")]
fn test_serialize_object_ordered() {
    let val = JValue::object_from_pairs(vec![("b", 42), ("a", 18)]);
    let res = serde_json::to_string(&val).unwrap();
    assert_eq!(res, r#"{"b":42,"a":18}"#);
}
