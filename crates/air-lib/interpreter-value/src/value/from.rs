use super::JValue;
use crate::{JsonString, Map};
use serde_json::Number;
use std::borrow::Cow;
use std::collections::HashMap;
use std::rc::Rc;
use std::string::String;
use std::vec::Vec;

macro_rules! from_integer {
    ($($ty:ident)*) => {
        $(
            impl From<$ty> for JValue {
                fn from(n: $ty) -> Self {
                    JValue::Number(n.into())
                }
            }
        )*
    };
}

from_integer! {
    i8 i16 i32 i64 isize
    u8 u16 u32 u64 usize
}

impl From<f32> for JValue {
    /// Convert 32-bit floating point number to `Value::Number`, or
    /// `Value::Null` if infinite or NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let f: f32 = 13.37;
    /// let x: Value = f.into();
    /// ```
    fn from(f: f32) -> Self {
        Number::from_f64(f as _).map_or(JValue::Null, JValue::Number)
    }
}

impl From<f64> for JValue {
    /// Convert 64-bit floating point number to `Value::Number`, or
    /// `Value::Null` if infinite or NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let f: f64 = 13.37;
    /// let x: Value = f.into();
    /// ```
    fn from(f: f64) -> Self {
        Number::from_f64(f).map_or(JValue::Null, JValue::Number)
    }
}

impl From<bool> for JValue {
    /// Convert boolean to `Value::Bool`.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let b = false;
    /// let x: Value = b.into();
    /// ```
    fn from(f: bool) -> Self {
        JValue::Bool(f)
    }
}

impl From<String> for JValue {
    /// Convert `String` to `Value::String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let s: String = "lorem".to_string();
    /// let x: Value = s.into();
    /// ```
    fn from(f: String) -> Self {
        JValue::String(f.into())
    }
}

impl From<JsonString> for JValue {
    /// Convert `String` to `Value::String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let s: String = "lorem".to_string();
    /// let x: Value = s.into();
    /// ```
    fn from(f: JsonString) -> Self {
        JValue::String(f)
    }
}

impl From<&str> for JValue {
    /// Convert string slice to `Value::String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let s: &str = "lorem";
    /// let x: Value = s.into();
    /// ```
    fn from(f: &str) -> Self {
        JValue::String(f.into())
    }
}

impl<'a> From<Cow<'a, str>> for JValue {
    /// Convert copy-on-write string to `Value::String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    /// use std::borrow::Cow;
    ///
    /// let s: Cow<str> = Cow::Borrowed("lorem");
    /// let x: Value = s.into();
    /// ```
    ///
    /// ```
    /// use serde_json::Value;
    /// use std::borrow::Cow;
    ///
    /// let s: Cow<str> = Cow::Owned("lorem".to_string());
    /// let x: Value = s.into();
    /// ```
    fn from(f: Cow<'a, str>) -> Self {
        JValue::String(f.into())
    }
}

impl From<Number> for JValue {
    /// Convert `Number` to `Value::Number`.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::{Number, Value};
    ///
    /// let n = Number::from(7);
    /// let x: Value = n.into();
    /// ```
    fn from(f: Number) -> Self {
        JValue::Number(f)
    }
}

impl From<Map<JsonString, JValue>> for JValue {
    /// Convert map (with string keys) to `Value::Object`.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::{Map, Value};
    ///
    /// let mut m = Map::new();
    /// m.insert("Lorem".to_string(), "ipsum".into());
    /// let x: Value = m.into();
    /// ```
    fn from(f: Map<JsonString, JValue>) -> Self {
        JValue::Object(f.into())
    }
}

impl<K: Into<JsonString>, V: Into<JValue>> From<HashMap<K, V>> for JValue {
    /// Convert map (with string keys) to `Value::Object`.
    ///
    /// # Examples
    ///
    /// ```
    /// use air_interpreter_value::JValue;
    /// use std::collections::HashMap;
    ///
    /// let mut m = HashMap::<&str, &str>::new();
    /// m.insert("Lorem", "ipsum");
    /// let x: JValue = m.into();
    /// ```
    fn from(f: HashMap<K, V>) -> Self {
        JValue::object_from_pairs(f)
    }
}

impl<T: Into<JValue>> From<Vec<T>> for JValue {
    /// Convert a `Vec` to `Value::Array`.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let v = vec!["lorem", "ipsum", "dolor"];
    /// let x: Value = v.into();
    /// ```
    fn from(f: Vec<T>) -> Self {
        JValue::Array(f.into_iter().map(Into::into).collect())
    }
}

impl<T: Clone + Into<JValue>> From<&[T]> for JValue {
    /// Convert a slice to `Value::Array`.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let v: &[&str] = &["lorem", "ipsum", "dolor"];
    /// let x: Value = v.into();
    /// ```
    fn from(f: &[T]) -> Self {
        JValue::Array(f.iter().cloned().map(Into::into).collect())
    }
}

impl<T: Into<JValue>> FromIterator<T> for JValue {
    /// Create a `Value::Array` by collecting an iterator of array elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let v = std::iter::repeat(42).take(5);
    /// let x: Value = v.collect();
    /// ```
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let v: Vec<_> = vec!["lorem", "ipsum", "dolor"];
    /// let x: Value = v.into_iter().collect();
    /// ```
    ///
    /// ```
    /// use std::iter::FromIterator;
    /// use serde_json::Value;
    ///
    /// let x: Value = Value::from_iter(vec!["lorem", "ipsum", "dolor"]);
    /// ```
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        JValue::Array(iter.into_iter().map(Into::into).collect())
    }
}

impl<K: Into<JsonString>, V: Into<JValue>> FromIterator<(K, V)> for JValue {
    /// Create a `Value::Object` by collecting an iterator of key-value pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let v: Vec<_> = vec![("lorem", 40), ("ipsum", 2)];
    /// let x: Value = v.into_iter().collect();
    /// ```
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        JValue::Object(Rc::new(
            iter.into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        ))
    }
}

impl From<()> for JValue {
    /// Convert `()` to `Value::Null`.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let u = ();
    /// let x: Value = u.into();
    /// ```
    #[inline]
    fn from((): ()) -> Self {
        JValue::Null
    }
}

impl<T> From<Option<T>> for JValue
where
    T: Into<JValue>,
{
    fn from(opt: Option<T>) -> Self {
        match opt {
            None => JValue::Null,
            Some(value) => Into::into(value),
        }
    }
}

impl From<&serde_json::Value> for JValue {
    fn from(value: &serde_json::Value) -> Self {
        use serde_json::Value;

        match value {
            Value::Null => JValue::Null,
            Value::Bool(b) => JValue::Bool(*b),
            Value::Number(n) => JValue::Number(n.clone()),
            Value::String(s) => JValue::String(s.as_str().into()),
            Value::Array(a) => JValue::Array(a.iter().map(Into::into).collect()),
            Value::Object(o) => {
                let oo = Map::from_iter(o.into_iter().map(|(k, v)| (k.as_str().into(), v.into())));
                JValue::Object(oo.into())
            }
        }
    }
}

impl From<serde_json::Value> for JValue {
    #[inline]
    fn from(value: serde_json::Value) -> Self {
        Self::from(&value)
    }
}
