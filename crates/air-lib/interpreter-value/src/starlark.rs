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

use starlark::typing::Ty;
use starlark::values::dict::AllocDict;
use starlark::values::type_repr::StarlarkTypeRepr;
use starlark::values::AllocFrozenValue;
use starlark::values::AllocValue;
use starlark::values::FrozenValue;
use starlark::values::Value;

use crate::JValue;

impl<'a> StarlarkTypeRepr for &'a JValue {
    fn starlark_type_repr() -> Ty {
        // Any Value.
        Value::starlark_type_repr()
    }
}

impl StarlarkTypeRepr for JValue {
    fn starlark_type_repr() -> Ty {
        // Any Value.
        Value::starlark_type_repr()
    }
}

impl<'v, 'a> AllocValue<'v> for &'a JValue {
    fn alloc_value(self, heap: &'v starlark::values::Heap) -> Value<'v> {
        match self {
            JValue::Null => Value::new_none(),
            JValue::Bool(b) => Value::new_bool(*b),
            JValue::Number(n) => heap.alloc(n),
            JValue::String(s) => heap.alloc(&**s),
            JValue::Array(a) => heap.alloc(&**a),
            JValue::Object(m) => {
                // Starlark cannot handle Rc<T>, get rid of it
                let compatible_iter = m.iter().map(|(k, v)| (&**k, v));
                heap.alloc(AllocDict(compatible_iter))
            }
        }
    }
}

impl AllocFrozenValue for &JValue {
    fn alloc_frozen_value(
        self,
        heap: &starlark::values::FrozenHeap,
    ) -> starlark::values::FrozenValue {
        match self {
            JValue::Null => FrozenValue::new_none(),
            JValue::Bool(b) => FrozenValue::new_bool(*b),
            JValue::Number(n) => heap.alloc(n),
            JValue::String(s) => heap.alloc(&**s),
            JValue::Array(a) => heap.alloc(&**a),
            JValue::Object(m) => {
                // Starlark cannot handle Rc<T>, get rid of it
                let compatible_iter = m.iter().map(|(k, v)| (&**k, v));
                heap.alloc(AllocDict(compatible_iter))
            }
        }
    }
}

impl<'heap> TryInto<JValue> for starlark::values::Value<'heap> {
    type Error = starlark::Error;

    fn try_into(self) -> Result<JValue, Self::Error> {
        // TODO fix double allocation:
        // unfortunately, Starlark `Value` is opaque and cannot be
        // converted to JValue directly; but it implements serde::Serialize through
        // `erased_serde` crate
        //
        // we might try to implement our version of `serde_json::to_value`
        //
        // first allocation: allocate a serde_json::Value
        let value = self.to_json_value()?;
        // second allocation: and then allocate JValue
        Ok(value.into())
    }
}
