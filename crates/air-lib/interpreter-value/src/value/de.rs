use crate::value::JValue;
use crate::{JsonString, Map};
use core::fmt;
use serde::de::{self, Deserialize, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde_json::Number;
use std::vec::Vec;

impl<'de> Deserialize<'de> for JValue {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<JValue, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = JValue;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("any valid JSON value")
            }

            #[inline]
            fn visit_bool<E>(self, value: bool) -> Result<JValue, E> {
                Ok(JValue::Bool(value))
            }

            #[inline]
            fn visit_i64<E>(self, value: i64) -> Result<JValue, E> {
                Ok(JValue::Number(value.into()))
            }

            #[inline]
            fn visit_u64<E>(self, value: u64) -> Result<JValue, E> {
                Ok(JValue::Number(value.into()))
            }

            #[inline]
            fn visit_f64<E>(self, value: f64) -> Result<JValue, E> {
                Ok(Number::from_f64(value).map_or(JValue::Null, JValue::Number))
            }

            fn visit_str<E>(self, value: &str) -> Result<JValue, E>
            where
                E: serde::de::Error,
            {
                Ok(JValue::String(value.into()))
            }

            #[inline]
            fn visit_none<E>(self) -> Result<JValue, E> {
                Ok(JValue::Null)
            }

            #[inline]
            fn visit_some<D>(self, deserializer: D) -> Result<JValue, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Deserialize::deserialize(deserializer)
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<JValue, E> {
                Ok(JValue::Null)
            }

            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> Result<JValue, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut vec = Vec::new();

                while let Some(elem) = tri!(visitor.next_element()) {
                    vec.push(elem);
                }

                Ok(JValue::Array(vec.into()))
            }

            fn visit_map<V>(self, mut visitor: V) -> Result<JValue, V::Error>
            where
                V: MapAccess<'de>,
            {
                match tri!(visitor.next_key_seed(KeyClassifier)) {
                    Some(KeyClass::Map(first_key)) => {
                        let mut values = Map::<JsonString, JValue>::new();

                        values.insert(first_key, tri!(visitor.next_value()));
                        while let Some((key, value)) = tri!(visitor.next_entry::<JsonString, _>()) {
                            values.insert(key, value);
                        }

                        Ok(JValue::Object(values.into()))
                    }
                    None => Ok(JValue::Object(Map::new().into())),
                }
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

struct KeyClassifier;

enum KeyClass {
    Map(JsonString),
}

impl<'de> DeserializeSeed<'de> for KeyClassifier {
    type Value = KeyClass;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(self)
    }
}

impl<'de> Visitor<'de> for KeyClassifier {
    type Value = KeyClass;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string key")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(KeyClass::Map(s.into()))
    }
}
