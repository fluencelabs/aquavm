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
 * This file is based on serde_json crate by Erick Tryzelaar and David Tolnay
 * licensed under conditions of MIT License and Apache License, Version 2.0.
 */

use super::JValue;
use core::fmt::{self, Display};
use core::ops;
use std::string::String;

/// A type that can be used to index into a `air_interpreter_value::JValue`.
///
/// The [`get`] and [`get_mut`] methods of `JValue` accept any type that
/// implements `Index`, as does the [square-bracket indexing operator]. This
/// trait is implemented for strings which are used as the index into a JSON
/// map, and for `usize` which is used as the index into a JSON array.
///
/// [`get`]: ../enum.JValue.html#method.get
/// [`get_mut`]: ../enum.JValue.html#method.get_mut
/// [square-bracket indexing operator]: ../enum.JValue.html#impl-Index%3CI%3E
///
/// This trait is sealed and cannot be implemented for types outside of
/// `air_interpreter_value`.
pub trait Index: private::Sealed {
    /// Return None if the key is not already in the array or object.
    #[doc(hidden)]
    fn index_into<'v>(&self, v: &'v JValue) -> Option<&'v JValue>;
}

impl Index for usize {
    fn index_into<'v>(&self, v: &'v JValue) -> Option<&'v JValue> {
        match v {
            JValue::Array(vec) => vec.get(*self),
            _ => None,
        }
    }
}

impl Index for str {
    fn index_into<'v>(&self, v: &'v JValue) -> Option<&'v JValue> {
        match v {
            JValue::Object(map) => map.get(self),
            _ => None,
        }
    }
}

impl Index for String {
    fn index_into<'v>(&self, v: &'v JValue) -> Option<&'v JValue> {
        self[..].index_into(v)
    }
}

impl<T> Index for &T
where
    T: ?Sized + Index,
{
    fn index_into<'v>(&self, v: &'v JValue) -> Option<&'v JValue> {
        (**self).index_into(v)
    }
}

// Prevent users from implementing the Index trait.
mod private {
    pub trait Sealed {}
    impl Sealed for usize {}
    impl Sealed for str {}
    impl Sealed for std::string::String {}
    impl<'a, T> Sealed for &'a T where T: ?Sized + Sealed {}
}

/// Used in panic messages.
struct Type<'a>(&'a JValue);

impl<'a> Display for Type<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            JValue::Null => formatter.write_str("null"),
            JValue::Bool(_) => formatter.write_str("boolean"),
            JValue::Number(_) => formatter.write_str("number"),
            JValue::String(_) => formatter.write_str("string"),
            JValue::Array(_) => formatter.write_str("array"),
            JValue::Object(_) => formatter.write_str("object"),
        }
    }
}

// The usual semantics of Index is to panic on invalid indexing.
//
// That said, the usual semantics are for things like Vec and BTreeMap which
// have different use cases than JValue. If you are working with a Vec, you know
// that you are working with a Vec and you can get the len of the Vec and make
// sure your indices are within bounds. The JValue use cases are more
// loosey-goosey. You got some JSON from an endpoint and you want to pull values
// out of it. Outside of this Index impl, you already have the option of using
// value.as_array() and working with the Vec directly, or matching on
// JValue::Array and getting the Vec directly. The Index impl means you can skip
// that and index directly into the thing using a concise syntax. You don't have
// to check the type, you don't have to check the len, it is all about what you
// expect the JValue to look like.
//
// Basically the use cases that would be well served by panicking here are
// better served by using one of the other approaches: get and get_mut,
// as_array, or match. The value of this impl is that it adds a way of working
// with JValue that is not well served by the existing approaches: concise and
// careless and sometimes that is exactly what you want.
impl<I> ops::Index<I> for JValue
where
    I: Index,
{
    type Output = JValue;

    /// Index into a `air_interpreter_value::JValue` using the syntax `value[0]` or
    /// `value["k"]`.
    ///
    /// Returns `JValue::Null` if the type of `self` does not match the type of
    /// the index, for example if the index is a string and `self` is an array
    /// or a number. Also returns `JValue::Null` if the given key does not exist
    /// in the map or the given index is not within the bounds of the array.
    ///
    /// For retrieving deeply nested values, you should have a look at the
    /// `JValue::pointer` method.
    fn index(&self, index: I) -> &JValue {
        const NULL: JValue = JValue::Null;
        index.index_into(self).unwrap_or(&NULL)
    }
}
