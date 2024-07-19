use polyplets::SecurityTetraplet;
use starlark::typing::Ty;
use starlark::values::type_repr::StarlarkTypeRepr;
use starlark::values::{
    starlark_value, AllocValue, FrozenHeap, FrozenStringValue, Heap, StarlarkValue, Value,
    ValueLike,
};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Hash,
    ::allocative::Allocative,
    ::serde::Serialize,
    ::starlark::any::ProvidesStaticType,
)]
pub(crate) struct StarlarkSecurityTetraplet {
    /// Id of a peer where corresponding value was set.
    peer_pk: FrozenStringValue,

    /// Id of a service that set corresponding value.
    service_id: FrozenStringValue,

    /// Name of a function that returned corresponding value.
    function_name: FrozenStringValue,

    /// Value was produced by applying this `lens` to the output from `call_service`.
    lens: FrozenStringValue,
}

impl StarlarkSecurityTetraplet {
    pub(crate) fn new(source: &SecurityTetraplet, heap: &FrozenHeap) -> Self {
        // TODO peer_pk, service_id and function_name may be repeated several times
        //      and worth to be cached.
        //
        // but lens is generally unique, except shortest strings that are not worth caching.
        //
        // N.B. Starlark has pub(crate) fn alloc_string_intern
        let peer_pk = heap.alloc_str(&source.peer_pk);
        let service_id = heap.alloc_str(&source.service_id);
        let function_name = heap.alloc_str(&source.function_name);
        let lens = heap.alloc_str(&source.lens);

        StarlarkSecurityTetraplet {
            peer_pk,
            service_id,
            function_name,
            lens,
        }
    }
}

#[starlark_value(type = "SecurityTetraplet", UnpackValue, StarlarkTypeRepr)]
impl<'v> StarlarkValue<'v> for StarlarkSecurityTetraplet {
    type Canonical = Self;

    #[inline]
    fn equals(&self, other: Value<'v>) -> starlark::Result<bool> {
        match other.downcast_ref::<StarlarkSecurityTetraplet>() {
            Some(other_tetraplet) => Ok(self == other_tetraplet),
            None => Ok(false),
        }
    }

    // field access
    fn get_attr(&self, attribute: &str, _heap: &'v Heap) -> Option<Value<'v>> {
        match attribute {
            // TODO check that to_value doesn't clone full string content (write or find test that checks heap size)
            // TODO heap is not used, but a Value is returned: it probably means that
            //      the shared string is simply wrapped by a Value
            "peer_pk" => Some(self.peer_pk.to_value()),
            "service_id" => Some(self.service_id.to_value()),
            "function_name" => Some(self.function_name.to_value()),
            "lens" => Some(self.lens.to_value()),
            _ => None,
        }
    }

    // field type
    fn attr_ty(name: &str) -> Option<Ty> {
        match name {
            "peer_pk" | "service_id" | "function_name" | "lens" => {
                Some(FrozenStringValue::starlark_type_repr())
            }
            _ => None,
        }
    }
}

impl<'v> AllocValue<'v> for StarlarkSecurityTetraplet {
    #[inline]
    fn alloc_value(self, heap: &'v Heap) -> Value<'v> {
        heap.alloc_simple(self)
    }
}

use std::fmt;

impl fmt::Display for StarlarkSecurityTetraplet {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_name = Self::TYPE;
        write!(
            f,
            "{}(peer_pk={:?}, service_id={:?}, function_name={:?}, lens={:?})",
            type_name,
            self.peer_pk.as_str(),
            self.service_id.as_str(),
            self.function_name.as_str(),
            self.lens.as_str(),
        )
    }
}
