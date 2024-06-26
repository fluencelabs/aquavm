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

//! The JValue enum, a loosely typed way of representing any valid JSON value.
//!

mod de;
mod from;
mod index;
mod partial_eq;
mod ser;

use core::fmt::{self, Debug, Display};
use core::mem;
use core::str;
use std::io;
use std::rc::Rc;

pub use self::index::Index;
use crate::JsonString;
pub use crate::Map;
pub use serde_json::Number;

/// Represents any valid JSON value with a cheap to clone Rc-based representation.
#[derive(Clone, Eq, PartialEq)]
pub enum JValue {
    /// Represents a JSON null value.
    Null,

    /// Represents a JSON boolean.
    Bool(bool),

    /// Represents a JSON number, whether integer or floating point.
    Number(Number),

    /// Represents a JSON string.
    String(JsonString),

    /// Represents a JSON array.
    Array(Rc<[JValue]>),

    /// Represents a JSON object.
    ///
    /// By default the map is backed by a BTreeMap. Enable the `preserve_order`
    /// feature of serde_json to use IndexMap instead, which preserves
    /// entries in the order they are inserted into the map. In particular, this
    /// allows JSON data to be deserialized into a JValue and serialized to a
    /// string while retaining the order of map keys in the input.
    Object(Rc<Map<JsonString, JValue>>),
}

impl Debug for JValue {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JValue::Null => formatter.write_str("Null"),
            JValue::Bool(boolean) => write!(formatter, "Bool({})", boolean),
            JValue::Number(number) => Debug::fmt(number, formatter),
            JValue::String(string) => write!(formatter, "String({:?})", string),
            JValue::Array(vec) => {
                tri!(formatter.write_str("Array "));
                Debug::fmt(vec, formatter)
            }
            JValue::Object(map) => {
                tri!(formatter.write_str("Object "));
                Debug::fmt(&**map, formatter)
            }
        }
    }
}

impl Display for JValue {
    /// Display a JSON value as a string.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct WriterFormatter<'a, 'b: 'a> {
            inner: &'a mut fmt::Formatter<'b>,
        }

        impl<'a, 'b> io::Write for WriterFormatter<'a, 'b> {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                // Safety: the serializer below only emits valid utf8 when using
                // the default formatter.
                let s = unsafe { str::from_utf8_unchecked(buf) };
                tri!(self.inner.write_str(s).map_err(io_error));
                Ok(buf.len())
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        fn io_error(_: fmt::Error) -> io::Error {
            // Error value does not matter because Display impl just maps it
            // back to fmt::Error.
            io::Error::new(io::ErrorKind::Other, "fmt error")
        }

        let alternate = f.alternate();
        let mut wr = WriterFormatter { inner: f };
        if alternate {
            // {:#}
            serde_json::ser::to_writer_pretty(&mut wr, self).map_err(|_| fmt::Error)
        } else {
            // {}
            serde_json::ser::to_writer(&mut wr, self).map_err(|_| fmt::Error)
        }
    }
}

fn parse_index(s: &str) -> Option<usize> {
    if s.starts_with('+') || (s.starts_with('0') && s.len() != 1) {
        return None;
    }
    s.parse().ok()
}

impl JValue {
    #[inline]
    pub fn string(s: impl Into<Rc<str>>) -> Self {
        Self::String(s.into())
    }

    #[inline]
    pub fn array(vec: impl Into<Rc<[JValue]>>) -> Self {
        Self::Array(vec.into())
    }

    pub fn array_from_iter(into_iter: impl IntoIterator<Item = impl Into<JValue>>) -> Self {
        Self::Array(into_iter.into_iter().map(Into::into).collect())
    }

    pub fn object(map: impl Into<Map<JsonString, JValue>>) -> Self {
        Self::Object(Rc::new(map.into()))
    }

    pub fn object_from_pairs(
        into_iter: impl IntoIterator<Item = (impl Into<JsonString>, impl Into<JValue>)>,
    ) -> Self {
        Self::Object(Rc::new(
            into_iter
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        ))
    }

    /// Index into a JSON array or map. A string index can be used to access a
    /// value in a map, and a usize index can be used to access an element of an
    /// array.
    ///
    /// Returns `None` if the type of `self` does not match the type of the
    /// index, for example if the index is a string and `self` is an array or a
    /// number. Also returns `None` if the given key does not exist in the map
    /// or the given index is not within the bounds of the array.
    ///
    /// Square brackets can also be used to index into a value in a more concise
    /// way. This returns `JValue::Null` in cases where `get` would have returned
    /// `None`.
    pub fn get<I: Index>(&self, index: I) -> Option<&JValue> {
        index.index_into(self)
    }

    /// Returns true if the `JValue` is an Object. Returns false otherwise.
    ///
    /// For any JValue on which `is_object` returns true, `as_object` and
    /// `as_object_mut` are guaranteed to return the map representation of the
    /// object.
    #[inline]
    pub fn is_object(&self) -> bool {
        self.as_object().is_some()
    }

    /// If the `JValue` is an Object, returns the associated Map. Returns None
    /// otherwise.
    #[inline]
    pub fn as_object(&self) -> Option<&Map<JsonString, JValue>> {
        match self {
            JValue::Object(map) => Some(map),
            _ => None,
        }
    }

