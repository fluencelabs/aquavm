use std::collections::BTreeMap;
use std::rc::Rc;

// We only use our own error type; no need for From conversions provided by the
// standard library's try! macro. This reduces lines of LLVM IR by 4%.
macro_rules! tri {
    ($e:expr $(,)?) => {
        match $e {
            core::result::Result::Ok(val) => val,
            core::result::Result::Err(err) => return core::result::Result::Err(err),
        }
    };
}

mod value;

pub use value::JValue;

pub type Map<K, V> = BTreeMap<K, V>;

// it is memory- and CPU-wise more effective than a string
pub type JsonString = Rc<str>;
