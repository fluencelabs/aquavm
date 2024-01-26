use super::JValue;
use std::string::String;

fn eq_i64(value: &JValue, other: i64) -> bool {
    value.as_i64().map_or(false, |i| i == other)
}

fn eq_u64(value: &JValue, other: u64) -> bool {
    value.as_u64().map_or(false, |i| i == other)
}

fn eq_f32(value: &JValue, other: f32) -> bool {
    match value {
        // NB: is not same as the original version
        JValue::Number(n) => n.as_f64().map_or(false, |i| i == other as f64),
        _ => false,
    }
}

fn eq_f64(value: &JValue, other: f64) -> bool {
    value.as_f64().map_or(false, |i| i == other)
}

fn eq_bool(value: &JValue, other: bool) -> bool {
    value.as_bool().map_or(false, |i| i == other)
}

fn eq_str(value: &JValue, other: &str) -> bool {
    value.as_str().map_or(false, |i| &**i == other)
}

impl PartialEq<str> for JValue {
    fn eq(&self, other: &str) -> bool {
        eq_str(self, other)
    }
}

impl PartialEq<&str> for JValue {
    fn eq(&self, other: &&str) -> bool {
        eq_str(self, other)
    }
}

impl PartialEq<JValue> for str {
    fn eq(&self, other: &JValue) -> bool {
        eq_str(other, self)
    }
}

impl PartialEq<JValue> for &str {
    fn eq(&self, other: &JValue) -> bool {
        eq_str(other, self)
    }
}

impl PartialEq<String> for JValue {
    fn eq(&self, other: &String) -> bool {
        eq_str(self, other.as_str())
    }
}

impl PartialEq<JValue> for String {
    fn eq(&self, other: &JValue) -> bool {
        eq_str(other, self.as_str())
    }
}

macro_rules! partialeq_numeric {
    ($($eq:ident [$($ty:ty)*])*) => {
        $($(
            impl PartialEq<$ty> for JValue {
                fn eq(&self, other: &$ty) -> bool {
                    $eq(self, *other as _)
                }
            }

            impl PartialEq<JValue> for $ty {
                fn eq(&self, other: &JValue) -> bool {
                    $eq(other, *self as _)
                }
            }

            impl<'a> PartialEq<$ty> for &'a JValue {
                fn eq(&self, other: &$ty) -> bool {
                    $eq(*self, *other as _)
                }
            }
        )*)*
    }
}

partialeq_numeric! {
    eq_i64[i8 i16 i32 i64 isize]
    eq_u64[u8 u16 u32 u64 usize]
    eq_f32[f32]
    eq_f64[f64]
    eq_bool[bool]
}
