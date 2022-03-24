use super::Value;

pub trait ToIterable {
    fn to_iterable<'value>(&'value self) -> Option<&(dyn Value + 'value)>;
}
