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

/*
* This file is based on serde_json crate licensed under conditions
* of MIT License.
* Copyright (C) 2023 by Erick Tryzelaar and David Tolnay

* Permission is hereby granted, free of charge, to any
* person obtaining a copy of this software and associated
* documentation files (the "Software"), to deal in the
* Software without restriction, including without
* limitation the rights to use, copy, modify, merge,
* publish, distribute, sublicense, and/or sell copies of
* the Software, and to permit persons to whom the Software
* is furnished to do so, subject to the following
* conditions:
*
* The above copyright notice and this permission notice
* shall be included in all copies or substantial portions
* of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
* ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
* TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
* PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
* SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
* CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
* OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
* IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
* DEALINGS IN THE SOFTWARE.
*/

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
    /// Convert 32-bit floating point number to `JValue::Number`, or
    /// `JValue::Null` if infinite or NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// use air_interpreter_value::JValue;
    ///
    /// let f: f32 = 13.37;
    /// let x: JValue = f.into();
    /// ```
    fn from(f: f32) -> Self {
        Number::from_f64(f as _).map_or(JValue::Null, JValue::Number)
    }
}

impl From<f64> for JValue {
    /// Convert 64-bit floating point number to `JValue::Number`, or
    /// `JValue::Null` if infinite or NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// use air_interpreter_value::JValue;
    ///
    /// let f: f64 = 13.37;
    /// let x: JValue = f.into();
    /// ```
    fn from(f: f64) -> Self {
        Number::from_f64(f).map_or(JValue::Null, JValue::Number)
    }
}

impl From<bool> for JValue {
    /// Convert boolean to `JValue::Bool`.
    ///
    /// # Examples
    ///
    /// ```
    /// use air_interpreter_value::JValue;
    ///
    /// let b = false;
    /// let x: JValue = b.into();
    /// ```
    fn from(f: bool) -> Self {
        JValue::Bool(f)
    }
}

impl From<String> for JValue {
    /// Convert `String` to `JValue::String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use air_interpreter_value::JValue;
    ///
    /// let s: String = "lorem".to_string();
    /// let x: JValue = s.into();
    /// ```
    fn from(f: String) -> Self {
        JValue::String(f.into())
    }
}

impl From<JsonString> for JValue {
    /// Convert `JsonString` to `JValue::String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use air_interpreter_value::JValue;
    ///
    /// let s: String = "lorem".to_string();
    /// let x: JValue = s.into();
    /// ```
    fn from(f: JsonString) -> Self {
        JValue::String(f)
    }
}

impl From<&str> for JValue {
    /// Convert string slice to `JValue::String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use air_interpreter_value::JValue;
    ///
    /// let s: &str = "lorem";
    /// let x: JValue = s.into();
    /// ```
    fn from(f: &str) -> Self {
        JValue::String(f.into())
    }
}

impl<'a> From<Cow<'a, str>> for JValue {
    /// Convert copy-on-write string to `JValue::String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use air_interpreter_value::JValue;
    /// use std::borrow::Cow;
    ///
    /// let s: Cow<str> = Cow::Borrowed("lorem");
    /// let x: JValue = s.into();
    /// ```
    ///
    /// ```
    /// use air_interpreter_value::JValue;
    /// use std::borrow::Cow;
    ///
    /// let s: Cow<str> = Cow::Owned("lorem".to_string());
    /// let x: JValue = s.into();
    /// ```
    fn from(f: Cow<'a, str>) -> Self {
        JValue::String(f.into())
    }
}

impl From<Number> for JValue {
    /// Convert `serde_json::Number` to `JValue::Number`.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Number;
    /// use air_interpreter_value::JValue;
    ///
    /// let n = Number::from(7);
    /// let x: JValue = n.into();
    /// ```
    fn from(f: Number) -> Self {
        JValue::Number(f)
    }
}

impl From<Map<JsonString, JValue>> for JValue {
    /// Convert map (with string keys) to `JValue::Object`.
    ///
    /// # Examples
    ///
    /// ```
    /// use air_interpreter_value::{Map, JValue, JsonString};
    ///
    /// let mut m = Map::<JsonString, JValue>::new();
    /// m.insert("Lorem".into(), "ipsum".into());
    /// let x: JValue = m.into();
    /// ```
    fn from(f: Map<JsonString, JValue>) -> Self {
        JValue::Object(f.into())
    }
}

impl<K: Into<JsonString>, V: Into<JValue>> From<HashMap<K, V>> for JValue {
    /// Convert map (with string keys) to `JValue::Object`.
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
    /// Convert a `Vec` to `JValue::Array`.
    ///
    /// # Examples
    ///
    /// ```
    /// use air_interpreter_value::JValue;
    ///
    /// let v = vec!["lorem", "ipsum", "dolor"];
    /// let x: JValue = v.into();
    /// ```
    fn from(f: Vec<T>) -> Self {
        JValue::Array(f.into_iter().map(Into::into).collect())
    }
}

impl<T: Clone + Into<JValue>> From<&[T]> for JValue {
    /// Convert a slice to `JValue::Array`.
    ///
    /// # Examples
    ///
    /// ```
    /// use air_interpreter_value::JValue;
    ///
    /// let v: &[&str] = &["lorem", "ipsum", "dolor"];
    /// let x: JValue = v.into();
    /// ```
    fn from(f: &[T]) -> Self {
        JValue::Array(f.iter().cloned().map(Into::into).collect())
    }
}

impl<T: Into<JValue>> FromIterator<T> for JValue {
    /// Create a `JValue::Array` by collecting an iterator of array elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use air_interpreter_value::JValue;
    ///
    /// let v = std::iter::repeat(42).take(5);
    /// let x: JValue = v.collect();
    /// ```
    ///
    /// ```
    /// use air_interpreter_value::JValue;
    ///
    /// let v: Vec<_> = vec!["lorem", "ipsum", "dolor"];
    /// let x: JValue = v.into_iter().collect();
    /// ```
    ///
    /// ```
    /// use std::iter::FromIterator;
    /// use air_interpreter_value::JValue;
    ///
    /// let x: JValue = JValue::from_iter(vec!["lorem", "ipsum", "dolor"]);
    /// ```
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        JValue::Array(iter.into_iter().map(Into::into).collect())
    }
}

impl<K: Into<JsonString>, V: Into<JValue>> FromIterator<(K, V)> for JValue {
    /// Create a `JValue::Object` by collecting an iterator of key-value pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use air_interpreter_value::JValue;
    ///
    /// let v: Vec<_> = vec![("lorem", 40), ("ipsum", 2)];
    /// let x: JValue = v.into_iter().collect();
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
    /// Convert `()` to `JValue::Null`.
    ///
    /// # Examples
    ///
    /// ```
    /// use air_interpreter_value::JValue;
    ///
    /// let u = ();
    /// let x: JValue = u.into();
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
