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

use air_interpreter_value::{JValue, Map};
use serde_json::Number;

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

#[test]
fn test_deserialize_f64() {
    let inp = "-3140000000000000.0";
    let val: JValue = serde_json::from_str(inp).unwrap();
    assert_eq!(val, JValue::Number(Number::from_f64(-3.14e15).unwrap()));
}

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
