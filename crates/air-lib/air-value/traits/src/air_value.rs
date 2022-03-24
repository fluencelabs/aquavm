use serde_json::Value as JValue;

pub trait Value {
    fn get_by_field_name<'value>(&'value self, field_name: &str) -> Option<&(dyn Value + 'value)>;
    fn get_by_idx<'value>(&'value self, idx: usize) -> Option<&(dyn Value + 'value)>;

    fn as_str(&self) -> Option<&str>;

    fn as_u64(&self) -> Option<u64>;

    fn as_i64(&self) -> Option<i64>;

    fn as_f64(&self) -> Option<f64>;

    fn as_bool(&self) -> Option<bool>;
}

impl Value for JValue {
    fn get_by_field_name<'a>(&'a self, field_name: &str) -> Option<&(dyn Value + 'a)> {
        self.get(field_name).map(|jvalue| {
            let boxed_value: &dyn Value = jvalue;
            boxed_value
        })
    }

    fn get_by_idx<'a>(&'a self, index: usize) -> Option<&(dyn Value + 'a)> {
        self.get(index).map(|jvalue| {
            let boxed_value: &dyn Value = jvalue;
            boxed_value
        })
    }

    fn as_str(&self) -> Option<&str> {
        self.as_str()
    }

    fn as_u64(&self) -> Option<u64> {
        self.as_u64()
    }

    fn as_i64(&self) -> Option<i64> {
        self.as_i64()
    }

    fn as_f64(&self) -> Option<f64> {
        self.as_f64()
    }

    fn as_bool(&self) -> Option<bool> {
        self.as_bool()
    }
}
