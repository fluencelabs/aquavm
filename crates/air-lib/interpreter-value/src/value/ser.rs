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

use crate::value::JValue;
use core::result;
use serde::ser::Serialize;

impl Serialize for JValue {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        match self {
            JValue::Null => serializer.serialize_unit(),
            JValue::Bool(b) => serializer.serialize_bool(*b),
            JValue::Number(n) => n.serialize(serializer),
            JValue::String(s) => serializer.serialize_str(s),
            JValue::Array(v) => v.serialize(serializer),
            JValue::Object(m) => {
                use serde::ser::SerializeMap;
                let mut map = tri!(serializer.serialize_map(Some(m.len())));
                for (k, v) in &**m {
                    tri!(map.serialize_entry(k, v));
                }
                map.end()
            }
        }
    }
}
