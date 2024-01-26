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