    /// Returns true if the `JValue` is an Array. Returns false otherwise.
    ///
    /// For any JValue on which `is_array` returns true, `as_array` and
    /// `as_array_mut` are guaranteed to return the vector representing the
    /// array.
    #[inline]
    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }

    /// If the `JValue` is an Array, returns the associated vector. Returns None
    /// otherwise.
    #[inline]
    pub fn as_array(&self) -> Option<&[JValue]> {
        match self {
            JValue::Array(array) => Some(array),
            _ => None,
        }
    }

    /// Returns true if the `JValue` is a String. Returns false otherwise.
    ///
    /// For any JValue on which `is_string` returns true, `as_str` is guaranteed
    /// to return the string slice.
    #[inline]
    pub fn is_string(&self) -> bool {
        self.as_str().is_some()
    }

    /// If the `JValue` is a string, returns the associated str. Returns None
    /// otherwise.
    #[inline]
    pub fn as_str(&self) -> Option<&JsonString> {
        match self {
            JValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns true if the `JValue` is a Number. Returns false otherwise.
    #[inline]
    pub fn is_number(&self) -> bool {
        matches!(self, JValue::Number(_))
    }

    /// If the `JValue` is a Number, returns the associated [`Number`]. Returns
    /// None otherwise.
    #[inline]
    pub fn as_number(&self) -> Option<&Number> {
        match self {
            JValue::Number(number) => Some(number),
            _ => None,
        }
    }

    /// Returns true if the `JValue` is an integer between `i64::MIN` and
    /// `i64::MAX`.
    ///
    /// For any JValue on which `is_i64` returns true, `as_i64` is guaranteed to
    /// return the integer value.
    #[inline]
    pub fn is_i64(&self) -> bool {
        match self {
            JValue::Number(n) => n.is_i64(),
            _ => false,
        }
    }

    /// Returns true if the `JValue` is an integer between zero and `u64::MAX`.
    ///
    /// For any JValue on which `is_u64` returns true, `as_u64` is guaranteed to
    /// return the integer value.
    #[inline]
    pub fn is_u64(&self) -> bool {
        match self {
            JValue::Number(n) => n.is_u64(),
            _ => false,
        }
    }

    /// Returns true if the `JValue` is a number that can be represented by f64.
    ///
    /// For any JValue on which `is_f64` returns true, `as_f64` is guaranteed to
    /// return the floating point value.
    ///
    /// Currently this function returns true if and only if both `is_i64` and
    /// `is_u64` return false but this is not a guarantee in the future.
    #[inline]
    pub fn is_f64(&self) -> bool {
        match self {
            JValue::Number(n) => n.is_f64(),
            _ => false,
        }
    }

    /// If the `JValue` is an integer, represent it as i64 if possible. Returns
    /// None otherwise.
    #[inline]
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            JValue::Number(n) => n.as_i64(),
            _ => None,
        }
    }

    /// If the `JValue` is an integer, represent it as u64 if possible. Returns
    /// None otherwise.
    #[inline]
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            JValue::Number(n) => n.as_u64(),
            _ => None,
        }
    }

    /// If the `JValue` is a number, represent it as f64 if possible. Returns
    /// None otherwise.
    #[inline]
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            JValue::Number(n) => n.as_f64(),
            _ => None,
        }
    }

    /// Returns true if the `JValue` is a Boolean. Returns false otherwise.
    ///
    /// For any JValue on which `is_boolean` returns true, `as_bool` is
    /// guaranteed to return the boolean value.
    #[inline]
    pub fn is_boolean(&self) -> bool {
        self.as_bool().is_some()
    }

    /// If the `JValue` is a Boolean, returns the associated bool. Returns None
    /// otherwise.
    #[inline]
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            JValue::Bool(b) => Some(b),
            _ => None,
        }
    }

    /// Returns true if the `JValue` is a Null. Returns false otherwise.
    ///
    /// For any JValue on which `is_null` returns true, `as_null` is guaranteed
    /// to return `Some(())`.
    #[inline]
    pub fn is_null(&self) -> bool {
        self.as_null().is_some()
    }

    /// If the `JValue` is a Null, returns (). Returns None otherwise.
    #[inline]
    pub fn as_null(&self) -> Option<()> {
        match *self {
            JValue::Null => Some(()),
            _ => None,
        }
    }

    /// Looks up a value by a JSON Pointer.
    ///
    /// JSON Pointer defines a string syntax for identifying a specific value
    /// within a JavaScript Object Notation (JSON) document.
    ///
    /// A Pointer is a Unicode string with the reference tokens separated by `/`.
    /// Inside tokens `/` is replaced by `~1` and `~` is replaced by `~0`. The
    /// addressed value is returned and if there is no such value `None` is
    /// returned.
    ///
    /// For more information read [RFC6901](https://tools.ietf.org/html/rfc6901).
    pub fn pointer(&self, pointer: &str) -> Option<&JValue> {
        if pointer.is_empty() {
            return Some(self);
        }
        if !pointer.starts_with('/') {
            return None;
        }
        pointer
            .split('/')
            .skip(1)
            .map(|x| x.replace("~1", "/").replace("~0", "~"))
            .try_fold(self, |target, token| match target {
                JValue::Object(map) => map.get(token.as_str()),
                JValue::Array(list) => parse_index(&token).and_then(|x| list.get(x)),
                _ => None,
            })
    }

    /// Takes the value out of the `JValue`, leaving a `Null` in its place.
    #[inline]
    pub fn take(&mut self) -> JValue {
        mem::replace(self, JValue::Null)
    }
}

/// The default value is `JValue::Null`.
impl Default for JValue {
    #[inline]
    fn default() -> JValue {
        JValue::Null
    }
}
