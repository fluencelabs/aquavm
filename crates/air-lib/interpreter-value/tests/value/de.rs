use air_interpreter_value::JValue;
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
    let inp = r#"{"a":18,"b":42}"#;
    let val: JValue = serde_json::from_str(inp).unwrap();
    let expected = JValue::object(maplit::btreemap! {
        "b".into() => 42.into(),
        "a".into() => 18.into(),
    });
    assert_eq!(val, expected);
}

#[test]
fn test_deserialize_object_ordered() {
    let inp = r#"{"a":18,"b":42}"#;
    let val: JValue = serde_json::from_str(inp).unwrap();
    assert_eq!(val, JValue::object_from_pairs(vec![("b", 42), ("a", 18)]));
}
