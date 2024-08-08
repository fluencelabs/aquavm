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

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use air_interpreter_value::JValue;
    use serde_json::json;

    use crate::execute;

    use super::*;

    #[test]
    fn test_tetraplet_peer_pk() {
        let tetraplet =
            SecurityTetraplet::new("my_peer", "service_id", "function_name", ".$.lens").into();
        let value = json!(42).into();
        let script = "get_tetraplet(0)[0].peer_pk";

        let res = execute(script, vec![(value, vec![tetraplet])])
            .unwrap()
            .unwrap();
        assert_eq!(res, "my_peer");
    }

    #[test]
    fn test_tetraplet_service_id() {
        let tetraplet =
            SecurityTetraplet::new("my_peer", "my_service", "my_func", ".$.lens").into();
        let value = json!(42).into();
        let script = "get_tetraplet(0)[0].service_id";

        let res = execute(script, vec![(value, vec![tetraplet])])
            .unwrap()
            .unwrap();
        assert_eq!(res, "my_service");
    }

    #[test]
    fn test_tetraplet_function_name() {
        let tetraplet =
            SecurityTetraplet::new("my_peer", "my_service", "my_func", ".$.lens").into();
        let value = json!(42).into();
        let script = "get_tetraplet(0)[0].function_name";

        let res = execute(script, vec![(value, vec![tetraplet])])
            .unwrap()
            .unwrap();
        assert_eq!(res, "my_func");
    }

    #[test]
    fn test_tetraplet_lens() {
        let tetraplet =
            SecurityTetraplet::new("my_peer", "my_service", "my_func", ".$.lens").into();
        let value = json!(42).into();
        let script = "get_tetraplet(0)[0].lens";

        let res = execute(script, vec![(value, vec![tetraplet])])
            .unwrap()
            .unwrap();
        assert_eq!(res, ".$.lens");
    }

    #[test]
    fn test_tetraplet_equals_copy() {
        let tetraplet =
            SecurityTetraplet::new("my_peer", "my_service", "my_func", ".$.lens").into();
        let value = json!(42).into();
        let script = "get_tetraplet(0) == get_tetraplet(0)";

        let res = execute(script, vec![(value, vec![tetraplet])])
            .unwrap()
            .unwrap();
        assert_eq!(res, true);
    }

    #[test]
    fn test_tetraplet_equals_self() {
        let tetraplet =
            SecurityTetraplet::new("my_peer", "my_service", "my_func", ".$.lens").into();
        let value = json!(42).into();
        let script = "tet = get_tetraplet(0)
tet == tet";

        let res = execute(script, vec![(value, vec![tetraplet])])
            .unwrap()
            .unwrap();
        assert_eq!(res, true);
    }

    #[test]
    fn test_tetraplet_same() {
        let tetraplet1 = SecurityTetraplet::new("my_peer", "my_service", "my_func", ".$.lens");
        let tetraplet2 = tetraplet1.clone();
        let value: JValue = json!(42).into();
        let script = "tet = get_tetraplet(0)
tet == tet";

        let res = execute(
            script,
            vec![
                (value.clone(), vec![tetraplet1.into()]),
                (value.clone(), vec![tetraplet2.into()]),
            ],
        )
        .unwrap()
        .unwrap();
        assert_eq!(res, true);
    }

    #[test]
    fn test_tetraplet_different() {
        // TODO it worth variating fields
        let tetraplet1 = Rc::new(SecurityTetraplet::new(
            "my_peer1",
            "my_service1",
            "my_func1",
            ".$.lens1",
        ));
        let tetraplet2 = Rc::new(SecurityTetraplet::new(
            "my_peer2",
            "my_service2",
            "my_func2",
            ".$.lens2",
        ));
        let value: JValue = json!(42).into();
        let script = "tet1 = get_tetraplet(0)
tet2 = get_tetraplet(1)
tet1 == tet2";

        let res = execute(
            script,
            vec![
                (value.clone(), vec![tetraplet1]),
                (value.clone(), vec![tetraplet2]),
            ],
        )
        .unwrap()
        .unwrap();
        assert_eq!(res, false);
    }
}
